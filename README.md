# CompactVdisk 💽⚡

**CompactVdisk** is a cross-platform desktop GUI utility built with **Tauri v2 + Vue 3 + Tailwind CSS** (with Rust native system backend commands) to inspect, compress, and reclaim storage from virtual disk files—specifically WSL2 (`ext4.vhdx`), VMware (`.vmdk`), and VirtualBox (`.vdi`).

---

## Features

- 🔍 **Auto-Discovery Dashboard (WSL & Docker Disks):**
  - Queries `wsl.exe -l -v` and scans standard package paths (`%LOCALAPPDATA%\Docker\wsl\data\ext4.vhdx` and `%LOCALAPPDATA%\Packages\...`).
  - Displays distros with status, file format, path, and size.

- 📁 **Custom Disk File Selector:**
  - Drag-and-drop zone and native file picker supporting `.vhdx`, `.vmdk`, and `.vdi` files.
  - Instant file inspection and format validation.

- 🚀 **Disk Compaction Engine:**
  - **WSL/VHDX:** Safely runs `wsl.exe --shutdown` followed by automated `diskpart` script execution (`select vdisk file="..."`, `compact vdisk`, `detach vdisk`).
  - **VMDK / VDI:** Checks for presence of CLI tools (`vmware-vdiskmanager`, `qemu-img`, `vboxmanage`) in `PATH` and executes shrinking commands.
  - **Non-blocking Execution:** Real-time log output streamed asynchronously from Rust to frontend via Tauri events (`compact-log`).

- 📊 **Reclaimed Space Summary:**
  - Detailed modal comparing disk size before and after compaction (Initial Size, New Size, Reclaimed Space, and Elapsed Time).

- 🛡️ **Safety & Administrator Checks:**
  - Auto-checks elevated administrator permissions and tool availability (`diskpart`, `qemu-img`, `vboxmanage`).

---

## Architecture Overview

```
CompactVdisk/
├── src/                          # Vue 3 Frontend
│   ├── components/               # UI components (AdminBanner, AutoDiscovery, CustomSelector, TerminalLog, ResultsModal)
│   ├── stores/                   # State management (diskStore.ts)
│   ├── types.ts                  # Shared TypeScript interfaces
│   ├── App.vue                   # Main layout
│   └── main.ts                   # App entrypoint
├── src-tauri/                    # Tauri v2 Rust Backend
│   ├── src/
│   │   ├── admin.rs              # Privilege & CLI tool availability checks
│   │   ├── compactor.rs          # Compaction engine & script generation
│   │   ├── disk_inspector.rs     # Disk format detection & size formatting
│   │   ├── models.rs             # Rust structs and serde models
│   │   ├── wsl_discovery.rs      # WSL list parser & path scanning
│   │   ├── tests.rs              # Rust unit tests
│   │   ├── lib.rs                # Module export definition
│   │   └── main.rs               # Tauri app builder & IPC invocation handlers
│   ├── tauri.conf.json           # Tauri configuration
│   └── Cargo.toml                # Rust dependencies
└── tests/                        # Vitest frontend unit tests
```

### Tauri IPC Bridge Commands

| Command Handler | Description |
|---|---|
| `check_admin` | Verifies administrative / root privileges required for `diskpart` |
| `check_tools` | Checks if `qemu-img`, `vboxmanage`, `vmware-vdiskmanager`, `wsl`, `diskpart` exist in `PATH` |
| `discover_disks` | Auto-detects installed WSL2 distributions and Docker WSL storage files |
| `inspect_disk` | Reads file size, metadata, and format for custom selected disk files |
| `compact_disk` | Asynchronously executes compaction CLI commands and streams live logs |

---

## Prerequisites

- **Node.js** (v18+ recommended) & **pnpm**
- **Rust toolchain** (`rustc` & `cargo` 1.75+)
- Platform Tools (depending on disk formats used):
  - **Windows / WSL:** `wsl.exe` and `diskpart.exe` (included in Windows)
  - **VMware / VMDK:** `vmware-vdiskmanager` or `qemu-img` in `PATH`
  - **VirtualBox / VDI:** `VBoxManage` in `PATH`

---

## Development Workflow

### Installation

```bash
pnpm install
```

### Running Dev Mode

```bash
pnpm tauri dev
```

### Running Tests

```bash
# Frontend Unit Tests
pnpm test

# Production Web Build Check
pnpm build
```

### Production Application Build

```bash
pnpm tauri build
```

---

## License

MIT
