import en from './en'

export default {
  app: { ...en.app },
  toolbar: {
    build: '构建',
    refresh: '刷新',
    settings: '设置',
    logs: '日志',
    utilities: '工具',
    about: '关于',
  },

  status: {
    status: '状态',
    source: '源文件',
    drive: '盘符',
    type: '类型',
    components: '组件',
    notSet: '未设置',
    idle: '空闲',
    ready: '就绪',
    building: '构建中',
    error: '错误',
    selected: '{count}/{total}',
  },

  sidebar: { title: '组件列表', allExpand: '全部展开' },

  categories: {
    configures: '构建配置',
    adk_ocs: 'ADK 可选组件',
    components_shell: 'Shell 界面',
    components_network: '网络组件',
    components_audio: '音频支持',
    components_system: '系统组件',
    drivers: '硬件驱动',
    apps: '应用程序',
    pematerial: 'PE 额外材料',
  },

  components: {
    // Configures
    cfg_build_catalog: { name: '完整目录', desc: '完整目录文件与 TTS 语音' },
    cfg_loader_network: { name: '加载器网络', desc: '加载时初始化网络连接' },
    cfg_system_audio: { name: '音频驱动补丁', desc: '音频驱动补丁' },
    cfg_system_computername: { name: '计算机名称', desc: '计算机名称设置' },
    cfg_system_peversion: { name: 'PE 版本', desc: 'PE 版本信息配置' },
    cfg_system_productoptions: { name: '产品选项', desc: '产品选项 (安全模式等)' },
    cfg_system_fbwf: { name: 'FBWF 过滤器', desc: '基于文件的写入过滤器' },
    cfg_system_textassoc: { name: '文本关联', desc: '文本文件关联' },
    cfg_account_admin: { name: '管理员账户', desc: '内置管理员账户配置' },
    cfg_customization_theme: { name: '主题自定义', desc: '主题自定义 (透明度等)' },
    
    // ADK OCs
    adk_settings: { name: 'ADK 设置', desc: 'ADK 基础设置' },
    adk_fonts: { name: '字体', desc: '字体支持' },
    adk_network: { name: '网络组件', desc: 'ADK 网络组件' },
    adk_powershell: { name: 'PowerShell', desc: 'PowerShell 环境' },
    adk_setup: { name: '安装程序组件', desc: '安装程序组件' },
    adk_misc: { name: '其他组件', desc: '其他 ADK 组件' },
    
    // Shell
    boot2winre: { name: '启动到 WinRE', desc: '启动到 WinRE 环境 (含 WiFi)' },
    shell_explorer: { name: '资源管理器', desc: 'Windows 资源管理器 (主题/设置)' },
    shell_winxshell: { name: 'WinXShell', desc: '轻量级 Shell 替代方案' },
    shell_startmenu_classic: { name: '经典开始菜单', desc: '经典开始菜单 (Classic Shell / StartIsBack)' },
    shell_settings: { name: 'Shell 设置', desc: '任务栏/资源管理器详细设置' },
    
    // Network
    network_full: { name: '完整网络支持', desc: '完整网络支持 (TCP/IP/DNS/DHCP/WLAN)' },
    wifi_package: { name: 'WinPE 无线包', desc: 'WinPE 无线网络包' },
    rndis: { name: 'RNDIS 驱动', desc: 'USB 网络共享驱动 (手机USB共享)' },
    pppoe: { name: 'PPPoE 支持', desc: 'PPPoE 拨号连接支持' },
    patch_drvinst: { name: 'DrvInst 补丁', desc: '驱动安装程序补丁' },
    
    // Audio
    audio_core: { name: '音频核心', desc: '核心音频支持 (WASAPI/MMCSS)' },
    media_player: { name: '媒体播放器', desc: 'Windows Media Player' },
    
    // System Components
    bitlocker: { name: 'BitLocker', desc: 'BitLocker 磁盘加密支持' },
    cred_dialog: { name: '凭据对话框', desc: '凭据对话框 (UAC提示)' },
    dwm: { name: 'DWM 桌面窗口管理器', desc: '桌面窗口管理器 (Aero效果)' },
    devices_bluetooth: { name: '蓝牙支持', desc: '蓝牙设备支持' },
    devices_camera: { name: '摄像头支持', desc: '摄像头设备支持' },
    devices_dsmsvc: { name: 'DSM 服务', desc: '设备设置管理器服务' },
    devices_printer: { name: '打印机支持', desc: '打印机和扫描仪支持' },
    ime_zhcn: { name: '简体中文输入法', desc: '简体中文输入法 (微软拼音/五笔等)' },
    ime_zhtw: { name: '繁体中文输入法', desc: '繁体中文输入法' },
    ime_ja: { name: '日语输入法', desc: '日语输入法' },
    ime_ko: { name: '韩语输入法', desc: '韩语输入法' },
    ie_browser: { name: 'IE 浏览器', desc: 'IE 浏览器' },
    mmc: { name: 'MMC 控制台', desc: 'Microsoft 管理控制台 (磁盘管理等)' },
    msi: { name: 'MSI 安装程序', desc: 'Windows Installer 服务' },
    mtp_support: { name: 'MTP 支持', desc: 'MTP 设备支持 (Android手机连接)' },
    netfx: { name: '.NET Framework', desc: '.NET Framework 3.5 运行时' },
    remote_desktop: { name: '远程桌面', desc: '远程桌面 (RDP) 支持' },
    search: { name: 'Windows 搜索', desc: 'Windows 索引服务' },
    syswow64_basic: { name: 'SysWOW64 基础', desc: 'WoW64 32位兼容层基础支持' },
    vcruntime: { name: 'VC++ 运行时', desc: 'Visual C++ 运行时库' },
    
    // Accessories
    acc_notepad: { name: '记事本', desc: '文本编辑器' },
    acc_mspaint: { name: '画图', desc: '绘图工具' },
    acc_snippingtool: { name: '截图工具', desc: '屏幕截图工具' },
    acc_winphoto: { name: '照片查看器', desc: 'Windows 照片查看器' },
    acc_accessibility: { name: '辅助功能', desc: '辅助功能支持' },
    acc_taskmgr: { name: '任务管理器', desc: '进程和性能监控' },
    acc_regedit: { name: '注册表编辑器', desc: '注册表管理工具' },
    
    // Runtime Kits
    rt_appcompat: { name: '应用兼容层', desc: '应用兼容性层' },
    rt_arm64_support: { name: 'ARM64 兼容层', desc: 'ARM64 架构 x86_64 兼容层' },
    rt_compatibility: { name: '兼容模式', desc: '兼容模式' },
    rt_directx: { name: 'DirectX', desc: 'DirectX 运行时' },
    rt_speech_onecore: { name: '语音识别运行时', desc: '语音识别运行时' },
    rt_syswow_sapi: { name: 'SysWOW64 SAPI', desc: 'WoW64 SAPI 支持' },
    
    // UWP/AppX
    uwp_taskbar: { name: 'UWP 任务栏', desc: 'UWP 任务栏设置 (隐藏搜索框等)' },
    uwp_startmenu: { name: 'UWP 开始菜单', desc: 'UWP 开始菜单' },
    uwp_settings: { name: 'UWP 设置', desc: 'UWP 设置应用' },
    uwp_ime: { name: 'UWP 输入法', desc: 'UWP 输入法面板' },
    uwp_taskmgr: { name: 'UWP 任务管理器', desc: 'UWP 任务管理器' },
    uwp_appsrvs: { name: 'UWP 应用服务', desc: 'UWP 应用服务框架' },
    uwp_appxapps: { name: 'UWP 预装应用', desc: '预装 UWP 应用' },
    
    // Drivers
    drv_system: { name: '系统驱动', desc: '系统驱动 (存储/显示/音频等)' },
    
    // Applications
    app_7zip: { name: '7-Zip', desc: '7-Zip 压缩工具' },
    app_explorerpp: { name: 'Explorer++', desc: '增强型资源管理器' },
    app_defraggler: { name: 'Defraggler', desc: '磁盘碎片整理工具' },
    app_imdisk: { name: 'ImDisk', desc: '虚拟磁盘工具' },
    app_hotswap: { name: 'HotSwap!', desc: '硬盘热插拔工具' },
    app_penetwork: { name: 'PENetwork', desc: 'PE 网络管理工具' },
    app_yong_ime: { name: '勇哥输入法', desc: '勇哥输入法 (中文)' },
    app_chrome: { name: 'Chrome 浏览器', desc: 'Google Chrome 浏览器' },
    
    // PE Material
    mat_office2007: { name: 'Office 2007', desc: 'Microsoft Office 2007' },
    mat_potplayer: { name: 'PotPlayer', desc: 'PotPlayer 视频播放器' },
    mat_dismpp: { name: 'Dism++', desc: 'Dism++ 系统镜像工具' },
    mat_everything: { name: 'Everything', desc: 'Everything 文件搜索' },
    mat_sumatrapdf: { name: 'SumatraPDF', desc: 'SumatraPDF PDF阅读器' },
    mat_winntsetup: { name: 'WinNTSetup', desc: 'WinNTSetup 系统安装工具' },
    mat_mpcbe: { name: 'MPC-BE', desc: 'MPC-BE 媒体播放器' },
    mat_libreoffice: { name: 'LibreOffice', desc: 'LibreOffice 办公套件' },
    
    build_iso: { name: '生成 ISO', desc: '生成可启动 ISO 镜像' },
    build_usb: { name: '制作 USB', desc: '制作 USB 启动盘' },
  },

  pages: {
    welcome: {
      title: '欢迎使用 BanaPE！',
      intro: '完全可定制的 WinPE 救援与恢复环境构建器，兼容 WimBuilder2 WIN10XPE 项目结构。从侧边栏选择组件并配置您的构建选项。',
      guideTitle: '快速入门指南',
      steps: [
        '<strong>选择源文件：</strong>选择包含 install.wim 的 Windows 安装文件夹',
        '<strong>配置选项：</strong>从侧边栏设置 Configures、ADK OCs 和系统选项',
        '<strong>选择组件：</strong>勾选/取消勾选要包含在 WinPE 构建中的组件（9 个分类共 87 项）',
        '<strong>点击构建：</strong>点击工具栏中的构建按钮开始',
        '<strong>等待并测试：</strong>根据选择通常需要 2-7 分钟完成构建',
      ],
      featWinpe: 'WinPE 构建器',
      featWinpeDesc: '从 Windows 安装源构建完全自定义的 WinPE（Win10/11）救援环境。支持 x64 架构，具备完整的组件模块化能力。',
      featComponents: '87 个组件',
      featComponentsDesc: '覆盖 Shell、网络、音频、系统、驱动、应用等全品类的模块化组件系统。每个组件均可独立启用或禁用。',
      featMultiLang: '5 种语言',
      featMultiLangDesc: '完整 UI 翻译支持简体中文、English、日本語、한국어、Русский。随时在工具栏切换语言。',
      featFast: '快速构建',
      featFastDesc: '基于 WimBuilder2 架构优化的构建流水线。根据所选组件数量，典型构建时间 2-7 分钟。',
      supportTitle: '基于 WimBuilder2 WIN10XPE',
      supportDesc: '组件结构和构建逻辑完全兼容 WimBuilder2-Full 项目。全部 87 个组件映射到原始 WimBuilder2 分类（00-Configures 至 02-PEMaterial）。',
    },
    source: {
      title: '📁 Windows 源配置',
      adkTitle: '📦 ADK 配置',
      pathLabel: '源文件夹路径',
      pathPlaceholder: '选择 Windows 安装源...',
      browse: '浏览...',
      detected: '已检测到：{filename}（{version}，{size}）',
      version: '版本',
      size: '大小',
      images: '映像数',
      bootIndex: '启动映像索引',
      image: '映像 {index}',
      adkPathLabel: 'ADK 安装路径',
      adkPlaceholder: '自动检测...',
    },
    component: {
      idLabel: 'ID',
      categoryLabel: '分类',
      statusLabel: '状态',
      enabled: '已启用',
      disabled: '已禁用',
      enableBtn: '启用组件',
      disableBtn: '禁用组件',
      descTitle: '功能描述',
      infoTitle: '技术信息',
      whatItDoes: '该组件的作用',
      wimSource: 'WimBuilder2 来源',
    },
    build: {
      title: '🔨 构建进度',
      logTitle: '📜 输出日志',
      noLog: '暂无日志条目...',
      clear: '清空',
      copy: '复制',
      copyAll: '复制全部',
    },
    logs: { title: '📜 构建日志', noLog: '暂无可用日志。' },
  },

  steps: [
    { name: '加载配置', desc: '正在加载配置...' },
    { name: '挂载 WIM', desc: '正在挂载 Windows 映像...' },
    { name: '应用补丁', desc: '正在应用组件补丁...' },
    { name: '配置系统', desc: '正在配置系统设置...' },
    { name: '添加驱动', desc: '正在添加设备驱动...' },
    { name: '优化映像', desc: '正在优化 PE 映像...' },
    { name: '生成 ISO', desc: '正在创建可启动 ISO...' },
    { name: '完成', desc: '构建完成！' },
  ],

  log: {
    buildStarted: '=== BanaPE 构建已开始 ===',
    wimDetected: 'WIM 文件检测成功',
    wimFailed: 'WIM 文件检测失败',
    buildComplete: '✅ 构建成功完成！',
    buildCancelled: '用户取消了构建',
    logsCopied: '日志已复制到剪贴板',
    step: '[步骤 {current}/{total}] {name}',
    sourceInfo: '源：{path}',
    componentInfo: '已选 {count} 个组件',
    targetInfo: '目标：{drive} ({type})',
  },

  contentStatus: {
    building: '正在构建 WinPE 映像...',
    ready: '就绪 • 已选 {count} 个组件',
    configure: '配置您的 WinPE 构建',
  },
}
