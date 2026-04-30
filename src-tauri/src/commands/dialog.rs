use tauri::AppHandle;

#[tauri::command]
pub async fn dialog_browse_folder(app: AppHandle) -> Result<Option<String>, String> {
    let folder = rfd::AsyncFileDialog::new()
        .pick_folder()
        .await;
    
    Ok(folder.map(|f| f.path().to_string_lossy().to_string()))
}

#[tauri::command]
pub async fn dialog_browse_file(
    app: AppHandle,
    filters: Option<Vec<(String, Vec<String>)>>,
) -> Result<Option<String>, String> {
    let mut dialog = rfd::AsyncFileDialog::new();
    
    if let Some(filter_list) = filters {
        for (name, extensions) in filter_list {
            dialog = dialog.add_filter(&name, &extensions);
        }
    }
    
    let file = dialog.pick_file().await;
    
    Ok(file.map(|f| f.path().to_string_lossy().to_string()))
}
