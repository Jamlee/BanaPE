use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct DsmsvcPatch;

impl Patch for DsmsvcPatch {
    fn id(&self) -> &str { "dsmsvc" }
    fn name(&self) -> &str { "DSMSVC" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.dsmsvc".into(),
                opt_type: "boolean".into(),
                label: "启用 DSMSVC".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.dsmsvc") {
            return Ok(());
        }

        // DSM Service for deployment
        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\dsmsvc.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
