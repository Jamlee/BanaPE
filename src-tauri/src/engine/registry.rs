use crate::engine::tool_runner::{RunOptions, ToolRunner};

/// Registry Manager - Handles registry operations via reg.exe
pub struct Registry {
    runner: ToolRunner,
    loaded_hives: Vec<String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            runner: ToolRunner::new(),
            loaded_hives: Vec::new(),
        }
    }

    /// Load a registry hive
    pub fn load(&mut self, hive_key: &str, hive_file: &str) -> Result<(), String> {
        self.runner.run(
            "reg",
            &vec!["load".to_string(), format!("HKLM\\{}", hive_key), hive_file.to_string()],
            RunOptions { admin: true, ignore_error: false },
        )?;
        self.loaded_hives.push(hive_key.to_string());
        Ok(())
    }

    /// Unload a registry hive
    pub fn unload(&mut self, hive_key: &str) -> Result<(), String> {
        let result = self.runner.run(
            "reg",
            &vec!["unload".to_string(), format!("HKLM\\{}", hive_key)],
            RunOptions { admin: true, ignore_error: false },
        );

        match result {
            Ok(_) => {
                self.loaded_hives.retain(|h| h != hive_key);
                Ok(())
            }
            Err(_) => {
                // Retry after a delay
                std::thread::sleep(std::time::Duration::from_secs(1));
                self.runner.run(
                    "reg",
                    &vec!["unload".to_string(), format!("HKLM\\{}", hive_key)],
                    RunOptions { admin: true, ignore_error: false },
                )?;
                self.loaded_hives.retain(|h| h != hive_key);
                Ok(())
            }
        }
    }

    /// Unload all loaded hives
    pub fn unload_all(&mut self) {
        let hives: Vec<String> = self.loaded_hives.clone();
        for hive in hives {
            let _ = self.unload(&hive);
        }
    }

    /// Load standard PE registry hives
    pub fn load_pe_hives(&mut self, mount_dir: &str, src_dir: Option<&str>) -> Result<(), String> {
        let hive_files = vec![
            ("Tmp_DEFAULT", format!("{}\\Windows\\System32\\config\\DEFAULT", mount_dir)),
            ("Tmp_SOFTWARE", format!("{}\\Windows\\System32\\config\\SOFTWARE", mount_dir)),
            ("Tmp_SYSTEM", format!("{}\\Windows\\System32\\config\\SYSTEM", mount_dir)),
            ("Tmp_DRIVERS", format!("{}\\Windows\\System32\\config\\DRIVERS", mount_dir)),
        ];

        for (key, path) in &hive_files {
            if std::path::Path::new(path).exists() {
                let _ = self.load(key, path);
            }
        }

        if let Some(src) = src_dir {
            let src_hives = vec![
                ("Src_DEFAULT", format!("{}\\Windows\\System32\\config\\DEFAULT", src)),
                ("Src_SOFTWARE", format!("{}\\Windows\\System32\\config\\SOFTWARE", src)),
                ("Src_SYSTEM", format!("{}\\Windows\\System32\\config\\SYSTEM", src)),
                ("Src_DRIVERS", format!("{}\\Windows\\System32\\config\\DRIVERS", src)),
            ];

            for (key, path) in &src_hives {
                if std::path::Path::new(path).exists() {
                    let _ = self.load(key, path);
                }
            }
        }

        Ok(())
    }

    /// Unload all PE registry hives
    pub fn unload_pe_hives(&mut self) {
        self.unload_all();
    }

    /// Add or modify a registry value
    pub fn add(&self, key: &str, name: Option<&str>, reg_type: Option<&str>, value: Option<&str>) -> Result<(), String> {
        let mut args = vec!["add".to_string(), key.to_string(), "/f".to_string()];
        if let Some(n) = name {
            args.push("/v".to_string());
            args.push(n.to_string());
        }
        if let Some(t) = reg_type {
            args.push("/t".to_string());
            args.push(t.to_string());
        }
        if let Some(v) = value {
            args.push("/d".to_string());
            args.push(v.to_string());
        }

        self.runner.run("reg", &args, RunOptions { admin: true, ignore_error: false })?;
        Ok(())
    }

    /// Delete a registry key or value
    pub fn delete(&self, key: &str, name: Option<&str>) -> Result<(), String> {
        let args = match name {
            Some(n) => vec!["delete".to_string(), key.to_string(), "/v".to_string(), n.to_string(), "/f".to_string()],
            None => vec!["delete".to_string(), key.to_string(), "/f".to_string()],
        };
        self.runner.run("reg", &args, RunOptions { admin: true, ignore_error: true })?;
        Ok(())
    }

    /// Query a registry key
    pub fn query(&self, key: &str, name: Option<&str>) -> Result<String, String> {
        let args = match name {
            Some(n) => vec!["query".to_string(), key.to_string(), "/v".to_string(), n.to_string()],
            None => vec!["query".to_string(), key.to_string()],
        };
        let result = self.runner.run("reg", &args, RunOptions { admin: true, ignore_error: true })?;
        Ok(result.stdout)
    }

    /// Copy a registry key from source to target
    pub fn copy(&self, src_key: &str, dest_key: &str) -> Result<(), String> {
        self.runner.run(
            "reg",
            &vec!["copy".to_string(), src_key.to_string(), dest_key.to_string(), "/f".to_string(), "/s".to_string()],
            RunOptions { admin: true, ignore_error: true },
        )?;
        Ok(())
    }

    /// Copy registry key from source (Src_) to target (Tmp_)
    /// Equivalent to RegCopy macro in WimBuilder2
    pub fn reg_copy(&self, key_path: &str) -> Result<(), String> {
        let clean_path = key_path.strip_prefix("HKLM\\").unwrap_or(key_path);
        let src_key = format!("HKLM\\Src_{}", clean_path);
        let tmp_key = format!("HKLM\\Tmp_{}", clean_path);
        self.copy(&src_key, &tmp_key)
    }

    /// Add registry value to Tmp_ hive (convenience method)
    pub fn reg_add(&self, key_path: &str, name: Option<&str>, reg_type: Option<&str>, value: Option<&str>) -> Result<(), String> {
        let mut clean_path = key_path.strip_prefix("HKLM\\").unwrap_or(key_path).to_string();
        if !clean_path.starts_with("Tmp_") && !clean_path.starts_with("Src_") {
            clean_path = format!("Tmp_{}", clean_path);
        }
        let full_key = format!("HKLM\\{}", clean_path);
        self.add(&full_key, name, reg_type, value)
    }

    /// Import a .reg file
    pub fn import(&self, reg_file: &str) -> Result<(), String> {
        self.runner.run("reg", &vec!["import".to_string(), reg_file.to_string()], RunOptions { admin: true, ignore_error: false })?;
        Ok(())
    }
}
