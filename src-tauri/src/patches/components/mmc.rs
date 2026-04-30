use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct MmcPatch;

impl Patch for MmcPatch {
    fn id(&self) -> &str { "mmc" }
    fn name(&self) -> &str { "MMC 管理控制台" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.MMC".into(),
                opt_type: "boolean".into(),
                label: "启用 MMC".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.MMC") {
            return Ok(());
        }

        // Add MMC files from source
        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\mmc.exe",
            "\\Windows\\System32\\mmcbase.dll",
            "\\Windows\\System32\\mmcndmgr.dll",
            "\\Windows\\System32\\mmcshext.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
