use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct MspaintPatch;

impl Patch for MspaintPatch {
    fn id(&self) -> &str { "mspaint" }
    fn name(&self) -> &str { "画图" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.mspaint".into(),
                opt_type: "boolean".into(),
                label: "启用画图".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.mspaint") {
            return Ok(());
        }

        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\mspaint.exe",
            "\\Windows\\System32\\zh-CN\\mspaint.exe.mui",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
