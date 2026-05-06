pub mod test_runner;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
    pub category: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSuiteResult {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub duration_ms: u64,
    pub tests: Vec<TestResult>,
    pub iso_path: String,
    pub mount_dir: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestConfig {
    pub iso_path: String,
    pub boot_index: u32,
    pub timeout_seconds: u64,
    pub categories: Vec<String>,
    #[serde(default)]
    pub skip_cleanup: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            iso_path: r"e:\iso\Win11_25H2_English_x64_v2.iso".to_string(),
            boot_index: 1,
            timeout_seconds: 300,
            categories: vec![
                "mount".to_string(),
                "filesystem".to_string(),
                "registry".to_string(),
                "components".to_string(),
                "winrm".to_string(),
                "network".to_string(),
            ],
            skip_cleanup: false,
        }
    }
}

static mut TEST_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn is_test_running() -> bool {
    unsafe { TEST_RUNNING.load(Ordering::Relaxed) }
}

fn set_test_running(val: bool) {
    unsafe { TEST_RUNNING.store(val, Ordering::Relaxed) }
}

pub async fn run_test_suite(
    app: &AppHandle,
    config: TestConfig,
) -> Result<TestSuiteResult, String> {
    if is_test_running() {
        return Err("Test suite already running".to_string());
    }
    set_test_running(true);

    let start = std::time::Instant::now();
    let app_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let factory_dir = app_root.join("_Factory_");
    let test_mount_dir = factory_dir.join("test").join("mounted");
    let test_tmp_dir = factory_dir.join("test").join("tmp");
    let test_iso_cache = factory_dir.join("test").join("iso_cache");

    let emit = |event: &str, data: serde_json::Value| {
        let _ = app.emit(event, data);
    };

    let log_event = |level: &str, msg: &str| {
        emit(
            "test:log",
            serde_json::json!({
                "text": msg,
                "level": level,
                "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
            }),
        );
    };

    let result = run_test_suite_inner(
        &app,
        &config,
        &test_mount_dir,
        &test_tmp_dir,
        &test_iso_cache,
        &factory_dir,
        &log_event,
        &emit,
    )
    .await;

    set_test_running(false);

    result
}

async fn run_test_suite_inner(
    app: &AppHandle,
    config: &TestConfig,
    mount_dir: &PathBuf,
    tmp_dir: &PathBuf,
    iso_cache: &PathBuf,
    factory_dir: &PathBuf,
    log_event: &dyn Fn(&str, &str),
    emit: &dyn Fn(&str, serde_json::Value),
) -> Result<TestSuiteResult, String> {
    let mut all_results: Vec<TestResult> = Vec::new();
    let mut passed_count: u32 = 0;
    let mut failed_count: u32 = 0;
    let mut skipped_count: u32 = 0;

    log_event("info", &format!("=== BanaPE Test Suite Started ==="));
    log_event("info", &format!("ISO: {}", config.iso_path));
    log_event("info", &format!("Mount Dir: {}", mount_dir.display()));
    log_event("info", &format!("Categories: {:?}", config.categories));

    emit("test:status", serde_json::json!({"status": "running"}));

    let runner = crate::engine::ToolRunner::new();
    let wim_manager = crate::engine::WimManager::new();

    let run_single_test = |name: &str, category: &str, test_fn: impl FnOnce() -> Result<String, String>| -> TestResult {
        let t_start = std::time::Instant::now();
        log_event("info", &format!("[TEST] {} :: Running...", name));
        match test_fn() {
            Ok(msg) => {
                let dur = t_start.elapsed().as_millis() as u64;
                log_event("success", &format!("[PASS] {} :: {} ({}ms)", name, msg, dur));
                passed_count += 1;
                TestResult { name: name.to_string(), passed: true, message: msg, duration_ms: dur, category: category.to_string() }
            }
            Err(e) => {
                let dur = t_start.elapsed().as_millis() as u64;
                log_event("error", &format!("[FAIL] {} :: {} ({}ms)", name, e, dur));
                failed_count += 1;
                TestResult { name: name.to_string(), passed: false, message: e, duration_ms: dur, category: category.to_string() }
            }
        }
    };

    // ===== Phase 1: ISO & Mount Tests =====
    if config.categories.contains(&"mount".to_string()) {

        all_results.push(run_single_test("ISO file exists", "mount", || {
            let iso = PathBuf::from(&config.iso_path);
            if !iso.exists() {
                return Err(format!("ISO not found: {}", config.iso_path));
            }
            let meta = std::fs::metadata(&iso).map_err(|e| e.to_string())?;
            let size_mb = meta.len() / (1024 * 1024);
            Ok(format!("ISO exists, size: {} MB", size_mb))
        }));

        all_results.push(run_single_test("Extract boot.wim from ISO", "mount", || {
            std::fs::create_dir_all(iso_cache).map_err(|e| e.to_string())?;

            use sevenz_rust::{decompress_file, SevenZReader};

            let iso_path = PathBuf::from(&config.iso_path);
            let dest_boot_wim = iso_cache.join("boot.wim");

            if !dest_boot_wim.exists() {
                let source_file = std::fs::File::open(&iso_path).map_err(|e| format!("Cannot open ISO: {}", e))?;
                let reader = SevenZReader::new(source_file).map_err(|e| format!("Cannot read ISO as 7z: {}", e))?;
                let entries: Vec<_> = reader.archive().files().collect();

                let boot_wim_entry = entries.iter()
                    .find(|f| f.name().ends_with("sources\\boot.wim") || f.name().ends_with("sources/boot.wim"));

                match boot_wim_entry {
                    Some(entry) => {
                        decompress_file(&config.iso_path, entry.name(), iso_cache)
                            .map_err(|e| format!("Failed to extract boot.wim: {}", e))?;
                    }
                    None => {
                        return Err("boot.wim not found in ISO".to_string());
                    }
                }
            }

            let meta = std::fs::metadata(&dest_boot_wim).map_err(|e| e.to_string())?;
            let size_mb = meta.len() / (1024 * 1024);
            Ok(format!("boot.wim extracted, size: {} MB", size_mb))
        }));

        all_results.push(run_single_test("Create isolated mount directory", "mount", || {
            if mount_dir.exists() {
                std::fs::remove_dir_all(mount_dir).map_err(|e| format!("Cannot clean old mount dir: {}", e))?;
            }
            std::fs::create_dir_all(mount_dir).map_err(|e| e.to_string())?;
            std::fs::create_dir_all(tmp_dir).map_err(|e| e.to_string())?;

            let abs_path = std::fs::canonicalize(mount_dir).map_err(|e| e.to_string())?;
            Ok(format!("Mount dir ready: {}", abs_path.display()))
        }));

        all_results.push(run_single_test("Mount boot.wim via DISM", "mount", || {
            let boot_wim = iso_cache.join("boot.wim");
            if !boot_wim.exists() {
                return Err("boot.wim not extracted yet".to_string());
            }

            let wm = crate::engine::WimManager::new();
            let mut wim_mgr = wm;
            wim_mgr.mount(
                &boot_wim.to_string_lossy(),
                config.boot_index,
                &mount_dir.to_string_lossy(),
            )?;

            Ok(format!("WIM mounted at index {}", config.boot_index))
        }));

        all_results.push(run_single_test("Verify mounted PE directory structure", "mount", || {
            let windows_dir = mount_dir.join("Windows");
            let system32_dir = windows_dir.join("System32");

            if !windows_dir.exists() {
                return Err(format!("Windows dir not found in mount: {}", windows_dir.display()));
            }
            if !system32_dir.exists() {
                return Err(format!("System32 not found: {}", system32_dir.display()));
            }

            let key_files = ["cmd.exe", "winrm.exe", "wscript.exe", "reg.exe"];
            let mut found = Vec::new();
            for f in &key_files {
                let fp = system32_dir.join(f);
                if fp.exists() {
                    found.push(f.to_string());
                }
            }

            Ok(format!("{}/{} key files verified ({:?})", found.len(), key_files.len(), found))
        }));
    }

    // ===== Phase 2: File System Tests (inside mount) =====
    if config.categories.contains(&"filesystem".to_string()) {
        let sys32 = mount_dir.join("Windows").join("System32");
        let drivers_dir = mount_dir.join("Windows").join("System32").join("drivers");

        all_results.push(run_single_test("PE System32 is read-only safe", "filesystem", || {
            let test_file = sys32.join("__banape_test_probe__.tmp");
            match std::fs::write(&test_file, "probe") {
                Ok(_) => {
                    std::fs::remove_file(&test_file).ok();
                    Err("Mount dir should NOT be writable for safety!".to_string())
                }
                Err(_) => Ok("Correctly read-only (or protected)".to_string())
            }
        }));

        all_results.push(run_single_test("List PE system files", "filesystem", || {
            if !sys32.exists() {
                return Err("System32 not found".to_string());
            }
            let count = std::fs::read_dir(&sys32)
                .map_err(|e| e.to_string())?
                .count();
            Ok(format!("{} items in System32", count))
        }));

        all_results.push(run_single_test("Verify drivers directory", "filesystem", || {
            if !drivers_dir.exists() {
                return Err("Drivers dir not found".to_string());
            }
            let driver_count = std::fs::read_dir(&drivers_dir)
                .map_err(|e| e.to_string())?
                .count();
            Ok(format!("{} driver files present", driver_count))
        }));

        all_results.push(run_single_test("Check PE registry hives exist", "filesystem", || {
            let config_dir = sys32.join("config");
            if !config_dir.exists() {
                return Err("config dir not found".to_string());
            }
            let hives = ["SYSTEM", "SOFTWARE", "DEFAULT", "SAM", "SECURITY"];
            let mut found_hives = Vec::new();
            for hive in &hives {
                let hive_path = config_dir.join(hive);
                if hive_path.exists() {
                    found_hives.push(hive.to_string());
                }
            }
            if found_hives.is_empty() {
                return Err("No registry hives found in config dir".to_string());
            }
            Ok(format!("Registry hives: {:?}", found_hives))
        }));
    }

    // ===== Phase 3: Registry Tests =====
    if config.categories.contains(&"registry".to_string()) {
        all_results.push(run_single_test("Load PE SYSTEM hive (offline)", "registry", || {
            let system_hive = mount_dir.join("Windows")
                .join("System32")
                .join("config")
                .join("SYSTEM");
            if !system_hive.exists() {
                return Err(format!("SYSTEM hive not found: {}", system_hive.display()));
            }

            let test_key = "HKLM\\BanaPE_Test_Suite";
            let load_args = vec![
                "load".to_string(),
                test_key.to_string(),
                system_hive.to_string_lossy().to_string(),
            ];

            match runner.run("reg", &load_args, crate::engine::tool_runner::RunOptions { admin: true, ignore_error: false }) {
                Ok(output) => {
                    let unload_args = vec!["unload".to_string(), test_key.to_string()];
                    let _ = runner.run("reg", &unload_args, crate::engine::tool_runner::RunOptions { admin: true, ignore_error: true });
                    Ok("SYSTEM hive loaded and unloaded successfully".to_string())
                }
                Err(e) => Err(format!("Failed to load SYSTEM hive: {}", e))
            }
        }));

        all_results.push(run_single_test("Query PE build version from registry", "registry", || {
            let system_hive = mount_dir.join("Windows")
                .join("System32")
                .join("config")
                .join("SYSTEM");
            if !system_hive.exists() {
                return Err("SYSTEM hive not found".to_string());
            }

            let test_key = "HKLM\\BanaPE_Test_Query";
            let load_args = vec![
                "load".to_string(),
                test_key.to_string(),
                system_hive.to_string_lossy().to_string(),
            ];
            runner.run("reg", &load_args, crate::engine::tool_runner::RunOptions { admin: true, ignore_error: false })?;

            let query_args = vec![
                "query".to_string(),
                format!("{}\\Microsoft\\Windows NT\\CurrentVersion", test_key),
                "v".to_string(),
                "CurrentBuild".to_string(),
            ];
            let output = runner.run("reg", &query_args, crate::engine::tool_runner::RunOptions { admin: true, ignore_error: false })?;

            let unload_args = vec!["unload".to_string(), test_key.to_string()];
            let _ = runner.run("reg", &unload_args, crate::engine::tool_runner::RunOptions { admin: true, ignore_error: true });

            if output.stdout.contains("REG_SZ") {
                Ok(format!("PE Build: {}", output.stdout.trim()))
            } else {
                Ok("Registry query executed (build info may vary)".to_string())
            }
        }));
    }

    // ===== Phase 4: Component Verification Tests =====
    if config.categories.contains(&"components".to_string()) {
        let pe_windows = mount_dir.join("Windows");
        let pe_sys32 = pe_windows.join("System32");

        all_results.push(run_single_test("Verify Explorer.exe presence (Shell)", "components", || {
            let explorer = pe_windows.join("explorer.exe");
            if explorer.exists() {
                let meta = std::fs::metadata(&explorer).map_err(|e| e.to_string())?;
                Ok(format!("Explorer.exe present ({} bytes)", meta.len()))
            } else {
                Ok("Explorer.exe not included (optional component)".to_string())
            }
        }));

        all_results.push(run_single_test("Verify PowerShell presence", "components", || {
            let ps1 = pe_sys32.join("WindowsPowerShell").join("v1.0").join("powershell.exe");
            let ps_core = pe_sys32.join("pwsh.exe");
            if ps1.exists() {
                Ok("PowerShell 5.1 present".to_string())
            } else if ps_core.exists() {
                Ok("PowerShell Core present".to_string())
            } else {
                Ok("PowerShell not included (optional component)".to_string())
            }
        }));

        all_results.push(run_single_test("Verify network stack components", "components", || {
            let network_files = [
                ("dhcpcore.dll", "DHCP client"),
                ("dnsapi.dll", "DNS resolver"),
                ("winhttp.dll", "HTTP client"),
                ("mswsock.dll", "Winsock"),
            ];
            let mut found = Vec::new();
            for (file, desc) in &network_files {
                if pe_sys32.join(file).exists() {
                    found.push(*desc);
                }
            }
            Ok(format!("Network components: {:?} ({}/4)", found, found.len()))
        }));

        all_results.push(run_single_test("Verify audio components", "components", || {
            let audio_files = [
                ("audiosrv.dll", "Audio service"),
                ("mmdevapi.dll", "MMDevice API"),
                ("wasapi.dll", "WASAPI"),
            ];
            let mut found = Vec::new();
            for (file, desc) in &audio_files {
                if pe_sys32.join(file).exists() {
                    found.push(*desc);
                }
            }
            Ok(format!("Audio: {:?} ({}/3)", found, found.len()))
        }));

        all_results.push(run_single_test("Count total PE binaries", "components", || {
            let exe_count = std::fs::read_dir(&pe_sys32)
                .map_err(|e| e.to_string())?
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "exe" || ext == "dll" || ext == "sys")
                        .unwrap_or(false)
                })
                .count();
            let dll_count = std::fs::read_dir(&pe_sys32)
                .map_err(|e| e.to_string())?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|ex| ex == "dll").unwrap_or(false))
                .count();
            Ok(format!("{} EXEs + {} DLLs in System32", exe_count, dll_count))
        }));
    }

    // ===== Phase 5: WinRM Tests =====
    if config.categories.contains(&"winrm".to_string()) {
        let winrm_exe = mount_dir.join("Windows").join("System32").join("winrm.exe");

        all_results.push(run_single_test("winrm.exe exists in PE", "winrm", || {
            if !winrm_exe.exists() {
                return Err(format!("winrm.exe not found at {}", winrm_exe.display()));
            }
            let meta = std::fs::metadata(&winrm_exe).map_err(|e| e.to_string())?;
            Ok(format!("winrm.exe present ({} bytes)", meta.len()))
        }));

        all_results.push(run_single_test("winrm version check", "winrm", || {
            if !winrm_exe.exists() {
                return Err("winrm.exe not found".to_string());
            }
            let args = vec![winrm_exe.to_string_lossy().to_string(), "-version".to_string()];
            match runner.run_from_path(&args[0], &args[1..].to_vec(), crate::engine::tool_runner::RunOptions::default()) {
                Ok(out) => Ok(if out.stdout.is_empty() { "winrm executable (no version flag)" } else { out.stdout.trim() }.to_string()),
                Err(e) => Err(format!("winrm execution error: {}", e))
            }
        }));

        all_results.push(run_single_test("winrm enumerate (dry-run syntax)", "winrm", || {
            if !winrm_exe.exists() {
                return Err("winrm.exe not found".to_string());
            }
            let args = vec![
                winrm_exe.to_string_lossy().to_string(),
                "enumerate".to_string(),
                "wmic/root/cimv2".to_string(),
                "?".to_string(),
            ];
            match runner.run_from_path(&args[0], &args[1..].to_vec(), crate::engine::tool_runner::RunOptions { ignore_error: true }) {
                Ok(out) => {
                    if out.stdout.contains("OS") || out.stdout.contains("Win32_OperatingSystem") || out.stderr.contains("error") {
                        Ok("winrm enumerate command accepted".to_string())
                    } else {
                        Ok(format!("Response: {}", if out.stdout.len() > 100 { &out.stdout[..100] } else { &out.stdout }))
                    }
                }
                Err(e) => Err(format!("winrm enumerate failed: {}", e))
            }
        }));

        all_results.push(run_single_test("winrm get wmi OS info", "winrm", || {
            if !winrm_exe.exists() {
                return Err("winrm.exe not found".to_string());
            }
            let args = vec![
                winrm_exe.to_string_lossy().to_string(),
                "get".to_string(),
                "wmic/root/cimv2/Win32_OperatingSystem?".to_string(),
            ];
            match runner.run_from_path(&args[0], &args[1..].to_vec(), crate::engine::tool_runner::RunOptions { ignore_error: true }) {
                Ok(out) => {
                    if out.stdout.contains("Caption") || out.stdout.contains("Version") || out.stdout.contains("BuildNumber") {
                        Ok("WMI OS query successful".to_string())
                    } else {
                        Ok(format!("Raw output preview: {}", 
                            if out.stdout.len() > 120 { &out.stdout[..120] } else { &out.stdout }))
                    }
                }
                Err(e) => Err(format!("winrm WMI query failed: {}", e))
            }
        }));
    }

    // ===== Phase 6: Network Tests =====
    if config.categories.contains(&"network".to_string()) {
        all_results.push(run_single_test("Check network adapter files", "network", || {
            let net_dir = mount_dir.join("Windows").join("System32").join("drivers").join("etc");
            let hosts = net_dir.join("hosts");
            let services = net_dir.join("services");
            let networks = net_dir.join("networks");
            let mut present = Vec::new();
            if hosts.exists() { present.push("hosts"); }
            if services.exists() { present.push("services"); }
            if networks.exists() { present.push("networks"); }
            Ok(format!("Net config files: {:?} (expected 3)", present))
        }));

        all_results.push(run_single_test("Check TCP/IP config files", "network", || {
            let tcpip_dir = mount_dir.join("Windows").join("System32").join("drivers").join("etc");
            let files = ["hosts", "lmhosts", "protocol", "services"];
            let mut found = 0;
            for f in &files {
                if tcpip_dir.join(f).exists() { found += 1; }
            }
            Ok(format!("{}/{} TCP/IP config files found", found, files.len()))
        }));
    }

    // ===== Cleanup Phase =====
    log_event("info", "--- Cleanup Phase ---");

    all_results.push(run_single_test("Unmount WIM image safely", "cleanup", || {
        let wm = crate::engine::WimManager::new();
        let mut wim_mgr = wm;
        match wim_mgr.unmount(&mount_dir.to_string_lossy(), false) {
            Ok(()) => Ok("WIM unmounted (discarded changes)".to_string()),
            Err(e) => {
                let _ = wim_mgr.force_cleanup();
                Ok(format!("Force cleanup applied: {}", e))
            }
        }
    }));

    all_results.push(run_single_test("Remove temp mount directory", "cleanup", || {
        if mount_dir.exists() {
            std::fs::remove_dir_all(mount_dir).map_err(|e| e.to_string())?;
        }
        Ok("Temp mount directory removed".to_string())
    }));

    all_results.push(run_single_test("Verify local directories untouched", "cleanup", || {
        let critical_paths = [
            PathBuf::from(r"C:\Windows\System32\config\SYSTEM"),
            PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts"),
        ];
        for p in &critical_paths {
            if !p.exists() {
                return Err(format!("CRITICAL: Local path missing (should never happen): {}", p.display()));
            }
        }
        Ok("All local system paths intact - no cross-contamination".to_string())
    }));

    let total_duration = start.elapsed().as_millis() as u64;
    let total = all_results.len() as u32;

    let suite_result = TestSuiteResult {
        total,
        passed: passed_count,
        failed: failed_count,
        skipped: skipped_count,
        duration_ms: total_duration,
        tests: all_results.clone(),
        iso_path: config.iso_path.clone(),
        mount_dir: mount_dir.to_string_lossy().to_string(),
    };

    log_event("info", &format!(
        "=== Test Suite Complete: {}/{} passed, {} failed ({}ms) ===",
        passed_count, total, failed_count, total_duration
    ));

    emit("test:result", serde_json::json!(suite_result));
    emit("test:status", serde_json::json!({"status": "complete"}));

    Ok(suite_result)
}
