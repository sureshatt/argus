# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Argus** is a real-time network analysis desktop application built with Tauri 2 (Rust backend + React frontend). It captures live network packets, parses them through OSI layers, and visualizes traffic with charts, graphs, and tables.

## Commands

```bash
# Development (starts both Vite dev server and Rust/Tauri)
pnpm tauri dev

# Production build
pnpm tauri build

# macOS distribution build (universal binary, DMG + PKG)
bash scripts/build-macos-pkg.sh

# Frontend only (Vite dev server on localhost:1420)
pnpm dev

# Type check
pnpm build   # runs tsc && vite build
```

> The project uses **pnpm** as the package manager. Do not use npm or yarn.

## Architecture

### Frontend (`src/`)

React 18 + TypeScript SPA served by Vite. The app is a single dashboard (`App.tsx`) composed of feature **Cards** — each card is a self-contained component that fetches its own data via Tauri IPC and renders a specific visualization.

**Data flow:**
1. User selects a network interface → stored in `useNetStore` (Zustand)
2. Selecting an interface invokes `Network.getNetworkLogs(interface)` (Tauri command)
3. The Rust backend starts capturing packets and emits events to the frontend
4. Cards subscribe to events or poll via Tauri commands for updated stats

**Key frontend paths:**
- `src/stores/net.store.ts` — global state (selected interface, selected packet, auto-view toggle)
- `src/services/network.ts` — typed wrappers around all `@tauri-apps/api/core` `invoke()` calls
- `src/types.ts` — all TypeScript types shared across cards
- `src/errors.ts` — centralized error message strings
- `src/Cards/` — one subdirectory per dashboard tile
- `src/components/` — reusable `Card`, `CardHeader`, `CardTitle`, `CardBody`, `Alert`

### Backend (`src-tauri/src/`)

Rust with Tokio async runtime. `lib.rs` registers all Tauri commands and initializes `AppState`.

**Packet capture pipeline:**
```
Interface selection
  → network_dumper.rs (pnet datalink capture, per-interface thread)
    → layers/datalink.rs → layers/network.rs → layers/transport.rs → layers/application.rs
      → parsers/ (ethernet, ipv4, ipv6, tcp, dns, arp, icmp, icmpv6)
        → storage/logger.rs → storage/log_entry.rs
          → storage/log_analytics.rs (computes stats)
            → Tauri events emitted to frontend
```

**AppState** (wrapped in `Arc<RwLock<T>>` for thread safety):
- Selected interface name
- Packet counter
- Basic logs: `VecDeque` (1000 packet limit)
- Detailed logs: `LRU` cache
- GeoIP ranges for country mapping

**Key backend paths:**
- `src-tauri/src/lib.rs` — Tauri command handlers, AppState init
- `src-tauri/src/network/network_dumper.rs` — packet sniffer main logic; emits `bpf_permission_error` or `capture_error` events on failure
- `src-tauri/src/network/ip_utils.rs` — GeoIP lookup utilities
- `src-tauri/src/storage/log_analytics.rs` — statistics computation
- `src-tauri/src/network/layers/` — OSI layer parsers
- `src-tauri/src/network/parsers/` — protocol-specific parsers
- `src-tauri/src/bin/chmodbpf.rs` — privileged helper binary; sets `/dev/bpf*` to `root:admin 0640`, run as root via LaunchDaemon
- `src-tauri/entitlements.plist` — macOS entitlements (no sandbox; required for raw BPF access)

### Visualization Libraries

- **@antv/g6 5.0** — ARP topology graph (`IPAddressesGraph`)
- **amCharts 5** — geo traffic map (`TrafficMap`)
- **Chart.js / react-chartjs-2** — bar/pie charts (`MostFoundNetProtocol`, `MostTrafficIps`)
- **@tanstack/react-table** — packet log table (`LiveNetworkLogs`)
- **react18-json-view** — packet detail viewer (`LogsJsonViewer`)

## Platform Notes

- Requires system-level packet capture permissions (root/admin or `CAP_NET_RAW` on Linux)
- Tauri bundles produce platform-native installers: DMG + PKG (macOS), MSI (Windows), AppImage (Linux)
- Frontend dev server is fixed to `localhost:1420` (required by Tauri)
- Rust changes require restarting `pnpm tauri dev`; React changes hot-reload via Vite HMR

### macOS BPF Permissions

macOS requires `/dev/bpf*` devices to be readable by the `admin` group for packet capture. This is handled via the **ChmodBPF LaunchDaemon pattern** (same as Wireshark):

- `packaging/macos/com.argus.chmodbpf.plist` — LaunchDaemon plist; runs helper as root at boot
- `packaging/macos/scripts/postinstall` — PKG postinstall script; installs daemon + binary with one-time admin prompt
- The app is **not sandboxed** — Mac App Store distribution is incompatible with raw socket access

**Distribution flow:**
```
Argus.dmg
  ├── Argus.app              ← drag to /Applications
  └── Install Extras.pkg     ← one-time admin prompt → installs ChmodBPF daemon
```

The `check_capture_permissions` Tauri command probes `/dev/bpf0` readability at startup. If denied, the `AvailableNetworkInterfaces` card shows a "Permission Required" alert guiding the user to run the installer.
