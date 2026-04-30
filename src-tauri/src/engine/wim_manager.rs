use crate::engine::tool_runner::{RunOptions, ToolRunner};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WimImageInfo {
    pub image_count: Option<u32>,
    pub images: Vec<WimImage>,
    pub version: Option<String>,
    pub arch: Option<String>,
    pub lang: Option<String>,
    pub build: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WimImage {
    pub index: Option<u32>,
    pub name: Option<String>,
    pub arch: Option<String>,
    pub build: Option<String>,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountState {
    pub mounted: bool,
    pub mount_dir: Option<String>,
    pub wim_path: Option<String>,
    pub index: Option<u32>,
}

pub struct WimManager {
    runner: ToolRunner,
    pub mount_state: MountState,
}

impl WimManager {
    pub fn new() -> Self {
        Self {
            runner: ToolRunner::new(),
            mount_state: MountState {
                mounted: false,
                mount_dir: None,
                wim_path: None,
                index: None,
            },
        }
    }

    /// Get WIM image information
    pub fn get_info(&self, wim_path: &str) -> Result<WimImageInfo, String> {
        // Try wimlib-imagex first
        let args1: Vec<String> = vec!["info".to_string(), wim_path.to_string()];
        match self.runner.run("wimlib-imagex", &args1, RunOptions::default()) {
            Ok(result) => Ok(self.parse_wim_info(&result.stdout)),
            Err(_) => {
                // Fallback to DISM
                let wim_file_arg = format!("/WimFile:{}", wim_path);
                let args2: Vec<String> = vec!["/Get-WimInfo".to_string(), wim_file_arg, "/English".to_string()];
                match self.runner.run(
                    "dism",
                    &args2,
                    RunOptions { admin: true, ignore_error: false },
                ) {
                    Ok(result) => Ok(self.parse_dism_info(&result.stdout)),
                    Err(e) => Err(format!("Failed to get WIM info: {}", e)),
                }
            }
        }
    }

    /// Mount a WIM image
    pub fn mount(&mut self, wim_path: &str, index: u32, mount_dir: &str) -> Result<(), String> {
        if self.mount_state.mounted {
            return Err("WIM is already mounted. Please unmount first.".to_string());
        }

        let mount_path = PathBuf::from(mount_dir);
        if !mount_path.exists() {
            std::fs::create_dir_all(&mount_path)
                .map_err(|e| format!("Failed to create mount dir: {}", e))?;
        }

        let mount_dir_str = mount_dir.to_string();
        let wim_file_arg = format!("/WimFile:{}", wim_path);
        let index_arg = format!("/Index:{}", index);
        let mount_dir_arg = format!("/MountDir:{}", mount_dir_str);
        let args: Vec<String> = vec![
            "/Mount-Wim".to_string(),
            wim_file_arg,
            index_arg,
            mount_dir_arg,
        ];
        self.runner.run(
            "dism",
            &args,
            RunOptions { admin: true, ignore_error: false },
        )?;

        self.mount_state = MountState {
            mounted: true,
            mount_dir: Some(mount_dir.to_string()),
            wim_path: Some(wim_path.to_string()),
            index: Some(index),
        };

        Ok(())
    }

    /// Unmount a WIM image
    pub fn unmount(&mut self, mount_dir: &str, commit: bool) -> Result<(), String> {
        let mount_dir_str = mount_dir.to_string();
        let mount_dir_arg = format!("/MountDir:{}", mount_dir_str);
        let commit_arg = if commit { "/Commit" } else { "/Discard" };
        let args: Vec<String> = vec!["/Unmount-Wim".to_string(), mount_dir_arg, commit_arg.to_string()];

        self.runner.run("dism", &args, RunOptions { admin: true, ignore_error: false })?;

        self.mount_state = MountState {
            mounted: false,
            mount_dir: None,
            wim_path: None,
            index: None,
        };

        Ok(())
    }

    /// Get current mount state
    pub fn get_state(&self) -> &MountState {
        &self.mount_state
    }

    /// Cleanup mount points
    pub fn cleanup_mount_points(&self) -> Result<(), String> {
        let args: Vec<String> = vec!["/Cleanup-Mountpoints".to_string()];
        self.runner.run("dism", &args, RunOptions { admin: true, ignore_error: false })?;
        Ok(())
    }

    /// Force cleanup - unmount all and cleanup
    pub fn force_cleanup(&mut self) -> Result<(), String> {
        if let Some(mount_dir) = self.mount_state.mount_dir.clone() {
            let _ = self.unmount(&mount_dir, false);
        }

        // Try DISM cleanup as fallback
        let _ = self.cleanup_mount_points();

        // Try to delete SUBST drive
        let subst_args: Vec<String> = vec!["X:".to_string(), "/D".to_string()];
        let _ = self.runner.run("subst", &subst_args, RunOptions { admin: false, ignore_error: true });

        self.mount_state = MountState {
            mounted: false,
            mount_dir: None,
            wim_path: None,
            index: None,
        };

        Ok(())
    }

    /// Export WIM image (optimization)
    pub fn export(&self, src_wim: &str, dest_wim: &str, index: u32) -> Result<(), String> {
        let index_str = index.to_string();
        let args1: Vec<String> = vec!["export".to_string(), src_wim.to_string(), index_str, dest_wim.to_string()];
        match self.runner.run(
            "wimlib-imagex",
            &args1,
            RunOptions { admin: true, ignore_error: false },
        ) {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fallback to DISM
                let src_arg = format!("/SourceImageFile:{}", src_wim);
                let src_idx_arg = format!("/SourceIndex:{}", index);
                let dest_arg = format!("/DestinationImageFile:{}", dest_wim);
                let args2: Vec<String> = vec![
                    "/Export-Image".to_string(),
                    src_arg,
                    src_idx_arg,
                    dest_arg,
                ];
                self.runner.run(
                    "dism",
                    &args2,
                    RunOptions { admin: true, ignore_error: false },
                )?;
                Ok(())
            }
        }
    }

    fn parse_wim_info(&self, stdout: &str) -> WimImageInfo {
        let mut info = WimImageInfo {
            image_count: None,
            images: Vec::new(),
            version: None,
            arch: None,
            lang: None,
            build: None,
            name: None,
        };
        let mut current_image: Option<WimImage> = None;

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Image Count:") {
                info.image_count = trimmed.split(':').nth(1).and_then(|s| s.trim().parse().ok());
            } else if trimmed.starts_with("Image ") && trimmed.ends_with(':') {
                if let Some(img) = current_image.take() {
                    info.images.push(img);
                }
                current_image = Some(WimImage {
                    index: None,
                    name: None,
                    arch: None,
                    build: None,
                    lang: None,
                });
            } else if let Some(ref mut img) = current_image {
                if trimmed.starts_with("Name:") {
                    img.name = trimmed.split(':').nth(1).map(|s| s.trim().to_string());
                } else if trimmed.starts_with("Architecture:") {
                    img.arch = trimmed.split(':').nth(1).map(|s| s.trim().to_string());
                } else if trimmed.starts_with("Build:") {
                    img.build = trimmed.split(':').nth(1).map(|s| s.trim().to_string());
                } else if trimmed.starts_with("Languages:") {
                    img.lang = trimmed.split(':').nth(1).map(|s| s.trim().split(',').next().map(|s| s.trim().to_string())).flatten();
                }
            }
        }

        if let Some(img) = current_image.take() {
            info.images.push(img);
        }

        // Fill convenience fields from first image
        if let Some(img) = info.images.first() {
            info.version = img.build.as_ref().map(|b| format!("10.0.{}", b));
            info.arch = img.arch.clone();
            info.lang = img.lang.clone();
            info.build = img.build.clone();
            info.name = img.name.clone();
        }

        info
    }

    fn parse_dism_info(&self, stdout: &str) -> WimImageInfo {
        let mut info = WimImageInfo {
            image_count: None,
            images: Vec::new(),
            version: None,
            arch: None,
            lang: None,
            build: None,
            name: None,
        };

        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Architecture") {
                info.arch = trimmed.split(':').nth(1).map(|s| s.trim().to_string());
            } else if trimmed.starts_with("Version") {
                info.version = trimmed.split(':').nth(1).map(|s| s.trim().to_string());
            } else if trimmed.starts_with("Languages") {
                info.lang = trimmed.split(':').nth(1)
                    .map(|s| s.trim().split(';').next().map(|s| s.trim().to_string())).flatten();
            }
        }

        info
    }
}
