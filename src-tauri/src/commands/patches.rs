use crate::patches::{PatchConfig, PatchLoader};

#[tauri::command]
pub fn patches_get_list() -> Result<Vec<PatchConfig>, String> {
    // 从配置文件加载补丁
    let loader = PatchLoader::new("patches");
    loader.load_all_patches()
}

#[tauri::command]
pub fn patches_get_by_category(category: String) -> Result<Vec<PatchConfig>, String> {
    let loader = PatchLoader::new("patches");
    loader.get_patches_by_category(&category)
}

#[tauri::command]
pub fn patches_get_by_id(id: String) -> Result<PatchConfig, String> {
    let loader = PatchLoader::new("patches");
    loader.load_patch_by_id(&id)
}
