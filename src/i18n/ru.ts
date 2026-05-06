import en from './en'

export default {
  app: { ...en.app },
  toolbar: { build: 'Сборка', refresh: 'Обновить', settings: 'Настройки', logs: 'Журнал', test: 'Тест', utilities: 'Утилиты', about: 'О программе' },
  status: { status: 'Статус', source: 'Источник', drive: 'Диск', type: 'Тип', components: 'Компоненты', notSet: 'Не задано', idle: 'Ожидание', ready: 'Готово', building: 'Сборка', error: 'Ошибка', selected: '{count}/{total}' },
  sidebar: { title: 'Компоненты', allExpand: 'Все' },

  categories: {
    configures: 'Конфигурация',
    adk_ocs: 'Доп. компоненты ADK',
    components_shell: 'Оболочка',
    components_network: 'Сеть',
    components_audio: 'Аудио',
    components_system: 'Системные компоненты',
    drivers: 'Драйверы',
    apps: 'Приложения',
    pematerial: 'Материалы PE',
  },

  components: {
    cfg_build_catalog: { name: 'Полный каталог', desc: 'Полный файл каталога и голоса TTS' },
    cfg_loader_network: { name: 'Сеть загрузчика', desc: 'Инициализация сети при загрузке' },
    cfg_system_audio: { name: 'Патч аудиодрайвера', desc: 'Патч для аудиодрайверов' },
    cfg_system_computername: { name: 'Имя компьютера', desc: 'Настройки имени компьютера' },
    cfg_system_peversion: { name: 'Версия PE', desc: 'Конфигурация версии PE' },
    cfg_system_productoptions: { name: 'Параметры продукта', desc: 'Параметры продукта (Безопасный режим и т.д.)' },
    cfg_system_fbwf: { name: 'FBWF (фильтр)', desc: 'Фильтр записи на основе файлов' },
    cfg_system_textassoc: { name: 'Связь текста', desc: 'Связи текстовых файлов' },
    cfg_account_admin: { name: 'Учетная запись админа', desc: 'Конфигурация встроенной учётной записи администратора' },
    cfg_customization_theme: { name: 'Кастомизация темы', desc: 'Кастомизация темы (прозрачность и т.д.)' },
    
    adk_settings: { name: 'Настройки ADK', desc: 'Базовые настройки ADK' },
    adk_fonts: { name: 'Шрифты', desc: 'Поддержка шрифтов' },
    adk_network: { name: 'Сетевые компоненты', desc: 'Сетевые компоненты ADK' },
    adk_powershell: { name: 'PowerShell', desc: 'Окружение PowerShell' },
    adk_setup: { name: 'Компоненты установки', desc: 'Компоненты программы установки' },
    adk_misc: { name: 'Прочее', desc: 'Другие компоненты ADK' },
    
    boot2winre: { name: 'Загрузка в WinRE', desc: 'Загрузка в среду WinRE (с WiFi)' },
    shell_explorer: { name: 'Проводник', desc: 'Проводник Windows (тема/настройки)' },
    shell_winxshell: { name: 'WinXShell', desc: 'Лёгкая альтернатива оболочки' },
    shell_startmenu_classic: { name: 'Классическое меню Пуск', desc: 'Классическое меню Пуск (Classic Shell / StartIsBack)' },
    shell_settings: { name: 'Настройки оболочки', desc: 'Подробные настройки панели задач/проводника' },
    
    network_full: { name: 'Полная поддержка сети', desc: 'Полная поддержка сети (TCP/IP/DNS/DHCP/WLAN)' },
    wifi_package: { name: 'Пакет WinPE WiFi', desc: 'Пакет беспроводной сети WinPE' },
    rndis: { name: 'Драйвер RNDIS', desc: 'Драйвер общего доступа к USB (телефон USB)' },
    pppoe: { name: 'Поддержка PPPoE', desc: 'Поддержка PPPoE-подключения' },
    patch_drvinst: { name: 'Патч DrvInst', desc: 'Патч установщика драйверов' },
    
    audio_core: { name: 'Ядро аудио', desc: 'Базовая поддержка аудио (WASAPI/MMCSS)' },
    media_player: { name: 'Windows Media Player', desc: 'Windows Media Player' },
    
    bitlocker: { name: 'BitLocker', desc: 'Поддержка шифрования дисков BitLocker' },
    cred_dialog: { name: 'Диалог учётных данных', desc: 'Диалог учётных данных (приглашение UAC)' },
    dwm: { name: 'DWM (диспетчер окон)', desc: 'Диспетчер окон рабочего стола (эффект Aero)' },
    devices_bluetooth: { name: 'Поддержка Bluetooth', desc: 'Поддержка устройств Bluetooth' },
    devices_camera: { name: 'Поддержка камеры', desc: 'Поддержка устройств камеры' },
    devices_dsmsvc: { name: 'Служба DSM', desc: 'Служба диспетчера настроек устройства' },
    devices_printer: { name: 'Поддержка принтеров', desc: 'Поддержка принтеров и сканеров' },
    ime_zhcn: { name: 'IME Китайский (упрощ.)', desc: 'Метод ввода упрощённого китайского (MS Pinyin/Wubi и т.д.)' },
    ime_zhtw: { name: 'IME Китайский (трад.)', desc: 'Метод ввода традиционного китайского' },
    ime_ja: { name: 'IME Японский', desc: 'Метод ввода японского' },
    ime_ko: { name: 'IME Корейский', desc: 'Метод ввода корейского' },
    ie_browser: { name: 'Internet Explorer', desc: 'Веб-браузер IE' },
    mmc: { name: 'ММС-модули', desc: 'Консоль управления Microsoft (управление дисками и т.д.)' },
    msi: { name: 'Установщик MSI', desc: 'Служба установщика Windows' },
    mtp_support: { name: 'Поддержка MTP', desc: 'Поддержка устройств MTP (подключение Android)' },
    netfx: { name: '.NET Framework', desc: 'Среда выполнения .NET Framework 3.5' },
    remote_desktop: { name: 'Удалённый рабочий стол', desc: 'Поддержка удалённого рабочего стола (RDP)' },
    search: { name: 'Поиск Windows', desc: 'Служба индексации Windows' },
    syswow64_basic: { name: 'SysWOW64 базовая', desc: 'Базовая поддержка слоя совместимости WoW64 32-бит' },
    vcruntime: { name: 'VC++ Runtime', desc: 'Библиотеки времени выполнения Visual C++' },
    
    acc_notepad: { name: 'Блокнот', desc: 'Текстовый редактор' },
    acc_mspaint: { name: 'Paint', desc: 'Инструмент рисования' },
    acc_snippingtool: { name: 'Ножницы', desc: 'Инструмент для снимков экрана' },
    acc_winphoto: { name: 'Просмотр фотографий', desc: 'Просмотрщик фотографий Windows' },
    acc_accessibility: { name: 'Спец. возможности', desc: 'Поддержка специальных возможностей' },
    acc_taskmgr: { name: 'Диспетчер задач', desc: 'Монитор процессов и производительности' },
    acc_regedit: { name: 'Редактор реестра', desc: 'Инструмент управления реестром' },
    
    rt_appcompat: { name: 'Совместимость приложений', desc: 'Слой совместимости приложений' },
    rt_arm64_support: { name: 'Поддержка ARM64', desc: 'Слой совместимости ARM64 x86_64' },
    rt_compatibility: { name: 'Режим совместимости', desc: 'Режим совместимости' },
    rt_directx: { name: 'DirectX', desc: 'Среда выполнения DirectX' },
    rt_speech_onecore: { name: 'Speech OneCore', desc: 'Среда распознавания речи' },
    rt_syswow_sapi: { name: 'SysWOW64 SAPI', desc: 'Поддержка SAPI WoW64' },
    
    uwp_taskbar: { name: 'Панель задач UWP', desc: 'Настройки панели задач UWP (скрыть поиск и т.д.)' },
    uwp_startmenu: { name: 'Меню Пуск UWP', desc: 'Меню Пуск UWP' },
    uwp_settings: { name: 'Настройки UWP', desc: 'Приложение настроек UWP' },
    uwp_ime: { name: 'IME UWP', desc: 'Панель ввода UWP' },
    uwp_taskmgr: { name: 'Диспетчер задач UWP', desc: 'Диспетчер задач UWP' },
    uwp_appsrvs: { name: 'Службы приложений UWP', desc: 'Каркас служб приложений UWP' },
    uwp_appxapps: { name: 'Предустановленные приложения UWP', desc: 'Предустановленные приложения UWP' },
    
    drv_system: { name: 'Системные драйверы', desc: 'Системные драйверы (хранилище/дисплей/аудио и т.д.)' },
    
    app_7zip: { name: '7-Zip', desc: 'Инструмент сжатия 7-Zip' },
    app_explorerpp: { name: 'Explorer++', desc: 'Расширенный проводник' },
    app_defraggler: { name: 'Defraggler', desc: 'Инструмент дефрагментации диска' },
    app_imdisk: { name: 'ImDisk', desc: 'Инструмент виртуальных дисков' },
    app_hotswap: { name: 'HotSwap!', desc: 'Инструмент горячей замены жёсткого диска' },
    app_penetwork: { name: 'PENetwork', desc: 'Инstrument управления сетью PE' },
    app_yong_ime: { name: 'Yong IME', desc: 'Метод ввода Yong (китайский)' },
    app_chrome: { name: 'Chrome', desc: 'Веб-браузер Google Chrome' },
    
    mat_office2007: { name: 'Office 2007', desc: 'Microsoft Office 2007' },
    mat_potplayer: { name: 'PotPlayer', desc: 'Видеоплеер PotPlayer' },
    mat_dismpp: { name: 'Dism++', desc: 'Инструмент системных образов Dism++' },
    mat_everything: { name: 'Everything', desc: 'Поиск файлов Everything' },
    mat_sumatrapdf: { name: 'SumatraPDF', desc: 'Читатель PDF SumatraPDF' },
    mat_winntsetup: { name: 'WinNTSetup', desc: 'Инструмент установки системы WinNTSetup' },
    mat_mpcbe: { name: 'MPC-BE', desc: 'Медиаплеер MPC-BE' },
    mat_libreoffice: { name: 'LibreOffice', desc: 'Офисный пакет LibreOffice' },
    
    build_iso: { name: 'Создать ISO', desc: 'Генерация загрузочного образа ISO' },
    build_usb: { name: 'Создать USB', desc: 'Создание загрузочного USB-накопителя' },
  },

  pages: {
    welcome: { title: 'Добро пожаловать в BanaPE!', intro: 'Полностью настраиваемый конструктор среды восстановления и спасения WinPE, полностью совместимый со структурой проекта WimBuilder2 WIN10XPE. Выберите компоненты из боковой панели и настройте параметры сборки.', guideTitle: 'Краткое руководство', steps: ['<strong>Выбор источника:</strong> Выберите папку установки Windows, содержащую install.wim','<strong>Настройка параметров:</strong> Настройте Configures, ADK OCs и параметры системы из боковой панели','<strong>Выбор компонентов:</strong> Вкл/выкл компоненты для включения в сборку WinPE (9 категорий, 87 элементов)','<strong>Запуск сборки:</strong> Нажмите кнопку Сборки на панели инструментов','<strong>Ожидание и тест:</strong> Обычно занимает 2-7 минут в зависимости от выбора'], featWinpe: 'Конструктор WinPE', featWinpeDesc: 'Сборка полностью настраиваемых сред восстановления WinPE (Win10/11) из установочных источников. Поддержка x64 архитектуры с полной модульностью компонентов.', featComponents: '87 компонентов', featComponentsDesc: 'Модульная система компонентов, охватывающая Shell, сеть, аудио, систему, драйверы, приложения и др. Каждый компонент можно независимо включить или отключить.', featMultiLang: '5 языков', featMultiLangDesc: 'Полный перевод UI на упрощённый китайский, English, 日本語, 한국어, Русский. Переключение языка в любое время из панели инструментов.', featFast: 'Быстрая сборка', featFastDesc: 'Оптимизированный конвейер сборки на основе архитектуры WimBuilder2. Типичное время сборки 2-7 минут в зависимости от выбранных компонентов.', supportTitle: 'На базе WimBuilder2 WIN10XPE', supportDesc: 'Структура компонентов и логика сборки полностью совместимы с проектом WimBuilder2-Full. Все 87 компонентов сопоставлены с оригинальными категориями WimBuilder2 (00-Configures — 02-PEMaterial).' },
    source: { title: '📁 Настройка источника Windows', adkTitle: '📦 Настройка ADK', pathLabel: 'Путь к папке источника', pathPlaceholder: 'Выберите источник установки Windows...', browse: 'Обзор...', detected: 'Обнаружено: {filename} ({version}, {size})', version: 'Версия', size: 'Размер', images: 'Образы', bootIndex: 'Индекс загрузочного образа', image: 'Образ {index}', adkPathLabel: 'Путь установки ADK', adkPlaceholder: 'Автоопределение...' },
    component: { idLabel: 'ID', categoryLabel: 'Категория', statusLabel: 'Статус', enabled: 'Включено', disabled: 'Отключено', enableBtn: 'Включить компонент', disableBtn: 'Отключить компонент', descTitle: 'Описание', infoTitle: 'Техническая информация', whatItDoes: 'Что делает этот компонент', wimSource: 'Источник WimBuilder2' },
    build: { title: '🔨 Прогресс сборки', logTitle: '📜 Журнал вывода', noLog: 'Записей в журнале пока нет...', clear: 'Очистить', copy: 'Копировать', copyAll: 'Копировать всё' },
    logs: { title: '📜 Журнал сборки', noLog: 'Доступных записей нет.' },
    test: { title: 'Автоматизированный набор тестов', isoPath: 'Путь к ISO', bootIndex: 'Индекс загрузки', timeout: 'Таймаут (сек)', runBtn: 'Запустить тесты', running: 'Выполняется', resultsTitle: 'Результаты тестов', executing: 'Выполнение тестов против смонтированного PE-образа...', logTitle: 'Журнал тестов' },
  },

  steps: [
    { name: 'Загрузка конф.', desc: 'Загрузка конфигурации...' }, { name: 'Монтирование WIM', desc: 'Монтирование образа Windows...' }, { name: 'Применение патчей', desc: 'Применение патчей компонентов...' }, { name: 'Конф. системы', desc: 'Настройка системных параметров...' }, { name: 'Добавление драйв.', desc: 'Добавление драйверов устройств...' }, { name: 'Оптимизация образа', desc: 'Оптимизация образа PE...' }, { name: 'Создание ISO', desc: 'Создание загрузочного ISO...' }, { name: 'Готово', desc: 'Сборка завершена!' }
  ],

  log: { buildStarted: '=== Сборка BanaPE начата ===', wimDetected: 'Файл WIM успешно обнаружен', wimFailed: 'Не удалось обнаружить файл WIM', buildComplete: '✅ Сборка успешно завершена!', buildCancelled: 'Сборка отменена пользователем', logsCopied: 'Журнал скопирован в буфер обмена', step: '[Шаг {current}/{total}] {name}', sourceInfo: 'Источник: {path}', componentInfo: 'Выбрано компонентов: {count}', targetInfo: 'Цель: {drive} ({type})' },

  contentStatus: { building: 'Создание образа WinPE...', ready: 'Готово • {count} компонентов', configure: 'Настройте вашу сборку WinPE' },
}
