use crate::patches::{Patch, PatchOption, PatchChoice, context::PatchContext};
use serde_json::Value;

pub struct BuildConfigPatch;

impl Patch for BuildConfigPatch {
    fn id(&self) -> &str { "build-config" }
    fn name(&self) -> &str { "构建配置" }
    fn category(&self) -> &str { "core" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption { key: "build.source".into(), opt_type: "select".into(), label: "源模式".into(), default: Value::String("light".into()), desc: None, choices: Some(vec![
                PatchChoice { value: Value::String("light".into()), label: "精简 (light)".into() },
                PatchChoice { value: Value::String("full".into()), label: "完整 (full)".into() },
            ]) },
            PatchOption { key: "build.registry.software".into(), opt_type: "select".into(), label: "SOFTWARE 注册表".into(), default: Value::String("merge".into()), desc: None, choices: Some(vec![
                PatchChoice { value: Value::String("merge".into()), label: "合并".into() },
                PatchChoice { value: Value::String("full".into()), label: "完整".into() },
            ]) },
            PatchOption { key: "build.wow64support".into(), opt_type: "boolean".into(), label: "WoW64 支持".into(), default: Value::Bool(false), desc: None, choices: None },
            PatchOption { key: "system.high_compatibility".into(), opt_type: "boolean".into(), label: "高兼容性模式".into(), default: Value::Bool(true), desc: None, choices: None },
            PatchOption { key: "config.computername".into(), opt_type: "text".into(), label: "计算机名".into(), default: Value::String("WINXPE".into()), desc: None, choices: None },
            PatchOption { key: "system.workgroup".into(), opt_type: "text".into(), label: "工作组".into(), default: Value::String("WORKGROUP".into()), desc: None, choices: None },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        let reg_software = ctx.get_option_str("build.registry.software").unwrap_or_default();
        if reg_software == "full" {
            ctx.add_files_from_source(&["\\Windows\\System32\\config\\SOFTWARE"])?;
        }

        let computer_name = ctx.get_option_str("config.computername").unwrap_or_else(|| "WINXPE".into());
        ctx.reg_add(r"SYSTEM\ControlSet001\Control\ComputerName\ComputerName", Some("ComputerName"), Some("REG_SZ"), Some(&computer_name))?;
        ctx.reg_add(r"SYSTEM\ControlSet001\Services\Tcpip\Parameters", Some("Hostname"), Some("REG_SZ"), Some(&computer_name))?;
        ctx.reg_add(r"SYSTEM\ControlSet001\Services\Tcpip\Parameters", Some("NV Hostname"), Some("REG_SZ"), Some(&computer_name))?;

        let fbwf_cache = ctx.get_option_str("config.fbwf.cache").unwrap_or_else(|| "2048".into());
        ctx.reg_add(r"SYSTEM\ControlSet001\Services\FBWF", Some("WinPECacheThreshold"), Some("REG_DWORD"), Some(&fbwf_cache))?;

        Ok(())
    }
}
