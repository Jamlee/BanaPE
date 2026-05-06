use crate::tests::{self, TestConfig, TestSuiteResult};
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn test_start(
    app: AppHandle,
    config: Option<TestConfig>,
) -> Result<TestSuiteResult, String> {
    let cfg = config.unwrap_or_default();
    tests::run_test_suite(&app, cfg).await
}

#[tauri::command]
pub fn test_is_running() -> bool {
    tests::is_test_running()
}

#[tauri::command]
pub fn test_get_default_config() -> TestConfig {
    TestConfig::default()
}
