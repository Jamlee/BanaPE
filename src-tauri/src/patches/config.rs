use serde::{Deserialize, Serialize};

/// 补丁配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchConfig {
    #[serde(rename = "patch")]
    pub patch: PatchInfo,
    
    #[serde(default)]
    pub files: FilesConfig,
    
    #[serde(default)]
    pub registry: RegistryConfig,
    
    #[serde(default)]
    pub commands: Vec<CommandConfig>,
}

/// 补丁元信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub order: u32,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

/// 文件配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FilesConfig {
    #[serde(default)]
    pub create_dirs: Vec<String>,
    
    #[serde(rename = "copy", default)]
    pub copy_files: Vec<FileCopy>,
}

/// 文件复制配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCopy {
    pub source: String,
    pub dest: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub recursive: bool,
}

/// 注册表配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RegistryConfig {
    #[serde(default)]
    pub load: Vec<RegistryLoad>,
    
    #[serde(rename = "add", default)]
    pub add_entries: Vec<RegistryAdd>,
    
    #[serde(default)]
    pub unload: Vec<String>,
}

/// 注册表加载
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryLoad {
    pub hive: String,
    pub file: String,
}

/// 注册表添加
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryAdd {
    pub hive: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub reg_type: String,
    pub data: String,
}

/// 命令配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    pub tool: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub admin: bool,
}

/// 补丁索引
#[derive(Debug, Deserialize)]
pub struct PatchIndex {
    pub version: String,
    pub description: String,
    pub categories: Vec<CategoryInfo>,
}

/// 分类信息
#[derive(Debug, Deserialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub patches: Vec<String>,
}
