#!/bin/bash
# Build script for macOS PKG distribution of Argus.
# Produces a universal (arm64 + x86_64) ChmodBPF helper binary and runs tauri build.
set -e

REPO="$(cd "$(dirname "$0")/.." && pwd)"
TAURI_DIR="$REPO/src-tauri"
RESOURCES_DIR="$TAURI_DIR/resources"

echo "==> Ensuring Rust cross-compilation targets are installed..."
rustup target add aarch64-apple-darwin x86_64-apple-darwin

echo "==> Building chmodbpf helper (arm64)..."
cargo build \
    --manifest-path "$TAURI_DIR/Cargo.toml" \
    --bin chmodbpf \
    --release \
    --target aarch64-apple-darwin

echo "==> Building chmodbpf helper (x86_64)..."
cargo build \
    --manifest-path "$TAURI_DIR/Cargo.toml" \
    --bin chmodbpf \
    --release \
    --target x86_64-apple-darwin

echo "==> Creating universal binary with lipo..."
mkdir -p "$RESOURCES_DIR"
lipo -create \
    -output "$RESOURCES_DIR/chmodbpf" \
    "$TAURI_DIR/target/aarch64-apple-darwin/release/chmodbpf" \
    "$TAURI_DIR/target/x86_64-apple-darwin/release/chmodbpf"

echo "==> Copying LaunchDaemon plist to resources..."
cp "$REPO/packaging/macos/com.argus.chmodbpf.plist" "$RESOURCES_DIR/com.argus.chmodbpf.plist"

echo "==> Building Tauri app (universal, DMG + PKG)..."
cd "$REPO"
pnpm tauri build --target universal-apple-darwin

echo ""
echo "==> Done."
echo "    App:  src-tauri/target/universal-apple-darwin/release/bundle/macos/argus.app"
echo "    DMG:  src-tauri/target/universal-apple-darwin/release/bundle/dmg/"
echo "    PKG:  src-tauri/target/universal-apple-darwin/release/bundle/pkg/"
