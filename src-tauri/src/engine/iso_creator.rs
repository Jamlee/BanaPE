use crate::engine::tool_runner::{RunOptions, ToolRunner};
use std::path::Path;

/// ISO Creator - Create bootable ISO using oscdimg.exe
pub struct IsoCreator {
    runner: ToolRunner,
}

impl IsoCreator {
    pub fn new() -> Self {
        Self { runner: ToolRunner::new() }
    }

    pub fn create(&self, src_folder: &str, target_dir: &str, factory_dir: &str, iso_dir: &str) -> Result<String, String> {
        let iso_label = "BOOTPE";
        let iso_path = Path::new(factory_dir).join(format!("{}.iso", iso_label));

        // Prepare ISO directory structure
        let iso_sources = Path::new(iso_dir).join("sources");
        std::fs::create_dir_all(&iso_sources)
            .map_err(|e| format!("Failed to create ISO sources dir: {}", e))?;

        // Copy boot files from source
        let src_path = Path::new(src_folder);
        if src_path.exists() {
            for dir in &["boot", "efi"] {
                let src = src_path.join(dir);
                let dest = Path::new(iso_dir).join(dir);
                if src.exists() && !dest.exists() {
                    let _ = copy_dir_recursive(&src, &dest);
                }
            }

            // Copy bootmgr files
            for file in &["bootmgr", "bootmgr.efi"] {
                let src = src_path.join(file);
                let dest = Path::new(iso_dir).join(file);
                if src.exists() && !dest.exists() {
                    let _ = std::fs::copy(&src, &dest);
                }
            }
        }

        // Copy the built boot.wim to ISO sources
        let built_wim = Path::new(target_dir).join("build").join("boot.wim");
        let dest_wim = iso_sources.join("boot.wim");
        if built_wim.exists() {
            std::fs::copy(&built_wim, &dest_wim)
                .map_err(|e| format!("Failed to copy boot.wim: {}", e))?;
        } else {
            let fallback = Path::new(target_dir).join("boot.wim");
            if fallback.exists() {
                std::fs::copy(&fallback, &dest_wim)
                    .map_err(|e| format!("Failed to copy boot.wim: {}", e))?;
            } else {
                return Err("boot.wim not found in build output".to_string());
            }
        }

        // Build ISO using oscdimg
        let efi_bin = Path::new(iso_dir)
            .join("efi").join("Microsoft").join("boot").join("efisys.bin");

        if efi_bin.exists() {
            // UEFI + BIOS dual boot
            self.runner.run("oscdimg", &vec![
                "-bootdata:2".to_string(),
                format!("#p0,e,b\"{}\"", Path::new(iso_dir).join("boot").join("etfsboot.com").display()),
                format!("#pEF,e,b\"{}\"", efi_bin.display()),
                "-h".to_string(),
                format!("-l{}", iso_label),
                "-m".to_string(),
                "-u2".to_string(),
                "-udfver102".to_string(),
                iso_dir.to_string(),
                iso_path.to_string_lossy().into_owned(),
            ], RunOptions { admin: true, ignore_error: false })?;
        } else {
            // BIOS only
            self.runner.run("oscdimg", &vec![
                format!("-b\"{}\"", Path::new(iso_dir).join("boot").join("etfsboot.com").display()),
                "-h".to_string(),
                format!("-l{}", iso_label),
                "-m".to_string(),
                "-u2".to_string(),
                "-udfver102".to_string(),
                iso_dir.to_string(),
                iso_path.to_string_lossy().into_owned(),
            ], RunOptions { admin: true, ignore_error: false })?;
        }

        Ok(iso_path.to_string_lossy().to_string())
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            std::fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}
