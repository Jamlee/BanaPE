# BanaPE - WinPE 构建器

> **⚠️ 工作进行中 (WIP)** | 基于 [Tauri 2.x](https://tauri.app) + [Vue 3](https://vuejs.org) 的 WinPE 救援与恢复环境构建工具，**具备与 [WimBuilder2 WIN10XPE](https://github.com/ChibiANU/WimBuilder2) 同等的构建能力**。

![BanaPE 主界面](docs/screenshot.png)

---

## ✨ 特性

- 🛠️ **87 个组件** — 模块化组件系统，覆盖 Shell / 网络 / 音频 / 系统 / 驱动 / 应用等全品类，每个组件可独立启用或禁用
- 🌐 **5 种语言** — 完整 UI 翻译：简体中文、English、日本語、한국어、Русский
- ⚡ **快速构建** — 基于与 WimBuilder2 同等架构优化，典型构建时间 2-7 分钟
- 🎨 **PEBakery 风格界面** — 经典树形菜单 + 内容区布局（参考 PhoenixPE/老毛桃风格）
- 🔧 **TOML 配置** — Rust 引擎读取 TOML 配置执行构建命令
- 📡 **实时进度** — Tauri Event 驱动的进度条、日志输出、状态面板更新

## 🏗️ 项目结构

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
│   ├── App.vue                   # 主界面 (PEBakery 风格)
│   ├── styles/main.css           # Indigo 科技主题
│   └── i18n/                     # 多语言翻译 (5 种语言)
│       ├── index.ts              # i18n 组合式函数
│       ├── en.ts                 # English
│       ├── zh-CN.ts              # 简体中文
│       ├── ja.ts                 # 日本語
│       ├── ko.ts                 # 한국어
│       └── ru.ts                 # Русский
├── patches/
│   └── component-index.toml      # 组件索引 (87 个组件, 9 个分类)
└── package.json
```

## 📦 组件系统

**具备与 WimBuilder2 WIN10XPE 项目同等的能力**，87 个组件分布在 9 个分类中：

| 分类 | ID | 数量 | 对应来源 |
|------|-----|------|----------|
| 配置项 | `configures` | 10 | `00-Configures` |
| ADK 组件 | `adk_ocs` | 6 | `01-ADK_OCs` |
| Shell | `components_shell` | 5 | `01-Components\00-Shell` |
| 网络 | `components_network` | 5 | `01-Components\02-Network` |
| 音频 | `components_audio` | 2 | `01-Components\03-Audio` |
| 系统组件 | `components_system` | ~40 | MMC/DWM/IME/辅助工具/Runtime/UWP |
| 驱动 | `drivers` | 1 | `03-Drivers` |
| 应用 | `apps` | 8 | `02-Apps` |
| PE 材料 | `pematerial` | 10 | `02-PEMaterial` |

### 核心组件

**Shell 外壳：**
- `shell_explorer` — Windows 资源管理器完整版（主题支持/文件操作）
- `shell_winxshell` — 轻量级 Shell 替代方案（低内存占用）
- `boot2winre` — 启动进入 WinRE 恢复环境（内置 WiFi）

**网络支持：**
- `network_full` — 完整 TCP/IP 协议栈（DNS/DHCP/WLAN）
- `wifi_package` — 无线网络驱动包（WiFi 连接管理）
- `rndis` — USB RNDIS 网络（手机 USB 共享网络）
- `pppoe` — PPPoE 宽带拨号连接

**系统组件：**
- `dwm` — DWM 桌面窗口管理器（Aero Glass 视觉效果）
- `mmc` — 微软管理控制台（磁盘管理/服务/事件查看器）
- `ime_zhcn` / `ime_ja` — 中日文输入法引擎
- `bitlocker` — BitLocker 磁盘加密驱动
- `remote_desktop` — 远程桌面 (RDP) 客户端
- `search` — Windows Search 文件索引搜索

**应用程序：**
- `app_7zip` — 7-Zip 压缩解压工具
- `app_chrome` — Google Chrome 浏览器
- `app_sumatrapdf` — SumatraPDF PDF 阅读器
- `app_everything` — Everything 文件搜索引擎
- `app_winntsetup` — WinNTSetup 系统安装部署工具

**PE 构建：**
- `build_iso` — 生成可启动 ISO 镜像（oscdimg）
- `build_usb` — 制作 USB 启动盘

## 🚀 快速开始

### 环境要求

- Windows 10/11 x64
- Node.js 18+
- Rust 工具链 (`rustup`)
- Windows ADK（可选，高级功能需要）

### 安装运行

```bash
git clone https://github.com/Jamlee/BanaPE.git
cd BanaPE
npm install
npm run tauri dev      # 开发模式
npm run tauri build    # 生产构建
```

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Vue 3 (Composition API) + Vite |
| 桌面框架 | Tauri v2 |
| 后端引擎 | Rust + Tokio |
| 配置格式 | TOML (`component-index.toml`) |
| 国际化 | 自定义 composable (localStorage 持久化) |
| 构建事件 | Tauri `app.emit()` 事件系统 |

## 📋 当前进度

> **⚠️ 工作进行中**

- [x] PEBakery 风格 UI 布局（工具栏 + 侧边栏树形菜单 + 内容区 + 状态栏）
- [x] Indigo 科技主题设计
- [x] 87 个组件 × 9 个分类（具备 WimBuilder2 同等能力）
- [x] 5 语言国际化支持
- [x] 组件详情页（描述/功能列表/技术信息表）
- [x] Welcome 页面（特性卡片 + 快速入门指南）
- [x] 多分辨率 ICO 图标生成
- [ ] 真实构建引擎对接 WimBuilder2 脚本
- [ ] WIM 映像挂载/卸载操作
- [ ] 注册表补丁应用
- [ ] 驱动注入流水线
- [ ] ISO 镜像生成 (oscdimg)
- [ ] USB 启动介质制作
- [ ] 设置页面实现
- [ ] 关于对话框实现

## 📄 许可证

MIT License

---

**版本**: 1.0.0-WIP  
**平台**: Windows 10/11 x64  
**能力对标**: [WimBuilder2](https://github.com/ChibiANU/WimBuilder2) WIN10XPE
