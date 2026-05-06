# BanaPE - WinPE Builder

> **⚠️ Work In Progress (WIP)** | A WinPE rescue and recovery environment builder powered by [Tauri 2.x](https://tauri.app) + [Vue 3](https://vuejs.org), **with equivalent build capabilities as [WimBuilder2 WIN10XPE](https://github.com/ChibiANU/WimBuilder2)**.

![BanaPE Main UI](docs/screenshot.png)

---

## ✨ Features

- 🛠️ **87 Components** — Modular component system covering Shell / Network / Audio / System / Drivers / Apps and more. Each component can be independently enabled or disabled.
- 🌐 **5 Languages** — Full UI translation: Simplified Chinese, English, 日本語, 한국어, Русский
- ⚡ **Fast Build** — Optimized pipeline with architecture equivalent to WimBuilder2. Typical build time 2-7 minutes.
- 🎨 **PEBakery-Style UI** — Classic tree menu + content area layout (inspired by PhoenixPE / USMBoot)
- 🔧 **TOML Configuration** — Rust engine reads TOML config to execute build commands
- 📡 **Real-time Progress** — Tauri Event-driven progress bar, log output, and status panel updates

## 🏗️ Architecture

```
BanaPE/
├── src-tauri/                    # Tauri Backend (Rust)
│   ├── src/
│   │   ├── commands/build.rs     # Build command + Event emitter
│   │   ├── engine/builder.rs     # Core build engine
│   │   └── patches/              # Component patch modules
│   ├── icons/icon.ico            # App icon (Indigo B Logo)
│   └── Cargo.toml
├── src/
│   ├── App.vue                   # Main UI (PEBakery style)
│   ├── styles/main.css           # Indigo Tech Theme
│   └── i18n/                     # Multi-language translations (5 languages)
│       ├── index.ts              # i18n composable
│       ├── en.ts                 # English
│       ├── zh-CN.ts              # 简体中文
│       ├── ja.ts                 # 日本語
│       ├── ko.ts                 # 한국어
│       └── ru.ts                 # Русский
├── patches/
│   └── component-index.toml      # Component index (87 components, 9 categories)
└── package.json
```

## 📦 Component System

**With equivalent capabilities to the WimBuilder2 WIN10XPE project**, 87 components distributed across 9 categories:

| Category | ID | Count | Source Mapping |
|----------|-----|-------|----------------|
| Configures | `configures` | 10 | `00-Configures` |
| ADK OCs | `adk_ocs` | 6 | `01-ADK_OCs` |
| Shell | `components_shell` | 5 | `01-Components\00-Shell` |
| Network | `components_network` | 5 | `01-Components\02-Network` |
| Audio | `components_audio` | 2 | `01-Components\03-Audio` |
| System | `components_system` | ~40 | MMC/DWM/IME/Accessories/Runtime/UWP |
| Drivers | `drivers` | 1 | `03-Drivers` |
| Apps | `apps` | 8 | `02-Apps` |
| PE Material | `pematerial` | 10 | `02-PEMaterial` |

### Key Components

**Shell:**
- `shell_explorer` — Full Windows Explorer (theme support / file operations)
- `shell_winxshell` — Lightweight Shell alternative (low memory footprint)
- `boot2winre` — Boot into WinRE recovery environment (built-in WiFi)

**Network:**
- `network_full` — Complete TCP/IP protocol stack (DNS/DHCP/WLAN)
- `wifi_package` — Wireless network driver package (WiFi connection manager)
- `rndis` — USB RNDIS networking (phone USB tethering)
- `pppoe` — PPPoE broadband dial-up connection

**System:**
- `dwm` — Desktop Window Manager (Aero Glass visual effects)
- `mmc` — Microsoft Management Console (Disk Mgmt / Services / Event Viewer)
- `ime_zhcn` / `ime_ja` — Chinese/Japanese IME engines
- `bitlocker` — BitLocker disk encryption driver
- `remote_desktop` — Remote Desktop (RDP) client
- `search` — Windows Search file indexing engine

**Applications:**
- `app_7zip` — 7-Zip compression tool
- `app_chrome` — Google Chrome browser
- `app_sumatrapdf` — SumatraPDF PDF reader
- `app_everything` — Everything file search engine
- `app_winntsetup` — WinNTSetup system deployment tool

**PE Build:**
- `build_iso` — Generate bootable ISO image (oscdimg)
- `build_usb` — Create USB bootable media

## 🚀 Quick Start

### Prerequisites

- Windows 10/11 x64
- Node.js 18+
- Rust toolchain (`rustup`)
- Windows ADK (optional, for advanced features)

### Install & Run

```bash
git clone https://github.com/Jamlee/BanaPE.git
cd BanaPE
npm install
npm run tauri dev      # Dev mode
npm run tauri build    # Production build
```

## 🛠️ Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | Vue 3 (Composition API) + Vite |
| Desktop Framework | Tauri v2 |
| Backend Engine | Rust + Tokio |
| Config Format | TOML (`component-index.toml`) |
| i18n | Custom composable (localStorage persistence) |
| Build Events | Tauri `app.emit()` event system |

## 📋 Current Status

> **⚠️ Work In Progress**

- [x] PEBakery-style UI layout (toolbar + sidebar tree + content area + statusbar)
- [x] Indigo Tech theme design
- [x] 87 components across 9 categories (equivalent capability to WimBuilder2)
- [x] i18n support for 5 languages
- [x] Component detail page with description/features/info table
- [x] Welcome page with feature cards and quick start guide
- [x] Multi-resolution ICO icon generation
- [ ] Real build engine integration with WimBuilder2-equivalent scripts
- [ ] Actual WIM mount/unmount operations
- [ ] Registry patch application
- [ ] Driver injection pipeline
- [ ] ISO generation (oscdimg)
- [ ] USB bootable media creation
- [ ] Settings page implementation
- [ ] About dialog implementation

## 📄 License

MIT License

---

**Version**: 1.0.0-WIP  
**Platform**: Windows 10/11 x64  
**Capability Equivalent To**: [WimBuilder2](https://github.com/ChibiANU/WimBuilder2) WIN10XPE
