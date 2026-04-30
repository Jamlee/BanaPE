use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Config Manager - Load and save configuration
pub struct ConfigManager {
    config_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub src_folder: String,
    #[serde(default = "default_boot_index")]
    pub boot_index: u32,
    #[serde(default)]
    pub adk_path: String,
    #[serde(default)]
    pub components: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub options: serde_json::Map<String, serde_json::Value>,
}

fn default_boot_index() -> u32 { 2 }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            src_folder: String::new(),
            boot_index: 2,
            adk_path: String::new(),
            components: serde_json::Map::new(),
            options: serde_json::Map::new(),
        }
    }
}

impl ConfigManager {
    pub fn new() -> Self {
        let app_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            config_path: app_root.join("config.json"),
        }
    }

    pub fn load(&self) -> AppConfig {
        match std::fs::read_to_string(&self.config_path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), String> {
        let data = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        std::fs::write(&self.config_path, data)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        Ok(())
    }
}
