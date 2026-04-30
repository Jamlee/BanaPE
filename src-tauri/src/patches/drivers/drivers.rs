use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct DriversPatch;

impl Patch for DriversPatch {
    fn id(&self) -> &str { "drivers" }
    fn name(&self) -> &str { "驱动支持" }
    fn category(&self) -> &str { "drivers" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.drivers".into(),
                opt_type: "boolean".into(),
                label: "启用驱动支持".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.drivers") {
            return Ok(());
        }

        // Add driver store and PnP support
        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\drivers",
            "\\Windows\\System32\\DriverStore",
            "\\Windows\\System32\\drvstore.dll",
            "\\Windows\\System32\\newdev.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        // Enable PnP service
        ctx.reg_add(
            r"SYSTEM\ControlSet001\Services\PlugPlay",
            Some("Start"),
            Some("REG_DWORD"),
            Some("0"),
        )?;

        Ok(())
    }
}
