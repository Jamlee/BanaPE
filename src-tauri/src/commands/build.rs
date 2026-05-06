use tauri::{State, AppHandle, Emitter};
use crate::AppState;
use crate::engine::config::AppConfig;
use crate::engine::builder;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex as TokioMutex;
use serde_json::json;

struct BuildState {
    cancelled: Arc<AtomicBool>,
    is_building: Arc<AtomicBool>,
}

static mut BUILD_STATE: Option<BuildState> = None;

// ===== Tauri 事件定义 =====
pub const EVENT_BUILD_PROGRESS: &str = "build:progress";
pub const EVENT_BUILD_LOG: &str = "build:log";
pub const EVENT_BUILD_STATUS: &str = "build:status";
pub const EVENT_BUILD_STEP: &str = "build:step";
pub const EVENT_BUILD_COMPLETE: &str = "build:complete";
pub const EVENT_BUILD_ERROR: &str = "build:error";

/// 发送构建进度事件
fn emit_progress(app: &AppHandle, percent: u32) {
    let _ = app.emit(EVENT_BUILD_PROGRESS, json!({
        "percent": percent,
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
    }));
}

/// 发送构建日志事件
fn emit_log(app: &AppHandle, text: &str, level: &str) {
    let _ = app.emit(EVENT_BUILD_LOG, json!({
        "text": text,
        "level": level,
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
    }));
}

/// 发送构建状态事件
fn emit_status(app: &AppHandle, status: &str) {
    let _ = app.emit(EVENT_BUILD_STATUS, json!({
        "status": status,
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
    }));
}

/// 发送构建步骤事件
fn emit_step(app: &AppHandle, step_index: usize, step_name: &str, step_desc: &str) {
    let _ = app.emit(EVENT_BUILD_STEP, json!({
        "index": step_index,
        "name": step_name,
        "desc": step_desc,
        "timestamp": chrono::Local::now().format("%H:%M:%S").to_string(),
    }));
}

#[tauri::command]
pub async fn build_start(
    app: AppHandle,
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    unsafe {
        if BUILD_STATE.is_some() {
            return Err("Build already in progress".to_string());
        }
        BUILD_STATE = Some(BuildState {
            cancelled: Arc::new(AtomicBool::new(false)),
            is_building: Arc::new(AtomicBool::new(true)),
        });
    }

    let build_state = unsafe { BUILD_STATE.as_ref().unwrap() };
    let cancelled = build_state.cancelled.clone();
    
    let wim_manager = Arc::new(TokioMutex::new(crate::engine::WimManager::new()));
    let app_clone = app.clone();

    // Emit initial status
    emit_status(&app, "starting");
    emit_log(&app, "=== BanaPE Build Started ===", "info");

    tauri::async_runtime::spawn(async move {
        let steps = builder::get_build_steps();
        let total_steps = steps.len();
        
        for (idx, step) in steps.iter().enumerate() {
            if cancelled.load(Ordering::Relaxed) {
                emit_log(&app_clone, "Build cancelled by user", "warning");
                emit_status(&app_clone, "cancelled");
                let _ = app_clone.emit(EVENT_BUILD_ERROR, json!({"message": "Cancelled"}));
                break;
            }

            // Emit step event
            emit_step(&app_clone, idx, &step.name, &step.desc);
            emit_log(&app_clone, &format!("[Step {}/{}] {}", idx + 1, total_steps, step.name), "info");

            // Execute step with progress updates
            match builder::execute_step(&app_clone, &config, &wim_manager, &cancelled, idx, total_steps).await {
                Ok(progress_data) => {
                    emit_log(&app_clone, &format!("✅ {} completed", step.name), "success");
                }
                Err(e) => {
                    emit_log(&app_clone, &format!("❌ {} failed: {}", step.name, e), "error");
                    emit_status(&app_clone, "error");
                    let _ = app_clone.emit(EVENT_BUILD_ERROR, json!({"message": e}));
                    break;
                }
            }
        }

        // Check if completed successfully
        if !cancelled.load(Ordering::Relaxed) {
            emit_progress(&app_clone, 100);
            emit_log(&app_clone, "✅ Build completed successfully!", "success");
            emit_status(&app_clone, "complete");
            let _ = app_clone.emit(EVENT_BUILD_COMPLETE, json!({"success": true}));
        }

        // Cleanup
        unsafe {
            if let Some(bs) = BUILD_STATE.take() {
                bs.is_building.store(false, Ordering::Relaxed);
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn build_cancel() -> Result<(), String> {
    unsafe {
        if let Some(ref bs) = BUILD_STATE {
            bs.cancelled.store(true, Ordering::Relaxed);
            Ok(())
        } else {
            Err("No build in progress".to_string())
        }
    }
}

#[tauri::command]
pub async fn build_cleanup(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    let mut wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    wim_manager.force_cleanup()?;
    Ok(())
}
