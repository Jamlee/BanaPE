use std::path::PathBuf;
use std::process::Command as StdCommand;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// ToolRunner - Execute external commands with progress tracking
pub struct ToolRunner {
    bin_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

impl ToolRunner {
    pub fn new() -> Self {
        let app_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            bin_dir: app_root.join("bin").join("x64"),
        }
    }

    pub fn with_bin_dir(bin_dir: PathBuf) -> Self {
        Self { bin_dir }
    }

    /// Run a command and wait for completion
    pub fn run(&self, tool: &str, args: &[String], options: RunOptions) -> Result<RunResult, String> {
        let cmd_path = self.resolve_tool(tool);
        let (final_cmd, final_args) = if options.admin {
            let nsudo = self.resolve_tool("NSudoC");
            let mut admin_args = vec![
                "-UseCurrentConsole".to_string(),
                "-Wait".to_string(),
                "-U:T".to_string(),
                cmd_path.to_string_lossy().to_string(),
            ];
            for a in args {
                admin_args.push(a.clone());
            }
            (nsudo, admin_args)
        } else {
            (cmd_path, args.to_vec())
        };

        let output = StdCommand::new(&final_cmd)
            .args(&final_args)
            .current_dir(&self.bin_dir)
            .env("PATH", format!("{};{}", self.bin_dir.display(), std::env::var("PATH").unwrap_or_default()))
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .map_err(|e| format!("Failed to start command: {} - {}", tool, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() && !options.ignore_error {
            Err(format!("Command failed: {} {}\n{}", tool, args.join(" "), stderr))
        } else {
            Ok(RunResult {
                stdout,
                stderr,
                code: output.status.code(),
            })
        }
    }

    /// Resolve tool path
    pub fn resolve_tool(&self, tool: &str) -> PathBuf {
        let system_tools = ["dism", "reg", "subst", "xcopy", "copy", "mkdir", "rmdir", "del", "icacls", "sc"];

        if system_tools.contains(&tool.to_lowercase().as_str()) {
            return PathBuf::from(format!("{}.exe", tool));
        }

        let tool_path = self.bin_dir.join(format!("{}.exe", tool));
        if tool_path.exists() {
            return tool_path;
        }

        PathBuf::from(tool)
    }
}

#[derive(Default)]
pub struct RunOptions {
    pub admin: bool,
    pub ignore_error: bool,
}
