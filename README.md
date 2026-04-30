# BanaPE - Modern WinPE Builder

基于 **Tauri 2.x** + **Vue 3** 的现代化 Windows PE 构建工具，支持 Windows 10/11 x64。

## ✨ 特性

- 🚀 **轻量快速** - 基于 Tauri，应用体积小，启动快
- 🎨 **现代界面** - 深色/浅色主题，响应式设计
- 🔧 **TOML 补丁系统** - 类似 PEBakery，易于手写和维护
- 📦 **完整功能** - Shell、网络、驱动、MMC 等组件支持
- 💾 **一键构建** - 从 ISO 到启动镜像的全自动化流程

## 📦 快速开始

### 环境要求

- Windows 10/11 x64
- Node.js 18+
- Rust 1.70+（首次运行自动安装）

### 安装和运行

```bash
# 1. 克隆项目
git clone <repository-url>
cd BanaPE

# 2. 安装依赖
npm install

# 3. 开发模式
npm run tauri:dev

# 4. 生产构建
npm run tauri:build
```

### 开发命令

```bash
npm run dev              # 前端开发服务器
npm run build            # 构建前端
npm run tauri:dev        # Tauri 开发模式（前端+后端）
npm run tauri:build      # 生产构建
npm run tauri:build:debug  # 调试构建
```

## 📁 项目结构

```
BanaPE/
├── src-tauri/              # Tauri 后端 (Rust)
│   ├── src/
│   │   ├── commands/      # Tauri 命令处理器
│   │   ├── engine/        # 核心引擎 (WIM、注册表、构建器)
│   │   └── patches/       # 补丁加载器
│   ├── capabilities/      # 权限配置
│   ├── Cargo.toml         # Rust 依赖
│   └── tauri.conf.json    # Tauri 配置
├── src/                   # Vue 3 前端
│   ├── views/             # 页面组件
│   └── styles/            # 样式
├── patches/               # TOML 补丁配置 ⭐
│   ├── core/              # 核心配置
│   ├── components/        # 功能组件
│   ├── network/           # 网络支持
│   └── drivers/           # 驱动支持
├── package.json
└── README.md
```

## 🔧 补丁系统

BanaPE 使用 **TOML 格式**的补丁配置，类似 PEBakery 的 `.script` 文件。

### 目录结构

```
patches/
├── core/
│   ├── build-config.toml  # 构建配置
│   └── account.toml       # 账户设置
├── components/
│   ├── shell.toml         # Shell (Explorer/WinXShell)
│   ├── mmc.toml           # MMC 管理控制台
│   └── notepad.toml       # 记事本
├── network/
│   └── network.toml       # TCP/IP、DNS、DHCP
└── drivers/
    └── drivers.toml       # PnP、驱动存储
```

### 配置格式示例

```toml
[patch]
id = "shell"
name = "Shell 配置"
category = "components"
order = 10
dependencies = ["build-config"]

# 文件操作
[[files.copy]]
source = "Windows\\explorer.exe"
dest = "Windows\\explorer.exe"

[files]
create_dirs = ["Windows\\System32\\config\\systemprofile\\Desktop"]

# 注册表操作
[registry]
load = [
    { hive = "SOFTWARE", file = "Windows\\System32\\config\\SOFTWARE" }
]

[[registry.add]]
hive = "SOFTWARE"
key = "Microsoft\\Windows NT\\CurrentVersion\\Winlogon"
value = "Shell"
type = "REG_SZ"
data = "explorer.exe"

unload = ["SOFTWARE"]

# 命令执行
[[commands]]
tool = "cmd"
args = ["/c", "echo done"]
admin = true
```

### 添加新补丁

1. 在 `patches/` 下创建 `.toml` 文件
2. 定义补丁元信息、文件操作、注册表修改、命令
3. 前端会自动加载，无需重新编译

详细说明：[patches/README.md](patches/README.md)

## 🏗️ 构建流程

1. **选择源文件** - Windows ISO 或挂载驱动器
2. **配置组件** - 选择需要的补丁
3. **执行构建**：
   - 复制 boot.wim
   - 挂载 WIM 映像
   - 加载注册表
   - 应用补丁（文件、注册表、命令）
   - 卸载注册表
   - 提交 WIM
   - 创建启动 ISO

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Vue 3 + Vite |
| 桌面框架 | Tauri 2.x |
| 后端 | Rust + Tokio |
| 配置格式 | TOML |

## 📝 注意事项

- 补丁配置文件修改后需重启应用生效
- 确保 `dependencies` 中引用的补丁存在
- 加载的注册表 hive 必须在 `unload` 中卸载
- `order` 值决定补丁执行顺序

## 📄 许可证

MIT License

---

**版本**: 1.0.0  
**平台**: Windows 10/11 x64
