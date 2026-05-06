import en from './en'

export default {
  app: { ...en.app },
  toolbar: { build: '빌드', refresh: '새로고침', settings: '설정', logs: '로그', utilities: '유틸리티', about: '정보' },
  status: { status: '상태', source: '소스', drive: '드라이브', type: '유형', components: '구성 요소', notSet: '설정 안됨', idle: '대기 중', ready: '준비 완료', building: '빌드 중', error: '오류', selected: '{count}/{total}' },
  sidebar: { title: '구성 요소', allExpand: '전체 펼치기' },

  categories: {
    configures: '구성',
    adk_ocs: 'ADK 선택 구성 요소',
    components_shell: '셸',
    components_network: '네트워크',
    components_audio: '오디오',
    components_system: '시스템 구성 요소',
    drivers: '드라이버',
    apps: '응용 프로그램',
    pematerial: 'PE 자료',
  },

  components: {
    cfg_build_catalog: { name: '전체 카탈로그', desc: '전체 카탈로그 파일 및 TTS 음성' },
    cfg_loader_network: { name: '로더 네트워크', desc: '로드 시 네트워크 초기화' },
    cfg_system_audio: { name: '오디오 드라이버 패치', desc: '오디오 드라이버 패치' },
    cfg_system_computername: { name: '컴퓨터 이름', desc: '컴퓨터 이름 설정' },
    cfg_system_peversion: { name: 'PE 버전', desc: 'PE 버전 정보 구성' },
    cfg_system_productoptions: { name: '제품 옵션', desc: '제품 옵션 (안전 모드 등)' },
    cfg_system_fbwf: { name: 'FBWF 필터', desc: '파일 기반 쓰기 필터' },
    cfg_system_textassoc: { name: '텍스트 연결', desc: '텍스트 파일 연결' },
    cfg_account_admin: { name: '관리자 계정', desc: '내장 관리자 계정 구성' },
    cfg_customization_theme: { name: '테마 사용자 지정', desc: '테마 사용자 정의 (투명도 등)' },
    
    adk_settings: { name: 'ADK 설정', desc: 'ADK 기본 설정' },
    adk_fonts: { name: '글꼴', desc: '글꼴 지원' },
    adk_network: { name: '네트워크 구성 요소', desc: 'ADK 네트워크 구성 요소' },
    adk_powershell: { name: 'PowerShell', desc: 'PowerShell 환경' },
    adk_setup: { name: '설치 프로그램 구성 요소', desc: '설치 프로그램 구성 요소' },
    adk_misc: { name: '기타', desc: '기타 ADK 구성 요소' },
    
    boot2winre: { name: 'WinRE 부팅', desc: 'WinRE 환경으로 부팅 (WiFi 포함)' },
    shell_explorer: { name: '탐색기', desc: 'Windows 파일 탐색기 (테마/설정)' },
    shell_winxshell: { name: 'WinXShell', desc: '경량 셸 대안' },
    shell_startmenu_classic: { name: '클래식 시작 메뉴', desc: '클래식 시작 메뉴 (Classic Shell / StartIsBack)' },
    shell_settings: { name: '셸 설정', desc: '작업 표시줄/탐색기 상세 설정' },
    
    network_full: { name: '전체 네트워크 지원', desc: '완전한 네트워크 지원 (TCP/IP/DNS/DHCP/WLAN)' },
    wifi_package: { name: 'WinPE WiFi 패키지', desc: 'WinPE 무선 네트워크 패키지' },
    rndis: { name: 'RNDIS 드라이버', desc: 'USB 네트워크 공유 드라이버 (폰 USB 테더링)' },
    pppoe: { name: 'PPPoE 지원', desc: 'PPPoE 다이얼업 연결 지원' },
    patch_drvinst: { name: 'DrvInst 패치', desc: '드라이버 설치 프로그램 패치' },
    
    audio_core: { name: '오디오 코어', desc: '핵심 오디오 지원 (WASAPI/MMCSS)' },
    media_player: { name: 'Windows Media Player', desc: 'Windows Media Player' },
    
    bitlocker: { name: 'BitLocker', desc: 'BitLocker 디스크 암호화 지원' },
    cred_dialog: { name: '자격 증명 대화 상자', desc: '자격 증명 대화 상자 (UAC 프롬프트)' },
    dwm: { name: 'DWM (데스크톱 창 관리자)', desc: '데스크톱 창 관리자 (Aero 효과)' },
    devices_bluetooth: { name: '블루투스 지원', desc: '블루투스 장치 지원' },
    devices_camera: { name: '카메라 지원', desc: '카메라 장치 지원' },
    devices_dsmsvc: { name: 'DSM 서비스', desc: '장치 설정 관리자 서비스' },
    devices_printer: { name: '프린터 지원', desc: '프린터 및 스캐너 지원' },
    ime_zhcn: { name: 'IME 중국어(간체)', desc: '간체 중국어 입력 방식 (MS 병음/오표 등)' },
    ime_zhtw: { name: 'IME 중국어(번체)', desc: '번체 중국어 입력 방식' },
    ime_ja: { name: 'IME 일본어', desc: '일본어 입력 방식' },
    ime_ko: { name: 'IME 한국어', desc: '한국어 입력 방식' },
    ie_browser: { name: 'Internet Explorer', desc: 'IE 웹 브라우저' },
    mmc: { name: 'MMC 스냅인', desc: 'Microsoft 관리 콘솔 (디스크 관리 등)' },
    msi: { name: 'MSI 설치 프로그램', desc: 'Windows Installer 서비스' },
    mtp_support: { name: 'MTP 지원', desc: 'MTP 장치 지원 (Android 폰 연결)' },
    netfx: { name: '.NET Framework', desc: '.NET Framework 3.5 런타임' },
    remote_desktop: { name: '원격 데스크톱', desc: '원격 데스크톱 (RDP) 지원' },
    search: { name: 'Windows 검색', desc: 'Windows 인덱싱 서비스' },
    syswow64_basic: { name: 'SysWOW64 기본', desc: 'WoW64 32비트 호환 계층 기본' },
    vcruntime: { name: 'VC++ 런타임', desc: 'Visual C++ 런타임 라이브러리' },
    
    acc_notepad: { name: '메모장', desc: '텍스트 편집기' },
    acc_mspaint: { name: '그림판', desc: '그리기 도구' },
    acc_snippingtool: { name: '잘라내기 도구', desc: '스크린샷 도구' },
    acc_winphoto: { name: '사진 보기', desc: 'Windows 사진 보기' },
    acc_accessibility: { name: '접근성', desc: '접근성 지원' },
    acc_taskmgr: { name: '작업 관리자', desc: '프로세스 및 성능 모니터' },
    acc_regedit: { name: '레지스트리 편집기', desc: '레지스트리 관리 도구' },
    
    rt_appcompat: { name: '앱 호환성', desc: '애플리케이션 호환성 레이어' },
    rt_arm64_support: { name: 'ARM64 호환', desc: 'ARM64 아키텍처 x86_64 호환 레이어' },
    rt_compatibility: { name: '호환 모드', desc: '호환 모드' },
    rt_directx: { name: 'DirectX', desc: 'DirectX 런타임' },
    rt_speech_onecore: { name: 'Speech OneCore', desc: '음성 인식 런타임' },
    rt_syswow_sapi: { name: 'SysWOW64 SAPI', desc: 'WoW64 SAPI 지원' },
    
    uwp_taskbar: { name: 'UWP 작업 표시줄', desc: 'UWP 작업 표시줄 설정 (검색 숨김 등)' },
    uwp_startmenu: { name: 'UWP 시작 메뉴', desc: 'UWP 시작 메뉴' },
    uwp_settings: { name: 'UWP 설정', desc: 'UWP 설정 앱' },
    uwp_ime: { name: 'UWP IME', desc: 'UWP 입력 패널' },
    uwp_taskmgr: { name: 'UWP 작업 관리자', desc: 'UWP 작업 관리자' },
    uwp_appsrvs: { name: 'UWP 앱 서비스', desc: 'UWP 애플리케이션 서비스 프레임워크' },
    uwp_appxapps: { name: 'UWP 사전 설치 앱', desc: '사전 설치 UWP 앱' },
    
    drv_system: { name: '시스템 드라이버', desc: '시스템 드라이버 (저장/디스플레이/오디오 등)' },
    
    app_7zip: { name: '7-Zip', desc: '7-Zip 압축 도구' },
    app_explorerpp: { name: 'Explorer++', desc: '확장 파일 탐색기' },
    app_defraggler: { name: 'Defraggler', desc: '디스크 조각 모음 도구' },
    app_imdisk: { name: 'ImDisk', desc: '가상 디스크 도구' },
    app_hotswap: { name: 'HotSwap!', desc: '하드 디스크 핫 스왑 도구' },
    app_penetwork: { name: 'PENetwork', desc: 'PE 네트워크 관리 도구' },
    app_yong_ime: { name: 'Yong IME', desc: 'Yong 입력 방식 (중국어)' },
    app_chrome: { name: 'Chrome 브라우저', desc: 'Google Chrome 브라우저' },
    
    mat_office2007: { name: 'Office 2007', desc: 'Microsoft Office 2007' },
    mat_potplayer: { name: 'PotPlayer', desc: 'PotPlayer 동영상 플레이어' },
    mat_dismpp: { name: 'Dism++', desc: 'Dism++ 시스템 이미지 도구' },
    mat_everything: { name: 'Everything', desc: 'Everything 파일 검색' },
    mat_sumatrapdf: { name: 'SumatraPDF', desc: 'SumatraPDF PDF 리더' },
    mat_winntsetup: { name: 'WinNTSetup', desc: 'WinNTSetup 시스템 설치 도구' },
    mat_mpcbe: { name: 'MPC-BE', desc: 'MPC-BE 미디어 플레이어' },
    mat_libreoffice: { name: 'LibreOffice', desc: 'LibreOffice 오피스 스위트' },
    
    build_iso: { name: 'ISO 생성', desc: '부팅 가능한 ISO 이미지 생성' },
    build_usb: { name: 'USB 생성', desc: 'USB 부팅 디스크 만들기' },
  },

  pages: {
    welcome: { title: 'BanaPE에 오신 것을 환영합니다!', intro: 'WimBuilder2 WIN10XPE 프로젝트 구조와 완전 호환되는 완전 사용자 정의 가능한 WinPE 구조 및 복구 환경 빌더입니다. 사이드바에서 구성 요소를 선택하고 빌드 옵션을 구성하세요.', guideTitle: '빠른 시작 가이드', steps: ['<strong>소스 선택:</strong> install.wim이 포함된 Windows 설치 폴더 선택','<strong>옵션 구성:</strong> 사이드바에서 Configures, ADK OCs 및 시스템 옵션 설정','<strong>구성 요소 선택:</strong> WinPE 빌드에 포함할 구성 요소 체크/해제(9카테고리 87항목)','<strong>빌드 시작:</strong> 도구 모음의 빌드 버튼 클릭','<strong>대기 및 테스트:</strong>선택에 따라 일반적으로 2-7분 소요'], featWinpe: 'WinPE 빌더', featWinpeDesc: 'Windows 설치 소스에서 완전 사용자 정의 WinPE(Win10/11) 복구 환경을 빌드합니다. x64 아키텍처 지원, 완전 모듈화 컴포넌트 구성.', featComponents: '87개 구성 요소', featComponentsDesc: 'Shell, 네트워크, 오디오, 시스템, 드라이버, 앱 등을 포괄하는 모듈형 구성 요소 시스템. 각 구성 요소는 독립적으로 활성화/비활성화 가능.', featMultiLang: '5개 언어', featMultiLangDesc: '간체 중국어, English, 日本語, 한국어, Русский의 완전 UI 번역. 도구 모음에서 언제든지 언어 전환 가능.', featFast: '고속 빌드', featFastDesc: 'WimBuilder2 아키텍처 기반 최적화 빌드 파이프라인. 선택한 구성 요소 수에 따라 일반적으로 2-7분 내 완료.', supportTitle: 'WimBuilder2 WIN10XPE 기반', supportDesc: '구성 요소 구조와 빌드 로직이 WimBuilder2-Full 프로젝트와 완전 호환. 전체 87개 구성 요소가 원본 WimBuilder2 카테고리(00-Configures ~ 02-PEMaterial)에 매핑.' },
    source: { title: '📁 Windows 소스 설정', adkTitle: '📦 ADK 설정', pathLabel: '소스 폴더 경로', pathPlaceholder: 'Windows 설치 소스 선택...', browse: '찾아보기...', detected: '감지됨: {filename} ({version}, {size})', version: '버전', size: '크기', images: '이미지 수', bootIndex: '부팅 이미지 인덱스', image: '이미지 {index}', adkPathLabel: 'ADK 설치 경로', adkPlaceholder: '자동 감지...' },
    component: { idLabel: 'ID', categoryLabel: '카테고리', statusLabel: '상태', enabled: '활성화됨', disabled: '비활성화됨', enableBtn: '구성 요소 활성화', disableBtn: '구성 요소 비활성화', descTitle: '설명', infoTitle: '기술 정보', whatItDoes: '이 구성 요소의 기능', wimSource: 'WimBuilder2 소스' },
    build: { title: '🔨 빌드 진행률', logTitle: '📜 출력 로그', noLog: '로그 항목이 아직 없습니다...', clear: '지우기', copy: '복사', copyAll: '모두 복사' },
    logs: { title: '📜 빌드 로그', noLog: '사용 가능한 로그가 없습니다.' },
  },

  steps: [
    { name: '설정 로드', desc: '설정을 불러오는 중...' }, { name: 'WIM 마운트', desc: 'Windows 이미지를 마운트하는 중...' }, { name: '패치 적용', desc: '구성 요소 패치를 적용하는 중...' }, { name: '시스템 구성', desc: '시스템 설정을 구성하는 중...' }, { name: '드라이버 추가', desc: '장치 드라이버를 추가하는 중...' }, { name: '이미지 최적화', desc: 'PE 이미지를 최적화하는 중...' }, { name: 'ISO 생성', desc: '부팅 가능한 ISO를 생성하는 중...' }, { name: '완료', desc: '빌드가 완료되었습니다!' }
  ],

  log: { buildStarted: '=== BanaPE 빌드 시작 ===', wimDetected: 'WIM 파일이 성공적으로 감지되었습니다', wimFailed: 'WIM 파일 감지 실패', buildComplete: '✅ 빌드가 성공적으로 완료되었습니다!', buildCancelled: '사용자가 빌드를 취소했습니다', logsCopied: '로그가 클립보드에 복사되었습니다', step: '[단계 {current}/{total}] {name}', sourceInfo: '소스: {path}', componentInfo: '{count}개 구성 요소 선택됨', targetInfo: '대상: {drive} ({type})' },

  contentStatus: { building: 'WinPE 이미지를 빌드하는 중...', ready: '준비 완료 • {count} 개 구성 요소', configure: 'WinPE 빌드 구성' },
}
