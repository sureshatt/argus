#!/bin/bash
# Build script for macOS distribution of Argus.
# Produces:
#   - Argus.app + Argus.dmg  (via tauri build)
#   - ArgusExtras.pkg        (ChmodBPF daemon installer, via pkgbuild)
set -e

REPO="$(cd "$(dirname "$0")/.." && pwd)"
TAURI_DIR="$REPO/src-tauri"
RESOURCES_DIR="$TAURI_DIR/resources"
BUILD_DIR="$TAURI_DIR/target/universal-apple-darwin/release/bundle"
PKG_PAYLOAD="$REPO/packaging/macos/pkg-payload"

echo "==> Ensuring Rust cross-compilation targets are installed..."
rustup target add aarch64-apple-darwin x86_64-apple-darwin

# tauri-build validates resource paths at compile time — create placeholders so
# the validation passes before the real binaries are built.
echo "==> Creating resource placeholders for tauri-build validation..."
mkdir -p "$RESOURCES_DIR"
touch "$RESOURCES_DIR/chmodbpf"
touch "$RESOURCES_DIR/com.argus.chmodbpf.plist"

echo "==> Building chmodbpf helper (arm64)..."
rustc \
    --edition 2021 \
    --target aarch64-apple-darwin \
    -o "$TAURI_DIR/target/chmodbpf-arm64" \
    "$TAURI_DIR/src/bin/chmodbpf.rs"

echo "==> Building chmodbpf helper (x86_64)..."
rustc \
    --edition 2021 \
    --target x86_64-apple-darwin \
    -o "$TAURI_DIR/target/chmodbpf-x86_64" \
    "$TAURI_DIR/src/bin/chmodbpf.rs"

echo "==> Creating universal binary with lipo..."
mkdir -p "$RESOURCES_DIR"
lipo -create \
    -output "$RESOURCES_DIR/chmodbpf" \
    "$TAURI_DIR/target/chmodbpf-arm64" \
    "$TAURI_DIR/target/chmodbpf-x86_64"

echo "==> Copying LaunchDaemon plist to resources..."
cp "$REPO/packaging/macos/com.argus.chmodbpf.plist" "$RESOURCES_DIR/com.argus.chmodbpf.plist"

echo "==> Building Tauri app (universal, DMG + app)..."
cd "$REPO"
pnpm tauri build --target universal-apple-darwin

echo "==> Assembling PKG payload structure..."
rm -rf "$PKG_PAYLOAD"
mkdir -p "$PKG_PAYLOAD/Library/PrivilegedHelperTools"
mkdir -p "$PKG_PAYLOAD/Library/LaunchDaemons"
cp "$RESOURCES_DIR/chmodbpf"                    "$PKG_PAYLOAD/Library/PrivilegedHelperTools/com.argus.chmodbpf"
cp "$RESOURCES_DIR/com.argus.chmodbpf.plist"    "$PKG_PAYLOAD/Library/LaunchDaemons/com.argus.chmodbpf.plist"

echo "==> Building ArgusExtras.pkg with pkgbuild..."
chmod +x "$REPO/packaging/macos/scripts/postinstall"
pkgbuild \
    --identifier "com.argus.chmodbpf" \
    --version "1.0" \
    --scripts "$REPO/packaging/macos/scripts" \
    --root "$PKG_PAYLOAD" \
    "$BUILD_DIR/dmg/ArgusExtras.pkg"

echo ""
echo "==> Done."
echo "    App:           $BUILD_DIR/macos/argus.app"
echo "    DMG:           $BUILD_DIR/dmg/"
echo "    Extras PKG:    $BUILD_DIR/dmg/ArgusExtras.pkg"
