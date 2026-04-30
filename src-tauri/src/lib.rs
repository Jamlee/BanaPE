mod engine;
mod patches;
mod commands;

use engine::WimManager;
use std::sync::Mutex;

pub struct AppState {
    pub wim_manager: Mutex<WimManager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            wim_manager: Mutex::new(WimManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::wim::wim_get_info,
            commands::wim::wim_mount,
            commands::wim::wim_unmount,
            commands::wim::wim_get_state,
            commands::wim::wim_cleanup_mount_points,
            commands::build::build_start,
            commands::build::build_cancel,
            commands::build::build_cleanup,
            commands::dialog::dialog_browse_folder,
            commands::dialog::dialog_browse_file,
            commands::config_cmd::config_load,
            commands::config_cmd::config_save,
            commands::patches::patches_get_list,
            commands::patches::patches_get_by_category,
            commands::patches::patches_get_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
