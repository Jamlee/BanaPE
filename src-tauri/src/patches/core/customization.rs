use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct CustomizationPatch;
impl Patch for CustomizationPatch {
    fn id(&self) -> &str { "customization" }
    fn name(&self) -> &str { "主题定制" }
    fn category(&self) -> &str { "core" }
    fn options(&self) -> Vec<PatchOption> {
        vec![PatchOption { key: "tweak.shortcut.noarrow".into(), opt_type: "boolean".into(), label: "移除快捷方式箭头".into(), default: Value::Bool(false), desc: None, choices: None }]
    }
    fn apply(&self, _ctx: &PatchContext) -> Result<(), String> { Ok(()) }
}
