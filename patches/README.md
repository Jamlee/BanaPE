# BanaPE 补丁配置

补丁配置文件使用 **TOML** 格式，易于手写和维护。

## 目录结构

```
patches/
├── core/                   # 核心配置
│   ├── build-config.toml   # 构建配置
│   └── account.toml        # 账户设置
├── components/             # 功能组件
│   ├── shell.toml          # Shell 配置
│   ├── mmc.toml            # MMC 管理控制台
│   └── notepad.toml        # 记事本
├── network/                # 网络支持
│   └── network.toml
├── drivers/                # 驱动支持
│   └── drivers.toml
└── runtime-kits/           # 运行时包
```

## 配置文件格式

### 基本结构

```toml
# 补丁元信息
[patch]
id = "补丁唯一标识"
name = "补丁名称"
category = "分类"
description = "补丁描述"
order = 10  # 排序
dependencies = ["依赖的补丁ID"]

# 文件操作
[files]
create_dirs = ["目录1", "目录2"]

[[files.copy]]
source = "源路径"
dest = "目标路径"
optional = false  # 是否可选

# 注册表操作
[registry]
load = [
    { hive = "注册表 hive", file = "注册表文件路径" }
]

[[registry.add]]
hive = "注册表 hive"
key = "注册表键"
value = "值名"
type = "REG_SZ"  # REG_SZ, REG_DWORD, REG_BINARY, REG_EXPAND_SZ
data = "数据"

unload = ["hive1", "hive2"]

# 命令执行
[[commands]]
tool = "工具名"
args = ["参数1", "参数2"]
admin = true  # 是否需要管理员权限
```

## 字段说明

### [patch] 补丁信息
- `id`: 补丁唯一标识符
- `name`: 补丁显示名称
- `category`: 分类 (core, components, network, drivers, runtime-kits)
- `description`: 补丁描述
- `order`: 执行顺序（数字越小越先执行）
- `dependencies`: 依赖的其他补丁 ID 列表

### [files] 文件操作
- `create_dirs`: 需要创建的目录列表
- `[[files.copy]]`: 复制文件配置
  - `source`: 源文件路径（相对于 Windows 源）
  - `dest`: 目标路径（相对于挂载的 WIM）
  - `optional`: 是否可选（true 表示源文件不存在时不报错）
  - `recursive`: 是否递归复制目录

### [registry] 注册表操作
- `load`: 加载注册表 hive 文件
  - `hive`: hive 名称（如 SYSTEM, SOFTWARE, DEFAULT, SAM）
  - `file`: hive 文件路径
- `[[registry.add]]`: 添加注册表项
  - `hive`: 目标 hive
  - `key`: 注册表键路径
  - `value`: 值名称（空字符串表示默认值）
  - `type`: 值类型（REG_SZ, REG_DWORD, REG_BINARY, REG_EXPAND_SZ）
  - `data`: 值数据
- `unload`: 需要卸载的 hive 列表

### [[commands]] 命令执行
- `tool`: 要执行的工具（dism, cmd, reg 等）
- `args`: 参数列表
- `admin`: 是否需要管理员权限

## 示例

### 添加一个新组件

创建 `patches/components/myapp.toml`:

```toml
[patch]
id = "myapp"
name = "我的应用"
category = "components"
description = "添加我的应用到 WinPE"
order = 50
dependencies = ["build-config", "shell"]

[[files.copy]]
source = "Tools\\MyApp.exe"
dest = "Windows\\System32\\MyApp.exe"

[files]
create_dirs = ["Windows\\System32\\MyApp"]

[registry]
load = [
    { hive = "SOFTWARE", file = "Windows\\System32\\config\\SOFTWARE" }
]

[[registry.add]]
hive = "SOFTWARE"
key = "Microsoft\\Windows\\CurrentVersion\\Run"
value = "MyApp"
type = "REG_SZ"
data = "X:\\Windows\\System32\\MyApp.exe"

unload = ["SOFTWARE"]
```

## 注意事项

1. **路径分隔符**: 使用双反斜杠 `\\` 或正斜杠 `/`
2. **依赖关系**: 确保 dependencies 中引用的补丁存在
3. **注册表**: 加载的 hive 必须在 unload 中卸载
4. **顺序**: order 值决定补丁执行顺序
5. **注释**: 使用 `#` 添加注释

## 参考

- PEBakery 脚本格式: https://github.com/pebakery/pebakery
- TOML 规范: https://toml.io/en/
