use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct NotepadPatch;

impl Patch for NotepadPatch {
    fn id(&self) -> &str { "notepad" }
    fn name(&self) -> &str { "记事本" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.notepad".into(),
                opt_type: "boolean".into(),
                label: "启用记事本".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.notepad") {
            return Ok(());
        }

        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\notepad.exe",
            "\\Windows\\System32\\zh-CN\\notepad.exe.mui",
        ];
        ctx.add_files_from_source(&patterns)?;

        Ok(())
    }
}
