use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct VcRuntimePatch;

impl Patch for VcRuntimePatch {
    fn id(&self) -> &str { "vcruntime" }
    fn name(&self) -> &str { "VC++ 运行时" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.vcruntime".into(),
                opt_type: "boolean".into(),
                label: "启用 VC++ 运行时".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.vcruntime") {
            return Ok(());
        }

        // Add VC++ runtime files from vendor or source
        // This would typically copy vcredist files
        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\msvcp140.dll",
            "\\Windows\\System32\\vcruntime140.dll",
            "\\Windows\\System32\\vcruntime140_1.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
