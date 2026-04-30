use crate::engine::file_extractor::FileExtractor;
use crate::engine::registry::Registry;
use crate::engine::tool_runner::{RunOptions, ToolRunner};
use serde_json::Map;

/// PatchContext - Context passed to each patch during apply()
pub struct PatchContext<'a> {
    pub src_wim: String,
    pub src_index: u32,
    pub mount_dir: String,
    pub x_drive: String,
    pub app_root: String,
    pub factory_dir: String,
    pub tmp_dir: String,
    pub pe_build: u32,
    pub pe_arch: String,
    pub pe_lang: String,
    pub options: Map<String, serde_json::Value>,
    pub components: Map<String, serde_json::Value>,
    pub registry: &'a Registry,
    pub extractor: &'a FileExtractor,
    pub runner: &'a ToolRunner,
}

impl<'a> PatchContext<'a> {
    /// Get an option value
    pub fn get_option(&self, key: &str) -> Option<&serde_json::Value> {
        self.options.get(key)
    }

    /// Get option as string
    pub fn get_option_str(&self, key: &str) -> Option<String> {
        self.options.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
    }

    /// Get option as bool
    pub fn get_option_bool(&self, key: &str) -> bool {
        self.options.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
    }

    /// Get option as u64
    pub fn get_option_u64(&self, key: &str) -> Option<u64> {
        self.options.get(key).and_then(|v| v.as_u64())
    }

    /// Extract files from source WIM
    pub fn add_files_from_source(&self, patterns: &[&str]) -> Result<(), String> {
        self.extractor.extract_from_wim(&self.src_wim, self.src_index, patterns, &self.mount_dir)
    }

    /// Registry: copy from Src_ to Tmp_
    pub fn reg_copy(&self, key_path: &str) -> Result<(), String> {
        self.registry.reg_copy(key_path)
    }

    /// Registry: add value to Tmp_ hive
    pub fn reg_add(&self, key_path: &str, name: Option<&str>, reg_type: Option<&str>, value: Option<&str>) -> Result<(), String> {
        self.registry.reg_add(key_path, name, reg_type, value)
    }

    /// Run an external command with admin privileges
    pub fn run(&self, cmd: &str, args: &[&str]) -> Result<(), String> {
        let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.runner.run(cmd, &args_str, RunOptions { admin: true, ignore_error: false })?;
        Ok(())
    }

    /// Run an external command (no admin)
    pub fn run_no_admin(&self, cmd: &str, args: &[&str]) -> Result<(), String> {
        let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        self.runner.run(cmd, &args_str, RunOptions::default())?;
        Ok(())
    }

    /// Copy a file to the mount directory
    pub fn add_file(&self, src: &str, dest: &str) -> Result<(), String> {
        let dest_path = std::path::Path::new(&self.mount_dir).join(dest.trim_start_matches('\\'));
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::copy(src, &dest_path).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Copy vendor directory to mount
    pub fn copy_vendor(&self, vendor_name: &str, dest_relative: Option<&str>) -> Result<(), String> {
        let vendor_dir = std::path::Path::new(&self.app_root).join("vendor").join(vendor_name);
        let dest = match dest_relative {
            Some(d) => std::path::Path::new(&self.mount_dir).join(d),
            None => std::path::Path::new(&self.mount_dir).join("Program Files").join(vendor_name),
        };
        if vendor_dir.exists() {
            copy_dir_recursive(&vendor_dir, &dest)?;
        }
        Ok(())
    }

    /// Write file to mount directory
    pub fn write_file(&self, relative_path: &str, content: &str) -> Result<(), String> {
        let path = std::path::Path::new(&self.mount_dir).join(relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Create directory in mount
    pub fn create_dir(&self, relative_path: &str) -> Result<(), String> {
        let path = std::path::Path::new(&self.mount_dir).join(relative_path);
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())
    }
}

fn copy_dir_recursive(src: &std::path::Path, dest: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    if let Ok(entries) = std::fs::read_dir(src) {
        for entry in entries.flatten() {
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());
            if src_path.is_dir() {
                copy_dir_recursive(&src_path, &dest_path)?;
            } else {
                std::fs::copy(&src_path, &dest_path).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}
