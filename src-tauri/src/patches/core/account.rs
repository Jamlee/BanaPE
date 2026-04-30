use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct AccountPatch;

impl Patch for AccountPatch {
    fn id(&self) -> &str { "account" }
    fn name(&self) -> &str { "账户设置" }
    fn category(&self) -> &str { "core" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption { key: "account.admin_enabled".into(), opt_type: "boolean".into(), label: "启用管理员账户".into(), default: Value::Bool(false), desc: None, choices: None },
            PatchOption { key: "account.admin_password".into(), opt_type: "text".into(), label: "管理员密码".into(), default: Value::String("123456".into()), desc: None, choices: None },
            PatchOption { key: "account.admin_autologon".into(), opt_type: "boolean".into(), label: "自动登录".into(), default: Value::Bool(true), desc: None, choices: None },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        ctx.reg_add(r"DEFAULT\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", Some("TaskbarGlomLevel"), Some("REG_DWORD"), Some("2"))?;
        Ok(())
    }
}
