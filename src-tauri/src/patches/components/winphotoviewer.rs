use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct WinPhotoViewerPatch;

impl Patch for WinPhotoViewerPatch {
    fn id(&self) -> &str { "winphotoviewer" }
    fn name(&self) -> &str { "照片查看器" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "component.winphotoviewer".into(),
                opt_type: "boolean".into(),
                label: "启用照片查看器".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        if !ctx.get_option_bool("component.winphotoviewer") {
            return Ok(());
        }

        let patterns: Vec<&str> = vec![
            "\\Windows\\System32\\shimgvw.dll",
            "\\Windows\\System32\\wuapi.dll",
        ];
        ctx.add_files_from_source(&patterns)?;

        // Register photo viewer
        ctx.reg_add(
            r"SOFTWARE\Classes\.jpg",
            None,
            None,
            Some("PhotoViewer.FileAssoc.Tiff"),
        )?;

        Ok(())
    }
}
