use tauri::State;
use crate::AppState;
use crate::engine::config::{AppConfig, ConfigManager};

#[tauri::command]
pub fn config_load(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config_manager = ConfigManager::new();
    Ok(config_manager.load())
}

#[tauri::command]
pub fn config_save(state: State<'_, AppState>, config: AppConfig) -> Result<(), String> {
    let config_manager = ConfigManager::new();
    config_manager.save(&config)
}
