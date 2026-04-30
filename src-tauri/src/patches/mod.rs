pub mod context;
pub mod config;
pub mod loader;
pub mod core;
pub mod components;
pub mod network;
pub mod drivers;
pub mod runtime_kits;

pub use context::PatchContext;
pub use config::PatchConfig;
pub use loader::PatchLoader;
use serde::{Deserialize, Serialize};

/// Patch trait - all patches implement this
pub trait Patch: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn category(&self) -> &str;
    fn dependencies(&self) -> Vec<&str> { vec![] }
    fn options(&self) -> Vec<PatchOption> { vec![] }
    fn apply(&self, ctx: &PatchContext) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOption {
    pub key: String,
    pub opt_type: String, // "boolean", "select", "text", "number"
    pub label: String,
    #[serde(default)]
    pub default: serde_json::Value,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<PatchChoice>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchChoice {
    pub value: serde_json::Value,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub options: Vec<PatchOption>,
}

/// Get all registered patches
pub fn get_all_patches() -> Vec<Box<dyn Patch>> {
    let mut patches: Vec<Box<dyn Patch>> = Vec::new();

    // Core
    patches.push(Box::new(core::build_config::BuildConfigPatch));
    patches.push(Box::new(core::account::AccountPatch));
    patches.push(Box::new(core::customization::CustomizationPatch));

    // Shell
    patches.push(Box::new(components::shell::ShellPatch));
    patches.push(Box::new(components::dwm::DwmPatch));
    patches.push(Box::new(components::mmc::MmcPatch));

    // Components
    patches.push(Box::new(components::vcruntime::VcRuntimePatch));
    patches.push(Box::new(components::mspaint::MspaintPatch));
    patches.push(Box::new(components::notepad::NotepadPatch));
    patches.push(Box::new(components::winphotoviewer::WinPhotoViewerPatch));
    patches.push(Box::new(components::msi::MsiPatch));
    patches.push(Box::new(components::dsmsvc::DsmsvcPatch));
    patches.push(Box::new(components::ime::ImePatch));

    // Network
    patches.push(Box::new(network::network::NetworkPatch));

    // Drivers
    patches.push(Box::new(drivers::drivers::DriversPatch));

    // Runtime Kits
    patches.push(Box::new(runtime_kits::runtime_kits::RuntimeKitsPatch));

    patches
}

/// Get patch list info for frontend
pub fn get_patch_list() -> Vec<PatchInfo> {
    get_all_patches().iter().map(|p| PatchInfo {
        id: p.id().to_string(),
        name: p.name().to_string(),
        category: p.category().to_string(),
        dependencies: p.dependencies().iter().map(|s| s.to_string()).collect(),
        options: p.options(),
    }).collect()
}
