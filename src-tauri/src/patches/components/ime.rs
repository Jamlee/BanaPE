use crate::patches::{Patch, PatchOption, context::PatchContext};
use serde_json::Value;

pub struct ImePatch;

impl Patch for ImePatch {
    fn id(&self) -> &str { "ime" }
    fn name(&self) -> &str { "输入法" }
    fn category(&self) -> &str { "components" }
    fn options(&self) -> Vec<PatchOption> {
        vec![
            PatchOption {
                key: "IME.indicator".into(),
                opt_type: "boolean".into(),
                label: "输入法指示器".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "IME.system_ime".into(),
                opt_type: "boolean".into(),
                label: "系统输入法".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "IME.ms_pinyin".into(),
                opt_type: "boolean".into(),
                label: "微软拼音".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "IME.ms_Phonetic".into(),
                opt_type: "boolean".into(),
                label: "微软注音".into(),
                default: Value::Bool(true),
                desc: None,
                choices: None,
            },
            PatchOption {
                key: "IME.ms_wubi".into(),
                opt_type: "boolean".into(),
                label: "微软五笔".into(),
                default: Value::Bool(false),
                desc: None,
                choices: None,
            },
        ]
    }

    fn apply(&self, ctx: &PatchContext) -> Result<(), String> {
        // Add IME files
        if ctx.get_option_bool("IME.system_ime") {
            let patterns: Vec<&str> = vec![
                "\\Windows\\System32\\ime",
            ];
            ctx.add_files_from_source(&patterns)?;
        }

        // Configure input methods
        if ctx.get_option_bool("IME.ms_pinyin") {
            ctx.reg_add(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                Some("MsPinyin"),
                Some("REG_SZ"),
                Some("imejp.exe"),
            )?;
        }

        // IME indicator
        if ctx.get_option_bool("IME.indicator") {
            ctx.reg_add(
                r"SOFTWARE\Microsoft\CTF",
                Some("EnableIndicator"),
                Some("REG_DWORD"),
                Some("1"),
            )?;
        }

        Ok(())
    }
}
