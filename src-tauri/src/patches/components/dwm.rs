use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct DwmPatch;

impl Patch for DwmPatch {
    fn id(&self) -> &str { "dwm" }
    fn name(&self) -> &str { "DWM 桌面窗口管理" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.DWM".into(),
                opt_type: "boolean".into(),
                label: "启用 DWM".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.DWM") {
            return Ok(());
        }

        // Enable DWM composition
        ctx.reg_add(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            Some("EnableLUA"),
            Some("REG_DWORD"),
            Some("0"),
        )?;

        // Add DWM startup
        ctx.reg_add(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            Some("DWM"),
            Some("REG_SZ"),
            Some("dwm.exe"),
        )?;

        Ok(())
    }
}
