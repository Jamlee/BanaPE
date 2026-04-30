use tauri::State;
use crate::AppState;
use crate::engine::wim_manager::{WimImageInfo, MountState};

#[tauri::command]
pub fn wim_get_info(state: State<'_, AppState>, wim_path: String) -> Result<WimImageInfo, String> {
    let wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    wim_manager.get_info(&wim_path)
}

#[tauri::command]
pub fn wim_mount(state: State<'_, AppState>, wim_path: String, index: u32, mount_dir: String) -> Result<(), String> {
    let mut wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    wim_manager.mount(&wim_path, index, &mount_dir)
}

#[tauri::command]
pub fn wim_unmount(state: State<'_, AppState>, mount_dir: String, commit: bool) -> Result<(), String> {
    let mut wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    wim_manager.unmount(&mount_dir, commit)
}

#[tauri::command]
pub fn wim_get_state(state: State<'_, AppState>) -> Result<MountState, String> {
    let wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    Ok(wim_manager.get_state().clone())
}

#[tauri::command]
pub fn wim_cleanup_mount_points(state: State<'_, AppState>) -> Result<(), String> {
    let wim_manager = state.wim_manager.lock().map_err(|e| e.to_string())?;
    wim_manager.cleanup_mount_points()
}
