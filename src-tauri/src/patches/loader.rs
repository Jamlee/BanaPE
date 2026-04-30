use std::fs;
use std::path::{Path, PathBuf};
use crate::patches::config::PatchConfig;

/// 补丁加载器
pub struct PatchLoader {
    base_path: PathBuf,
}

impl PatchLoader {
    /// 创建新的补丁加载器
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }
    
    /// 加载所有补丁配置
    pub fn load_all_patches(&self) -> Result<Vec<PatchConfig>, String> {
        let mut patches = Vec::new();
        
        // 遍历所有分类目录
        if !self.base_path.exists() {
            return Err(format!("补丁目录不存在: {:?}", self.base_path));
        }
        
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| format!("读取补丁目录失败: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let path = entry.path();
            
            // 只处理目录
            if path.is_dir() {
                let category_patches = self.load_patches_from_category(&path)?;
                patches.extend(category_patches);
            }
        }
        
        // 按 order 排序
        patches.sort_by_key(|p| p.patch.order);
        
        Ok(patches)
    }
    
    /// 从分类目录加载补丁
    fn load_patches_from_category(&self, dir: &Path) -> Result<Vec<PatchConfig>, String> {
        let mut patches = Vec::new();
        
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("读取分类目录 {:?} 失败: {}", dir, e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
            let path = entry.path();
            
            // 只处理 .toml 文件
            if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
                match self.load_patch_config(&path) {
                    Ok(config) => patches.push(config),
                    Err(e) => eprintln!("警告: 加载补丁 {:?} 失败: {}", path, e),
                }
            }
        }
        
        Ok(patches)
    }
    
    /// 加载单个补丁配置
    pub fn load_patch_config(&self, path: &Path) -> Result<PatchConfig, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取文件 {:?} 失败: {}", path, e))?;
        
        let config: PatchConfig = toml::from_str(&content)
            .map_err(|e| format!("解析 TOML {:?} 失败: {}", path, e))?;
        
        Ok(config)
    }
    
    /// 根据 ID 加载特定补丁
    pub fn load_patch_by_id(&self, id: &str) -> Result<PatchConfig, String> {
        let all_patches = self.load_all_patches()?;
        
        all_patches
            .into_iter()
            .find(|p| p.patch.id == id)
            .ok_or_else(|| format!("未找到补丁: {}", id))
    }
    
    /// 获取所有补丁 ID 列表
    pub fn get_patch_ids(&self) -> Result<Vec<String>, String> {
        let patches = self.load_all_patches()?;
        Ok(patches.into_iter().map(|p| p.patch.id).collect())
    }
    
    /// 按分类获取补丁
    pub fn get_patches_by_category(&self, category: &str) -> Result<Vec<PatchConfig>, String> {
        let patches = self.load_all_patches()?;
        Ok(patches
            .into_iter()
            .filter(|p| p.patch.category == category)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_patch_config() {
        let loader = PatchLoader::new("patches");
        let result = loader.load_all_patches();
        assert!(result.is_ok());
        
        let patches = result.unwrap();
        assert!(!patches.is_empty());
        
        // 检查排序
        for i in 1..patches.len() {
            assert!(patches[i].patch.order >= patches[i - 1].patch.order);
        }
    }
}
