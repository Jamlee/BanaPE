use crate::engine::config::AppConfig;
use crate::engine::file_extractor::FileExtractor;
use crate::engine::iso_creator::IsoCreator;
use crate::engine::registry::Registry;
use crate::engine::tool_runner::{RunOptions, ToolRunner};
use crate::engine::wim_manager::WimManager;
use crate::patches::{Patch, PatchContext, get_all_patches};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BuildProgress {
    step_index: usize,
    percent: u32,
    message: String,
    detail: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct BuildLog {
    level: String,
    message: String,
    timestamp: u64,
}

struct BuildStep {
    name: &'static str,
    weight: u32,
}

const BUILD_STEPS: [BuildStep; 11] = [
    BuildStep { name: "准备工作目录", weight: 2 },
    BuildStep { name: "复制 boot.wim", weight: 3 },
    BuildStep { name: "挂载 WIM 映像", weight: 10 },
    BuildStep { name: "映射驱动器 (SUBST)", weight: 1 },
    BuildStep { name: "加载注册表", weight: 2 },
    BuildStep { name: "应用补丁", weight: 60 },
    BuildStep { name: "卸载注册表", weight: 2 },
    BuildStep { name: "清理日志文件", weight: 1 },
    BuildStep { name: "提交 WIM 映像", weight: 10 },
    BuildStep { name: "导出优化 WIM", weight: 5 },
    BuildStep { name: "创建启动 ISO", weight: 4 },
];

pub async fn execute_build(app: AppHandle, config: AppConfig, wim_manager: &Arc<tokio::sync::Mutex<WimManager>>, cancelled: Arc<AtomicBool>) -> Result<(), String> {
    let total_weight: u32 = BUILD_STEPS.iter().map(|s| s.weight).sum();
    let app_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let factory_dir = app_root.join("_Factory_");
    let target_dir = factory_dir.join("target");
    let tmp_dir = factory_dir.join("tmp");
    let mount_dir = target_dir.join("mounted");
    let src_dir = target_dir.join("install");

    let src_folder = config.src_folder.clone();
    let boot_wim_src = PathBuf::from(&src_folder).join("sources").join("boot.wim");
    let boot_index = config.boot_index;

    // Build context (shared state)
    let runner = ToolRunner::new();
    let mut registry = Registry::new();
    let extractor = FileExtractor::new();

    let mut ctx_mount_dir = mount_dir.to_string_lossy().to_string();
    let mut ctx_x_drive = ctx_mount_dir.clone();
    let mut ctx_pe_build: u32 = 0;
    let mut ctx_pe_lang = "zh-CN".to_string();
    let ctx_pe_arch = "x64".to_string();

    let boot_wim_dest = target_dir.join("boot.wim");

    let log = |level: &str, msg: &str| {
        let _ = app.emit("build-log", BuildLog {
            level: level.to_string(),
            message: msg.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        });
    };

    let progress = |step_idx: usize, msg: &str, detail: &str| {
        let mut completed_weight: u32 = 0;
        for i in 0..step_idx {
            completed_weight += BUILD_STEPS[i].weight;
        }
        let percent = completed_weight;
        let _ = app.emit("build-progress", BuildProgress {
            step_index: step_idx,
            percent,
            message: msg.to_string(),
            detail: detail.to_string(),
        });
    };

    // Step 0: Prepare work directory
    progress(0, BUILD_STEPS[0].name, "正在执行...");
    log("info", &format!("创建工作目录: {:?}", target_dir));
    std::fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(factory_dir.join("log")).map_err(|e| e.to_string())?;

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 1: Copy boot.wim
    progress(1, BUILD_STEPS[1].name, "正在执行...");
    if !boot_wim_src.exists() {
        return Err(format!("boot.wim not found: {:?}", boot_wim_src));
    }
    log("info", &format!("复制 boot.wim: {:?} -> {:?}", boot_wim_src, boot_wim_dest));
    std::fs::copy(&boot_wim_src, &boot_wim_dest).map_err(|e| e.to_string())?;

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 2: Mount WIM
    progress(2, BUILD_STEPS[2].name, "正在执行...");
    log("info", &format!("挂载 WIM: {:?} [{}] -> {:?}", boot_wim_dest, boot_index, mount_dir));

    // Get WIM info first
    {
        let wm = wim_manager.lock().await;
        if let Ok(info) = wm.get_info(&boot_wim_dest.to_string_lossy()) {
            ctx_pe_lang = info.lang.clone().unwrap_or_default();
        }
    }

    {
        let mut wm = wim_manager.lock().await;
        wm.mount(&boot_wim_dest.to_string_lossy(), boot_index, &mount_dir.to_string_lossy())?;
    }

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 3: SUBST drive
    progress(3, BUILD_STEPS[3].name, "正在执行...");
    let drive_letters = "XABCDEFGHIJKLMNOPQRSTUVWYZ";
    let mut subst_drive = "X:".to_string();
    for letter in drive_letters.chars() {
        let drive = format!("{}:\\", letter);
        if !std::path::Path::new(&drive).exists() {
            subst_drive = format!("{}:", letter);
            break;
        }
    }
    log("info", &format!("映射驱动器: SUBST {} {:?}", subst_drive, mount_dir));
    match runner.run("subst", &vec![subst_drive.clone(), mount_dir.to_string_lossy().into_owned()], RunOptions::default()) {
        Ok(_) => ctx_x_drive = subst_drive.clone(),
        Err(_) => {
            log("warn", &format!("SUBST 失败，使用路径替代: {:?}", mount_dir));
            ctx_x_drive = mount_dir.to_string_lossy().to_string();
        }
    }

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 4: Load registry
    progress(4, BUILD_STEPS[4].name, "正在执行...");
    let install_wim = PathBuf::from(&src_folder).join("sources").join("install.wim");
    let src_dir_str = src_dir.to_string_lossy().to_string();
    if install_wim.exists() {
        log("info", "提取源 WIM 注册表文件...");
        let _ = extractor.extract_registry_files(&install_wim.to_string_lossy(), 1, &src_dir_str);
        registry.load_pe_hives(&mount_dir.to_string_lossy(), Some(&src_dir_str))?;
    } else {
        log("info", "加载 PE 注册表...");
        registry.load_pe_hives(&mount_dir.to_string_lossy(), None)?;
    }

    // Query build version
    if let Ok(output) = registry.query(r"HKLM\Src_SOFTWARE\Microsoft\Windows NT\CurrentVersion", Some("CurrentBuild")) {
        if let Some(caps) = regex::Regex::new(r"CurrentBuild\s+REG_SZ\s+(\d+)")
            .ok()
            .and_then(|re| re.captures(&output))
        {
            if let Some(m) = caps.get(1) {
                ctx_pe_build = m.as_str().parse().unwrap_or(0);
            }
        }
    }

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 5: Apply patches
    progress(5, BUILD_STEPS[5].name, "正在执行...");
    log("info", "开始应用补丁...");

    let patches = get_all_patches();
    let total_patches = patches.len();

    for (i, patch) in patches.iter().enumerate() {
        if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

        let patch_percent = (i * 100) / total_patches;
        progress(5, &format!("应用补丁 [{}/{}]: {}", i + 1, total_patches, patch.name()), &format!("{}%", patch_percent));
        log("info", &format!("[{}/{}] 应用补丁: {}", i + 1, total_patches, patch.name()));

        let patch_ctx = PatchContext {
            src_wim: install_wim.to_string_lossy().to_string(),
            src_index: 1,
            mount_dir: ctx_mount_dir.clone(),
            x_drive: ctx_x_drive.clone(),
            app_root: app_root.to_string_lossy().to_string(),
            factory_dir: factory_dir.to_string_lossy().to_string(),
            tmp_dir: tmp_dir.to_string_lossy().to_string(),
            pe_build: ctx_pe_build,
            pe_arch: ctx_pe_arch.clone(),
            pe_lang: ctx_pe_lang.clone(),
            options: config.options.clone(),
            components: config.components.clone(),
            registry: &registry,
            extractor: &extractor,
            runner: &runner,
        };

        patch.apply(&patch_ctx).map_err(|e| format!("补丁 {} 失败: {}", patch.name(), e))?;
    }

    log("info", "所有补丁应用完成");

    if cancelled.load(Ordering::Relaxed) { return Err("Build cancelled".to_string()); }

    // Step 6: Unload registry
    progress(6, BUILD_STEPS[6].name, "正在执行...");
    log("info", "卸载注册表...");
    registry.unload_pe_hives();

    // Step 7: Cleanup log files
    progress(7, BUILD_STEPS[7].name, "正在执行...");
    log("info", "清理日志文件...");
    let config_dir = mount_dir.join("Windows").join("System32").join("config");
    if config_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&config_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".LOG") || name.contains('{') {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    // Step 8: Commit WIM
    progress(8, BUILD_STEPS[8].name, "正在执行...");
    // Delete SUBST first
    if ctx_x_drive != ctx_mount_dir {
        let _ = runner.run("subst", &vec![ctx_x_drive, "/D".to_string()], RunOptions { ignore_error: true, ..Default::default() });
    }
    log("info", "提交 WIM 映像...");
    {
        let mut wm = wim_manager.lock().await;
        wm.unmount(&mount_dir.to_string_lossy(), true)?;
    }

    // Step 9: Export optimized WIM
    progress(9, BUILD_STEPS[9].name, "正在执行...");
    let exported_wim = target_dir.join("build").join("boot.wim");
    std::fs::create_dir_all(exported_wim.parent().unwrap()).map_err(|e| e.to_string())?;
    log("info", "导出优化 WIM...");
    {
        let wm = wim_manager.lock().await;
        wm.export(&boot_wim_dest.to_string_lossy(), &exported_wim.to_string_lossy(), 1)?;
    }

    // Step 10: Create ISO
    progress(10, BUILD_STEPS[10].name, "正在执行...");
    let iso_dir = app_root.join("_ISO_");
    log("info", "创建启动 ISO...");
    let iso_creator = IsoCreator::new();
    let iso_path = iso_creator.create(
        &src_folder,
        &target_dir.to_string_lossy(),
        &factory_dir.to_string_lossy(),
        &iso_dir.to_string_lossy(),
    )?;

    log("info", &format!("构建完成! ISO: {}", iso_path));
    Ok(())
}
