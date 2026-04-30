use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct MsiPatch;

impl Patch for MsiPatch {
    fn id(&self) -> &str { "msi" }
    fn name(&self) -> &str { "MSI 安装器" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.MSI".into(),
                opt_type: "boolean".into(),
                label: "启用 MSI 安装器".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.MSI") {
            return Ok(());
        }

        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\msiexec.exe",
            "\\Windows\\System32\\msi.dll",
            "\\Windows\\System32\\msihnd.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        // Enable MSI service
        ctx.reg_add(
            r"SYSTEM\ControlSet001\Services\msiserver",
            Some("Start"),
            Some("REG_DWORD"),
            Some("3"),
        )?;

        Ok(())
    }
}
