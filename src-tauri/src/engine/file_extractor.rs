use crate::engine::tool_runner::{RunOptions, ToolRunner};

/// File Extractor - Extract files from source WIM using wimlib-imagex
pub struct FileExtractor {
    runner: ToolRunner,
}

impl FileExtractor {
    pub fn new() -> Self {
        Self { runner: ToolRunner::new() }
    }

    /// Extract files from source WIM to destination
    pub fn extract_from_wim(&self, src_wim: &str, src_index: u32, patterns: &[&str], dest_dir: &str) -> Result<(), String> {
        let dest_path = std::path::Path::new(dest_dir);
        if !dest_path.exists() {
            std::fs::create_dir_all(dest_path)
                .map_err(|e| format!("Failed to create dest dir: {}", e))?;
        }

        let mut args = vec!["extract".to_string(), src_wim.to_string(), format!("{}", src_index)];
        for pattern in patterns {
            args.push(pattern.replace('/', "\\"));
        }
        args.push(format!("--dest-dir={}", dest_dir));
        args.push("--no-acls".to_string());

        self.runner.run("wimlib-imagex", &args, RunOptions { admin: true, ignore_error: false })?;
        Ok(())
    }

    /// Extract registry files from source WIM
    pub fn extract_registry_files(&self, src_wim: &str, src_index: u32, dest_dir: &str) -> Result<(), String> {
        let reg_files = vec![
            "\\Windows\\System32\\config\\SOFTWARE",
            "\\Windows\\System32\\config\\SYSTEM",
            "\\Windows\\System32\\config\\DEFAULT",
            "\\Windows\\System32\\config\\DRIVERS",
        ];

        let _ = self.extract_from_wim(src_wim, src_index, &reg_files, dest_dir);
        Ok(())
    }
}
