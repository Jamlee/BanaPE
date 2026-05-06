# BanaPE - WinPE Builder

<p align="center">
  <strong>⚠️ Work In Progress (WIP)</strong><br>
  基于 <a href="https://tauri.app">Tauri 2.x</a> + <a href="https://vuejs.org">Vue 3</a> 的 WinPE 救援与恢复环境构建工具<br>
  <strong>具备与 <a href="https://github.com/ChibiANU/WimBuilder2">WimBuilder2 WIN10XPE</a> 同等的构建能力</strong>
</p>

<p align="center">
  <img src="docs/screenshot.png" alt="BanaPE Screenshot" width="700">
</p>

---

## 🌐 Languages / 语言

| Language | 文件 |
|----------|------|
| **简体中文** | [README.zh-CN.md](./README.zh-CN.md) |
| **English** | [README.en.md](./README.en.md) |

---

## ✨ Features / 特性

- 🛠️ **87 Components** — 模块化组件系统，覆盖 Shell / 网络 / 音频 / 系统 / 驱动 / 应用等全品类
- 🌐 **5 Languages** — 简体中文、English、日本語、한국어、Русский
- ⚡ **Fast Build** — 典型构建时间 2-7 分钟
- 🎨 **PEBakery Style UI** — 经典树形菜单布局（参考 PhoenixPE/老毛桃）
- 🔧 **TOML Config** — Rust 引擎读取配置执行构建命令
- 📡 **Real-time Progress** — Tauri Event 驱动的进度条、日志、状态更新

## 📦 Component System Overview

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

## 🚀 Quick Start

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
| Config Format | TOML |
| i18n | 5 languages (localStorage) |

## 📋 Status

> **⚠️ WIP** — UI, i18n, component system complete. Build engine integration in progress.

See [README.zh-CN.md](./README.zh-CN.md) or [README.en.md](./README.en.md) for full documentation.

## 📄 License

MIT

---

**Version**: 1.0.0-WIP · **Platform**: Windows 10/11 x64 · **Capability Equivalent To**: [WimBuilder2](https://github.com/ChibiANU/WimBuilder2)
