use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct RuntimeKitsPatch;

impl Patch for RuntimeKitsPatch {
    fn id(&self) -> &str { "runtime-kits" }
    fn name(&self) -> &str { "运行时包" }
    fn category(&self) -> &str { "runtime-kits" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.runtimeKits".into(),
                opt_type: "boolean".into(),
                label: "启用运行时包".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.runtimeKits") {
            return Ok(());
        }

        // Add common runtime files
        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\ucrtbase.dll",
            "\\Windows\\System32\\api-ms-win-*.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
