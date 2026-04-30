use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct NetworkPatch;

impl Patch for NetworkPatch {
    fn id(&self) -> &str { "network" }
    fn name(&self) -> &str { "网络支持" }
    fn category(&self) -> &str { "network" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "network.function_discovery".into(),
                opt_type: "boolean".into(),
                label: "功能发现".into(),
                default: Value::Bool(true),
                desc: Some("启用网络功能发现服务".into()),
                choices: None,
            },
            PatchOption {
                key: "network.builtin_drivers".into(),
                opt_type: "boolean".into(),
                label: "内置网络驱动".into(),
                default: Value::Bool(true),
                desc: Some("包含常用网络适配器驱动".into()),
                choices: None,
            },
            PatchOption {
                key: "network.full_functional".into(),
                opt_type: "boolean".into(),
                label: "全功能网络".into(),
                default: Value::Bool(true),
                desc: Some("启用完整网络功能 (DNS/DHCP/WinHTTP 等)".into()),
                choices: None,
            },
            PatchOption {
                key: "network.trayicon".into(),
                opt_type: "boolean".into(),
                label: "网络托盘图标".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "network.trayicon_lan".into(),
                opt_type: "boolean".into(),
                label: "LAN 托盘图标".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "network.trayicon_wlan".into(),
                opt_type: "boolean".into(),
                label: "WLAN 托盘图标".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        // Network services
        if ctx.get_option_bool("network.function_discovery") {
            ctx.reg_add(
                r"SYSTEM\ControlSet001\Services\fdPHost",
                Some("Start"),
                Some("REG_DWORD"),
                Some("3"),
            )?;
            ctx.reg_add(
                r"SYSTEM\ControlSet001\Services\FDResPub",
                Some("Start"),
                Some("REG_DWORD"),
                Some("3"),
            )?;
        }

        // Full network functionality
        if ctx.get_option_bool("network.full_functional") {
            let patterns: Vec<&str> = vec![
                "\\Windows\\System32\\drivers\\tcpip.sys",
                "\\Windows\\System32\\drivers\\afd.sys",
                "\\Windows\\System32\\netshell.dll",
                "\\Windows\\System32\\netman.dll",
            ];
            ctx.add_files_from_source(&patterns)?;

            // Enable network services
            ctx.reg_add(
                r"SYSTEM\ControlSet001\Services\Dnscache",
                Some("Start"),
                Some("REG_DWORD"),
                Some("3"),
            )?;
            ctx.reg_add(
                r"SYSTEM\ControlSet001\Services\Dhcp",
                Some("Start"),
                Some("REG_DWORD"),
                Some("3"),
            )?;
        }

        // Network tray icons
        if ctx.get_option_bool("network.trayicon") {
            ctx.reg_add(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                Some("HideSCAHealth"),
                Some("REG_DWORD"),
                Some("0"),
            )?;
        }

        Ok(())
    }
}
