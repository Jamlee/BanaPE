use tauri::{State, AppHandle};
use crate::AppState;
use crate::engine::config::AppConfig;
use crate::engine::builder;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex as TokioMutex;

struct BuildState {
    cancelled: Arc<AtomicBool>,
    is_building: Arc<AtomicBool>,
}

// Global build state
static mut BUILD_STATE: Option<BuildState> = None;

#[tauri::command]
pub async fn build_start(
    app: AppHandle,
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    // Initialize build state
    unsafe {
        if BUILD_STATE.is_some() {
            return Err("构建已在进行中".to_string());
        }
        BUILD_STATE = Some(BuildState {
            cancelled: Arc::new(AtomicBool::new(false)),
            is_building: Arc::new(AtomicBool::new(true)),
        });
    }

    let build_state = unsafe { BUILD_STATE.as_ref().unwrap() };
    let cancelled = build_state.cancelled.clone();
    
    // Create a new WimManager for the build
    let wim_manager = Arc::new(TokioMutex::new(crate::engine::WimManager::new()));

    // Run build in background
    tauri::async_runtime::spawn(async move {
        let result = builder::execute_build(app, config, &wim_manager, cancelled).await;
        
        // Cleanup build state
        unsafe {
            if let Some(bs) = BUILD_STATE.take() {
                bs.is_building.store(false, Ordering::Relaxed);
            }
        }

        match result {
            Ok(_) => {
                println!("Build completed successfully");
            }
            Err(e) => {
                eprintln!("Build failed: {}", e);
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
            Err("没有正在进行的构建".to_string())
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
