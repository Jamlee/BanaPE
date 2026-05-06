import en from './en'

export default {
  app: { ...en.app },
  toolbar: { build: 'ビルド', refresh: '更新', settings: '設定', logs: 'ログ', utilities: 'ユーティリティ', about: 'について' },
  status: { status: 'ステータス', source: 'ソース', drive: 'ドライブ', type: 'タイプ', components: 'コンポーネント', notSet: '未設定', idle: '待機中', ready: '準備完了', building: 'ビルド中', error: 'エラー', selected: '{count}/{total}' },
  sidebar: { title: 'コンポーネント', allExpand: 'すべて展開' },

  categories: {
    configures: '構成',
    adk_ocs: 'ADK オプション',
    components_shell: 'シェル',
    components_network: 'ネットワーク',
    components_audio: 'オーディオ',
    components_system: 'システム コンポーネント',
    drivers: 'ドライバー',
    apps: 'アプリケーション',
    pematerial: 'PE マテリアル',
  },

  components: {
    cfg_build_catalog: { name: '完全カタログ', desc: '完全カタログファイルと TTS 音声' },
    cfg_loader_network: { name: 'ローダー ネットワーク', desc: 'ロード時にネットワークを初期化' },
    cfg_system_audio: { name: 'オーディオ ドライバー パッチ', desc: 'オーディオ ドライバー パッチ' },
    cfg_system_computername: { name: 'コンピューター名', desc: 'コンピューター名の設定' },
    cfg_system_peversion: { name: 'PE バージョン', desc: 'PE バージョン情報の構成' },
    cfg_system_productoptions: { name: '製品オプション', desc: '製品オプション (セーフモード等)' },
    cfg_system_fbwf: { name: 'FBWF (フィルター)', desc: 'ファイルベース書き込みフィルター' },
    cfg_system_textassoc: { name: 'テキスト関連付け', desc: 'テキストファイルの関連付け' },
    cfg_account_admin: { name: '管理者アカウント', desc: '組み込み管理者アカウント構成' },
    cfg_customization_theme: { name: 'カスタマイズ テーマ', desc: 'テーマ カスタマイズ (透明度等)' },
    
    adk_settings: { name: 'ADK 設定', desc: 'ADK 基本設定' },
    adk_fonts: { name: 'フォント', desc: 'フォント サポート' },
    adk_network: { name: 'ネットワーク コンポーネント', desc: 'ADK ネットワーク コンポーネント' },
    adk_powershell: { name: 'PowerShell', desc: 'PowerShell 環境' },
    adk_setup: { name: 'セットアップ コンポーネント', desc: 'セットアップ プログラム コンポーネント' },
    adk_misc: { name: 'その他', desc: 'その他の ADK コンポーネント' },
    
    boot2winre: { name: 'WinRE へブート', desc: 'WinRE 環境へブート (WiFi 含む)' },
    shell_explorer: { name: 'エクスプローラー', desc: 'Windows ファイル エクスプローラー (テーマ/設定)' },
    shell_winxshell: { name: 'WinXShell', desc: '軽量シェル代替案' },
    shell_startmenu_classic: { name: 'クラシック スタートメニュー', desc: 'クラシック スタートメニュー (Classic Shell / StartIsBack)' },
    shell_settings: { name: 'シェル設定', desc: 'タスクバー/エクスプローラー詳細設定' },
    
    network_full: { name: 'フル ネットワーク サポート', desc: '完全なネットワーク サポート (TCP/IP/DNS/DHCP/WLAN)' },
    wifi_package: { name: 'WinPE WiFi パッケージ', desc: 'WinPE 無線ネットワーク パッケージ' },
    rndis: { name: 'RNDIS ドライバー', desc: 'USB ネットワーク共有ドライバー (スマホ USB ヶザリング)' },
    pppoe: { name: 'PPPoE サポート', desc: 'PPPoE ダイヤルアップ接続サポート' },
    patch_drvinst: { name: 'DrvInst パッチ', desc: 'ドライバー インストーラー パッチ' },
    
    audio_core: { name: 'オーディオ コア', desc: 'コア オーディオ サポート (WASAPI/MMCSS)' },
    media_player: { name: 'Windows Media Player', desc: 'Windows Media Player' },
    
    bitlocker: { name: 'BitLocker', desc: 'BitLocker ディスク暗号化サポート' },
    cred_dialog: { name: '資格情報ダイアログ', desc: '資格情報ダイアログ (UAC プロンプト)' },
    dwm: { name: 'DWM (デスクトップウィンドウマネージャ)', desc: 'デスクトップ ウィンドウ マネージャー (Aero 効果)' },
    devices_bluetooth: { name: 'Bluetooth サポート', desc: 'Bluetooth デバイス サポート' },
    devices_camera: { name: 'カメラ サポート', desc: 'カメラ デバイス サポート' },
    devices_dsmsvc: { name: 'DSM サービス', desc: 'デバイス設定マネージャー サービス' },
    devices_printer: { name: 'プリンター サポート', desc: 'プリンターとスキャナー サポート' },
    ime_zhcn: { name: 'IME 中国語(簡体)', desc: '簡体字中国語入力方式 (MS ピンイン/五筆等)' },
    ime_zhtw: { name: 'IME 中国語(繁体)', desc: '繁体字中国語入力方式' },
    ime_ja: { name: 'IME 日本語', desc: '日本語入力方式' },
    ime_ko: { name: 'IME 韓国語', desc: '韓国語入力方式' },
    ie_browser: { name: 'Internet Explorer', desc: 'IE ウェブブラウザ' },
    mmc: { name: 'MMC スナップイン', desc: 'Microsoft 管理コンソール (ディスク管理等)' },
    msi: { name: 'MSI インストーラー', desc: 'Windows Installer サービス' },
    mtp_support: { name: 'MTP サポート', desc: 'MTP デバイス サポート (Android 携帯接続)' },
    netfx: { name: '.NET Framework', desc: '.NET Framework 3.5 ランタイム' },
    remote_desktop: { name: 'リモート デスクトップ', desc: 'リモート デスクトップ (RDP) サポート' },
    search: { name: 'Windows 検索', desc: 'Windows インデックス サービス' },
    syswow64_basic: { name: 'SysWOW64 基本', desc: 'WoW64 32ビット互換レイヤー基本' },
    vcruntime: { name: 'VC++ ランタイム', desc: 'Visual C++ ランタイム ライブラリ' },
    
    acc_notepad: { name: 'メモ帳', desc: 'テキスト エディター' },
    acc_mspaint: { name: 'ペイント', desc: '描画ツール' },
    acc_snippingtool: { name: '切り取りツール', desc: 'スクリーンショット ツール' },
    acc_winphoto: { name: 'フォト ビューアー', desc: 'Windows フォト ビューアー' },
    acc_accessibility: { name: 'アクセシビリティ', desc: 'アクセシビリティ サポート' },
    acc_taskmgr: { name: 'タスクマネージャー', desc: 'プロセスとパフォーマンス モニター' },
    acc_regedit: { name: 'レジストリ エディター', desc: 'レジストリ管理ツール' },
    
    rt_appcompat: { name: 'アプリ互換性', desc: 'アプリケーション互換性レイヤー' },
    rt_arm64_support: { name: 'ARM64 x86_64 サポート', desc: 'ARM64 アーキテクチャ x86_64 互換レイヤー' },
    rt_compatibility: { name: '互換モード', desc: '互換モード' },
    rt_directx: { name: 'DirectX', desc: 'DirectX ランタイム' },
    rt_speech_onecore: { name: 'Speech OneCore', desc: '音声認識ランタイム' },
    rt_syswow_sapi: { name: 'SysWOW64 SAPI', desc: 'WoW64 SAPI サポート' },
    
    uwp_taskbar: { name: 'UWP タスクバー', desc: 'UWP タスクバー設定 (検索非表示等)' },
    uwp_startmenu: { name: 'UWP スタートメニュー', desc: 'UWP スタートメニュー' },
    uwp_settings: { name: 'UWP 設定', desc: 'UWP 設定アプリ' },
    uwp_ime: { name: 'UWP IME', desc: 'UWP 入力パネル' },
    uwp_taskmgr: { name: 'UWP タスクマネージャー', desc: 'UWP タスクマネージャー' },
    uwp_appsrvs: { name: 'UWP アプリ サービス', desc: 'UWP アプリケーション サービス フレームワーク' },
    uwp_appxapps: { name: 'UWP AppX アプリ', desc: 'プレインストール UWP アプリ' },
    
    drv_system: { name: 'システム ドライバー', desc: 'システム ドライバー (ストレージ/ディスプレイ/オーディオ等)' },
    
    app_7zip: { name: '7-Zip', desc: '7-Zip 圧縮ツール' },
    app_explorerpp: { name: 'Explorer++', desc: '拡張ファイル エクスプローラー' },
    app_defraggler: { name: 'Defraggler', desc: 'ディスク最適化ツール' },
    app_imdisk: { name: 'ImDisk', desc: '仮想ディスク ツール' },
    app_hotswap: { name: 'HotSwap!', desc: 'ハードディスク ホットスワップ ツール' },
    app_penetwork: { name: 'PENetwork', desc: 'PE ネットワーク管理ツール' },
    app_yong_ime: { name: 'Yong IME', desc: 'Yong 入力方式 (中国語)' },
    app_chrome: { name: 'Chrome ブラウザ', desc: 'Google Chrome ブラウザ' },
    
    mat_office2007: { name: 'Office 2007', desc: 'Microsoft Office 2007' },
    mat_potplayer: { name: 'PotPlayer', desc: 'PotPlayer 動画プレイヤー' },
    mat_dismpp: { name: 'Dism++', desc: 'Dism++ システムイメージ ツール' },
    mat_everything: { name: 'Everything', desc: 'Everything ファイル検索' },
    mat_sumatrapdf: { name: 'SumatraPDF', desc: 'SumatraPDF PDF リーダー' },
    mat_winntsetup: { name: 'WinNTSetup', desc: 'WinNTSetup システムインストーラー' },
    mat_mpcbe: { name: 'MPC-BE', desc: 'MPC-BE メディアプレイヤー' },
    mat_libreoffice: { name: 'LibreOffice', desc: 'LibreOffice オフィススイート' },
    
    build_iso: { name: 'ISO 作成', desc: 'ブータブル ISO イメージを生成' },
    build_usb: { name: 'USB 作成', desc: 'USB ブートドライブを作成' },
  },

  pages: {
    welcome: { title: 'BanaPE へようこそ！', intro: 'WimBuilder2 WIN10XPE プロジェクト構造と完全互換の WinPE 救援・復旧環境ビルダー。サイドバーからコンポーネントを選択し、ビルドオプションを設定してください。', guideTitle: 'クイックスタートガイド', steps: ['<strong>ソースを選択：</strong>install.wim を含む Windows インストールフォルダーを選択','<strong>オプション構成：</strong>サイドバーから Configures、ADK OCs、システムオプションを設定','<strong>コンポーネント選択：</strong>WinPE ビルドに含めるコンポーネントのチェックオン/オフ（9カテゴリ87項目）','<strong>ビルド開始：</strong>ツールバーのビルドボタンをクリック','<strong>待機・テスト：</strong>選択内容により通常 2〜7 分で完了'], featWinpe: 'WinPE ビルダー', featWinpeDesc: 'Windows インストールソースから完全カスタマイズ可能な WinPE（Win10/11）救援環境をビルド。x64 アーキテクチャ対応、完全モジュラーなコンポーネント構成。', featComponents: '87 コンポーネント', featComponentsDesc: 'Shell、ネットワーク、オーディオ、システム、ドライバー、アプリなどをカバーするモジュラーコンポーネントシステム。各コンポーネントは独立して有効/無効化可能。', featMultiLang: '5言語対応', featMultiLangDesc: '簡体字中国語、English、日本語、한국어、Русский の完全 UI 翻訳。ツールバーからいつでも言語切替可能。', featFast: '高速ビルド', featFastDesc: 'WimBuilder2 アーキテクチャに基づいた最適化ビルドパイプライン。選択したコンポーネント数により通常 2-7 分で完了。', supportTitle: 'WimBuilder2 WIN10XPE ベース', supportDesc: 'コンポーネント構造とビルドロジックは WimBuilder2-Full プロジェクトと完全互換。全 87 コンポーネントが元の WimBuilder2 カテゴリ（00-Configures 〜 02-PEMaterial）にマッピング。' },
    source: { title: '📁 Windows ソース設定', adkTitle: '📦 ADK 設定', pathLabel: 'ソースフォルダーパス', pathPlaceholder: 'Windows インストールソースを選択...', browse: '参照...', detected: '検出されました：{filename}（{version}、{size}）', version: 'バージョン', size: 'サイズ', images: 'イメージ数', bootIndex: 'ブートイメージインデックス', image: 'イメージ {index}', adkPathLabel: 'ADK インストールパス', adkPlaceholder: '自動検出...' },
    component: { idLabel: 'ID', categoryLabel: 'カテゴリ', statusLabel: 'ステータス', enabled: '有効', disabled: '無効', enableBtn: 'コンポーネントを有効にする', disableBtn: 'コンポーネントを無効にする', descTitle: '説明', infoTitle: '技術情報', whatItDoes: 'このコンポーネントの機能', wimSource: 'WimBuilder2 ソース' },
    build: { title: '🔨 ビルド進行状況', logTitle: '📜 出力ログ', noLog: 'ログエントリはまだありません...', clear: 'クリア', copy: 'コピー', copyAll: 'すべてコピー' },
    logs: { title: '📜 ビルドログ', noLog: '利用可能なログはありません。' },
  },

  steps: [
    { name: '設定読込', desc: '設定を読み込んでいます...' }, { name: 'WIM マウント', desc: 'Windows イメージをマウントしています...' }, { name: 'パッチ適用', desc: 'コンポーネントパッチを適用しています...' }, { name: 'システム構成', desc: 'システム設定を構成しています...' }, { name: 'ドライバー追加', desc: 'デバイス ドライバーを追加しています...' }, { name: 'イメージ最適化', desc: 'PE イメージを最適化しています...' }, { name: 'ISO 生成', desc: 'ブータブル ISO を作成しています...' }, { name: '完了', desc: 'ビルドが完了しました！' }
  ],

  log: { buildStarted: '=== BanaPE ビルド開始 ===', wimDetected: 'WIM ファイルが正常に検出されました', wimFailed: 'WIM ファイルの検出に失敗しました', buildComplete: '✅ ビルドが正常に完了しました！', buildCancelled: 'ユーザーによりビルドがキャンセルされました', logsCopied: 'ログがクリップボードにコピーされました', step: '[ステップ {current}/{total}] {name}', sourceInfo: 'ソース：{path}', componentInfo: '{count} 個のコンポーネントが選択されています', targetInfo: 'ターゲット：{drive} ({type})' },

  contentStatus: { building: 'WinPE イメージをビルド中...', ready: '準備完了 • {count} コンポーネント', configure: 'WinPE ビルドを設定' },
}
