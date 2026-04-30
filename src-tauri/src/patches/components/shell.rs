use crate::patches::{Patch, PatchOption, PatchChoice, context::PatchContext};
use serde_json::Value;

pub struct ShellPatch;

impl Patch for ShellPatch {
    fn id(&self) -> &str { "shell" }
    fn name(&self) -> &str { "Shell (Explorer + WinXShell)" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "shell.app".into(),
                opt_type: "select".into(),
                label: "Shell 类型".into(),
                default: Value::String("winxshell".into()),
                desc: None,
                choices: Some(vec![
                    PatchChoice { value: Value::String("explorer".into()), label: "Explorer".into() },
                    PatchChoice { value: Value::String("winxshell".into()), label: "WinXShell".into() },
                ]),
            },
            PatchOption {
                key: "shell.wallpaper".into(),
                opt_type: "text".into(),
                label: "壁纸路径".into(),
                default: Value::String("wallpaper.jpg".into()),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.fix.showDesktop".into(),
                opt_type: "boolean".into(),
                label: "显示桌面按钮".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.ui.resolution".into(),
                opt_type: "boolean".into(),
                label: "分辨率设置".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.clockarea.event".into(),
                opt_type: "boolean".into(),
                label: "时钟区域事件".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.system.property".into(),
                opt_type: "boolean".into(),
                label: "系统属性".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.shortcut.ocf".into(),
                opt_type: "boolean".into(),
                label: "打开所在文件夹".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "shell.fix.fullscreen".into(),
                opt_type: "boolean".into(),
                label: "全屏自动隐藏任务栏".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        let shell_app = ctx.get_option_str("shell.app").unwrap_or_else(|| "winxshell".into());
        
        // Configure shell type
        if shell_app == "explorer" {
            ctx.reg_add(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
                Some("Shell"),
                Some("REG_SZ"),
                Some("explorer.exe"),
            )?;
        } else {
            // WinXShell
            ctx.reg_add(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
                Some("Shell"),
                Some("REG_SZ"),
                Some("WinXShell.exe"),
            )?;
        }

        // Show desktop button
        if ctx.get_option_bool("shell.fix.showDesktop") {
            ctx.reg_add(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                Some("TaskbarShowDesktop"),
                Some("REG_DWORD"),
                Some("1"),
            )?;
        }

        Ok(())
    }
}
