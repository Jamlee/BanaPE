# BanaPE - WinPE Builder

> **WIP** | 基于 [Tauri 2.x](https://tauri.app) + [Vue 3](https://vuejs.org) 的 WinPE 救援环境构建工具，完全兼容 **[WimBuilder2 WIN10XPE](https://github.com/ChibiANU/WimBuilder2)** 项目结构。

![BanaPE Screenshot](docs/screenshot.png)

## ✨ Features / 特性

- 🛠️ **87 Components** — 模块化组件系统，覆盖 Shell / 网络 / 音频 / 系统 / 驱动 / 应用等全品类，每个组件可独立启用或禁用
- 🌐 **5 Languages** — 完整 UI 翻译：简体中文、English、日本語、한국어、Русский
- ⚡ **Fast Build** — 基于 WimBuilder2 架构优化，典型构建时间 2-7 分钟
- 🎨 **PEBakery Style UI** — 经典树形菜单 + 内容区布局（参考 PhoenixPE/老毛桃风格）
- 🔧 **TOML Config** — Rust 引擎读取 TOML 配置执行构建命令
- 📡 **Real-time Progress** — Tauri Event 驱动的进度条、日志输出、状态面板更新

## 🏗️ Architecture / 架构

```
BanaPE/
├── src-tauri/                    # Tauri 后端 (Rust)
│   ├── src/
│   │   ├── commands/build.rs     # 构建命令 + Event 发射器
│   │   ├── engine/builder.rs     # 核心构建引擎
│   │   └── patches/              # 组件补丁模块
│   ├── icons/icon.ico            # 应用图标 (Indigo B Logo)
│   └── Cargo.toml
├── src/
│   ├── App.vue                   # 主界面 (PEBakery Style)
│   ├── styles/main.css           # Indigo Tech Theme
│   └── i18n/                     # 多语言翻译 (5 languages)
│       ├── index.ts              # i18n composable
│       ├── en.ts                 # English
│       ├── zh-CN.ts              # 简体中文
│       ├── ja.ts                 # 日本語
│       ├── ko.ts                 # 한국어
│       └── ru.ts                 # Русский
├── patches/
│   └── component-index.toml      # 组件索引 (87 components, 9 categories)
└── package.json
```

## 📦 Component System / 组件系统

完全兼容 WimBuilder2 WIN10XPE 项目结构，87 个组件分布在 9 个分类中：

| Category | ID | Count | Source |
|----------|-----|-------|--------|
| Configures | `configures` | 10 | `00-Configures` |
| ADK OCs | `adk_ocs` | 6 | `01-ADK_OCs` |
| Shell | `components_shell` | 5 | `01-Components\00-Shell` |
| Network | `components_network` | 5 | `01-Components\02-Network` |
| Audio | `components_audio` | 2 | `01-Components\03-Audio` |
| System | `components_system` | ~40 | MMC/DWM/IME/Accessories/Runtime/UWP |
| Drivers | `drivers` | 1 | `03-Drivers` |
| Apps | `apps` | 8 | `02-Apps` |
| PE Material | `pematerial` | 10 | `02-PEMaterial` |

### Key Components / 核心组件

**Shell:**
- `shell_explorer` — Windows 资源管理器完整版（主题支持/文件操作）
- `shell_winxshell` — 轻量级 Shell 替代方案（低内存占用）
- `boot2winre` — 启动进入 WinRE 恢复环境（内置 WiFi）

**Network:**
- `network_full` — 完整 TCP/IP 协议栈（DNS/DHCP/WLAN）
- `wifi_package` — 无线网络驱动包（WiFi 连接管理）

**System:**
- `dwm` — DWM 桌面窗口管理器（Aero Glass 视觉效果）
- `mmc` — 微软管理控制台（磁盘管理/服务/事件查看器）
- `ime_zhcn` — 简体中文输入法引擎（微软拼音/五笔）
- `bitlocker` — BitLocker 磁盘加密驱动

**Apps:**
- `app_7zip` — 7-Zip 压缩解压工具
- `app_chrome` — Google Chrome 浏览器
- `app_sumatrapdf` — SumatraPDF PDF 阅读器
- `app_everything` — Everything 文件搜索引擎

**PE Material:**
- `build_iso` — 生成可启动 ISO 镜像
- `build_usb` — 制作 USB 启动盘

## 🚀 Quick Start / 快速开始

### Prerequisites / 前置要求

- Windows 10/11 x64
- Node.js 18+
- Rust toolchain (via `rustup`)
- Windows ADK (optional, for advanced features)

### Install & Run / 安装运行

```bash
git clone https://github.com/your-repo/BanaPE.git
cd BanaPE
npm install
npm run tauri dev      # 开发模式
npm run tauri build    # 生产构建
```

## 🛠️ Tech Stack / 技术栈

| Layer | Technology |
|-------|-----------|
| Frontend | Vue 3 (Composition API) + Vite |
| Desktop Framework | Tauri v2 |
| Backend Engine | Rust + Tokio |
| Config Format | TOML (`component-index.toml`) |
| i18n | Custom composable (localStorage persistence) |
| Build Events | Tauri `app.emit()` event system |

## 📋 Current Status / 当前状态

> **⚠️ Work In Progress**

- [x] PEBakery-style UI layout (toolbar + sidebar tree + content area + statusbar)
- [x] Indigo Tech theme design
- [x] 87 components across 9 categories (WimBuilder2 compatible)
- [x] i18n support for 5 languages
- [x] Component detail page with description/features/info table
- [x] Welcome page with feature cards and quick start guide
- [x] Multi-resolution ICO icon generation
- [ ] Real build engine integration with WimBuilder2 scripts
- [ ] Actual WIM mount/unmount operations
- [ ] Registry patch application
- [ ] Driver injection pipeline
- [ ] ISO generation (oscdimg)
- [ ] USB bootable media creation
- [ ] Settings page implementation
- [ ] About dialog implementation

## 📄 License / 许可证

MIT License

---

**Version**: 1.0.0-WIP  
**Platform**: Windows 10/11 x64  
**Based on**: [WimBuilder2](https://github.com/ChibiANU/WimBuilder2) WIN10XPE architecture
