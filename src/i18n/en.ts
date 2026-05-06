export default {
  app: { name: 'BanaPE', subtitle: 'WinPE Builder', version: 'v1.0.0', style: 'Indigo Tech Style' },
  
  toolbar: { build: 'Build', refresh: 'Refresh', settings: 'Settings', logs: 'Logs', test: 'Test', about: 'About' },

  status: { status: 'Status', source: 'Source', drive: 'Drive', type: 'Type', components: 'Components', notSet: 'Not Set', idle: 'Idle', ready: 'Ready', building: 'Building', error: 'Error', selected: '{count}/{total}' },

  sidebar: { title: 'Components', allExpand: 'All' },

  categories: {
    configures: 'Configures',
    adk_ocs: 'ADK OCs',
    components_shell: 'Shell',
    components_network: 'Network',
    components_audio: 'Audio',
    components_system: 'System Components',
    drivers: 'Drivers',
    apps: 'Applications',
    pematerial: 'PE Material',
  },

  components: {
    // Configures
    cfg_build_catalog: { name: 'Build Catalog', desc: 'Full catalog file & TTS voices' },
    cfg_loader_network: { name: 'Loader Network', desc: 'Initialize network on load' },
    cfg_system_audio: { name: 'Audio Driver Patch', desc: 'Audio driver patch' },
    cfg_system_computername: { name: 'Computer Name', desc: 'Computer name settings' },
    cfg_system_peversion: { name: 'PE Version', desc: 'PE version info configuration' },
    cfg_system_productoptions: { name: 'Product Options', desc: 'Product options (SafeMode etc.)' },
    cfg_system_fbwf: { name: 'FBWF (Filter)', desc: 'File-Based Write Filter' },
    cfg_system_textassoc: { name: 'Text Association', desc: 'Text file associations' },
    cfg_account_admin: { name: 'Admin Account', desc: 'Built-in admin account config' },
    cfg_customization_theme: { name: 'Customization Theme', desc: 'Theme customization (transparency etc.)' },
    
    // ADK OCs
    adk_settings: { name: 'ADK Settings', desc: 'Basic ADK settings' },
    adk_fonts: { name: 'Fonts', desc: 'Font support' },
    adk_network: { name: 'Network Components', desc: 'ADK network components' },
    adk_powershell: { name: 'PowerShell', desc: 'PowerShell environment' },
    adk_setup: { name: 'Setup Components', desc: 'Setup program components' },
    adk_misc: { name: 'Miscellaneous', desc: 'Other ADK components' },
    
    // Shell
    boot2winre: { name: 'Boot to WinRE', desc: 'Boot to WinRE environment (with WiFi)' },
    shell_explorer: { name: 'Explorer', desc: 'Windows File Explorer (theme/settings)' },
    shell_winxshell: { name: 'WinXShell', desc: 'Lightweight Shell alternative' },
    shell_startmenu_classic: { name: 'Classic Start Menu', desc: 'Classic start menu (Classic Shell / StartIsBack)' },
    shell_settings: { name: 'Shell Settings', desc: 'Taskbar/Explorer detailed settings' },
    
    // Network
    network_full: { name: 'Full Network Support', desc: 'Complete network support (TCP/IP/DNS/DHCP/WLAN)' },
    wifi_package: { name: 'WinPE WiFi Package', desc: 'WinPE wireless network package' },
    rndis: { name: 'RNDIS Driver', desc: 'USB network sharing driver (phone USB tethering)' },
    pppoe: { name: 'PPPoE Support', desc: 'PPPoE dial-up connection support' },
    patch_drvinst: { name: 'DrvInst Patch', desc: 'Driver installation program patch' },
    
    // Audio
    audio_core: { name: 'Audio Core', desc: 'Core audio support (WASAPI/MMCSS)' },
    media_player: { name: 'Windows Media Player', desc: 'Windows Media Player' },
    
    // System Components
    bitlocker: { name: 'BitLocker', desc: 'BitLocker disk encryption support' },
    cred_dialog: { name: 'Credential Dialog', desc: 'Credential dialog (UAC prompt)' },
    dwm: { name: 'DWM (Desktop Window Manager)', desc: 'Desktop Window Manager (Aero effects)' },
    devices_bluetooth: { name: 'Bluetooth Support', desc: 'Bluetooth device support' },
    devices_camera: { name: 'Camera Support', desc: 'Camera device support' },
    devices_dsmsvc: { name: 'DSM Service', desc: 'Device Settings Manager service' },
    devices_printer: { name: 'Printer Support', desc: 'Printer and scanner support' },
    ime_zhcn: { name: 'IME Chinese (Simplified)', desc: 'Simplified Chinese IME (MS Pinyin/Wubi etc.)' },
    ime_zhtw: { name: 'IME Chinese (Traditional)', desc: 'Traditional Chinese IME' },
    ime_ja: { name: 'IME Japanese', desc: 'Japanese input method' },
    ime_ko: { name: 'IME Korean', desc: 'Korean input method' },
    ie_browser: { name: 'Internet Explorer', desc: 'IE web browser' },
    mmc: { name: 'MMC Snap-ins', desc: 'Microsoft Management Console (Disk Mgmt etc.)' },
    msi: { name: 'MSI Installer', desc: 'Windows Installer service' },
    mtp_support: { name: 'MTP Support', desc: 'MTP device support (Android phone connection)' },
    netfx: { name: '.NET Framework', desc: '.NET Framework 3.5 runtime' },
    remote_desktop: { name: 'Remote Desktop', desc: 'Remote Desktop (RDP) support' },
    search: { name: 'Windows Search', desc: 'Windows Indexing service' },
    syswow64_basic: { name: 'SysWOW64 Basic', desc: 'WoW64 32-bit compatibility layer base' },
    vcruntime: { name: 'VC++ Runtime', desc: 'Visual C++ runtime libraries' },
    
    // Accessories
    acc_notepad: { name: 'Notepad', desc: 'Text editor' },
    acc_mspaint: { name: 'Paint', desc: 'Drawing tool' },
    acc_snippingtool: { name: 'Snipping Tool', desc: 'Screenshot tool' },
    acc_winphoto: { name: 'Photo Viewer', desc: 'Windows Photo Viewer' },
    acc_accessibility: { name: 'Accessibility', desc: 'Accessibility support' },
    acc_taskmgr: { name: 'Task Manager', desc: 'Process and performance monitor' },
    acc_regedit: { name: 'Registry Editor', desc: 'Registry management tool' },
    
    // Runtime Kits
    rt_appcompat: { name: 'App Compatibility', desc: 'Application compatibility layer' },
    rt_arm64_support: { name: 'ARM64 x86_64 Support', desc: 'ARM64 architecture x86_64 compat layer' },
    rt_compatibility: { name: 'Compatibility Mode', desc: 'Compatibility mode' },
    rt_directx: { name: 'DirectX', desc: 'DirectX runtime' },
    rt_speech_onecore: { name: 'Speech OneCore', desc: 'Speech recognition runtime' },
    rt_syswow_sapi: { name: 'SysWOW64 SAPI', desc: 'WoW64 SAPI support' },
    
    // UWP/AppX
    uwp_taskbar: { name: 'UWP Taskbar', desc: 'UWP taskbar settings (hide search etc.)' },
    uwp_startmenu: { name: 'UWP Start Menu', desc: 'UWP start menu' },
    uwp_settings: { name: 'UWP Settings', desc: 'UWP Settings app' },
    uwp_ime: { name: 'UWP IME', desc: 'UWP input method panel' },
    uwp_taskmgr: { name: 'UWP Task Manager', desc: 'UWP task manager' },
    uwp_appsrvs: { name: 'UWP App Services', desc: 'UWP application services framework' },
    uwp_appxapps: { name: 'UWP AppX Apps', desc: 'Pre-installed UWP applications' },
    
    // Drivers
    drv_system: { name: 'System Drivers', desc: 'System drivers (storage/display/audio etc.)' },
    
    // Applications
    app_7zip: { name: '7-Zip', desc: '7-Zip compression tool' },
    app_explorerpp: { name: 'Explorer++', desc: 'Enhanced file explorer' },
    app_defraggler: { name: 'Defraggler', desc: 'Disk defragmentation tool' },
    app_imdisk: { name: 'ImDisk', desc: 'Virtual disk tool' },
    app_hotswap: { name: 'HotSwap!', desc: 'Hard disk hot-swap tool' },
    app_penetwork: { name: 'PENetwork', desc: 'PE network management tool' },
    app_yong_ime: { name: 'Yong IME', desc: 'Yong input method (Chinese)' },
    app_chrome: { name: 'Chrome Browser', desc: 'Google Chrome browser' },
    
    // PE Material
    mat_office2007: { name: 'Office 2007', desc: 'Microsoft Office 2007' },
    mat_potplayer: { name: 'PotPlayer', desc: 'PotPlayer video player' },
    mat_dismpp: { name: 'Dism++', desc: 'Dism++ system image tool' },
    mat_everything: { name: 'Everything', desc: 'Everything file search' },
    mat_sumatrapdf: { name: 'SumatraPDF', desc: 'SumatraPDF PDF reader' },
    mat_winntsetup: { name: 'WinNTSetup', desc: 'WinNTSetup system installer' },
    mat_mpcbe: { name: 'MPC-BE', desc: 'MPC-BE media player' },
    mat_libreoffice: { name: 'LibreOffice', desc: 'LibreOffice office suite' },
    
    build_iso: { name: 'Build ISO', desc: 'Generate bootable ISO image' },
    build_usb: { name: 'Create USB', desc: 'Create USB boot drive' },
  },

  pages: {
    welcome: {
      title: 'Welcome to BanaPE!',
      intro: 'A fully customizable WinPE rescue and recovery environment builder, compatible with WimBuilder2 WIN10XPE project structure. Select components from the sidebar and configure your build options.',
      guideTitle: 'Quick Start Guide',
      steps: [
        '<strong>Select Source:</strong> Choose your Windows installation folder containing install.wim',
        '<strong>Configure Options:</strong> Set up Configures, ADK OCs and system options from sidebar',
        '<strong>Choose Components:</strong> Check/uncheck components to include in your WinPE build (87 items across 9 categories)',
        '<strong>Press Build:</strong> Click the Build button in toolbar to start the process',
        '<strong>Wait & Test:</strong> The build typically takes 2-7 minutes depending on selections',
      ],
      featWinpe: 'WinPE Builder',
      featWinpeDesc: 'Build fully customized Windows PE (Win10/11) rescue environments from installation sources. Supports x64 architecture with full component modularity.',
      featComponents: '87 Components',
      featComponentsDesc: 'Modular component system covering Shell, Network, Audio, System, Drivers, Apps and more. Each component can be independently enabled or disabled.',
      featMultiLang: '5 Languages',
      featMultiLangDesc: 'Full UI translation in Simplified Chinese, English, Japanese, Korean and Russian. Switch language anytime from toolbar.',
      featFast: 'Fast Build',
      featFastDesc: 'Optimized build pipeline leveraging WimBuilder2 architecture. Typical build completes in 2-7 minutes depending on selected components.',
      supportTitle: 'Based on WimBuilder2 WIN10XPE',
      supportDesc: 'Component structure and build logic are fully compatible with WimBuilder2-Full project. All 87 components map to original WimBuilder2 categories (00-Configures through 02-PEMaterial).',
    },
    source: {
      title: '📁 Windows Source Configuration',
      adkTitle: '📦 ADK Configuration',
      pathLabel: 'Source Folder Path',
      pathPlaceholder: 'Select Windows installation source...',
      browse: 'Browse...',
      detected: 'Detected: {filename} ({version}, {size})',
      version: 'Version',
      size: 'Size',
      images: 'Images',
      bootIndex: 'Boot Image Index',
      image: 'Image {index}',
      adkPathLabel: 'ADK Installation Path',
      adkPlaceholder: 'Auto-detect...',
    },
    component: {
      idLabel: 'ID',
      categoryLabel: 'Category',
      statusLabel: 'Status',
      enabled: 'Enabled',
      disabled: 'Disabled',
      enableBtn: 'Enable Component',
      disableBtn: 'Disable Component',
      descTitle: 'Description',
      infoTitle: 'Technical Info',
      whatItDoes: 'What This Component Does',
      wimSource: 'WimBuilder2 Source',
    },
    build: {
      title: '🔨 Build Progress',
      logTitle: '📜 Output Log',
      noLog: 'No log entries yet...',
      clear: 'Clear',
      copy: 'Copy',
      copyAll: 'Copy All',
    },
    logs: { title: '📜 Build Logs', noLog: 'No log entries available.' },
    test: {
      title: 'Automated Test Suite',
      isoPath: 'ISO Path',
      bootIndex: 'Boot Index',
      timeout: 'Timeout (sec)',
      runBtn: 'Run Tests',
      running: 'Running',
      resultsTitle: 'Test Results',
      executing: 'Executing tests against mounted PE image',
      logTitle: 'Test Log',
    },
  },

  steps: [
    { name: 'Load Config', desc: 'Loading configuration...' },
    { name: 'Mount WIM', desc: 'Mounting Windows image...' },
    { name: 'Apply Patches', desc: 'Applying component patches...' },
    { name: 'Configure System', desc: 'Configuring system settings...' },
    { name: 'Add Drivers', desc: 'Adding device drivers...' },
    { name: 'Optimize Image', desc: 'Optimizing PE image...' },
    { name: 'Generate ISO', desc: 'Creating bootable ISO...' },
    { name: 'Complete', desc: 'Build finished!' },
  ],

  log: {
    buildStarted: '=== BanaPE Build Started ===',
    wimDetected: 'WIM file detected successfully',
    wimFailed: 'Failed to detect WIM file',
    buildComplete: '✅ Build completed successfully!',
    buildCancelled: 'Build cancelled by user',
    logsCopied: 'Logs copied to clipboard',
    step: '[Step {current}/{total}] {name}',
    sourceInfo: 'Source: {path}',
    componentInfo: 'Components: {count} selected',
    targetInfo: 'Target: {drive} ({type})',
  },

  contentStatus: {
    building: 'Building WinPE image...',
    ready: 'Ready to build • {count} components',
    configure: 'Configure your WinPE build',
  },
}
