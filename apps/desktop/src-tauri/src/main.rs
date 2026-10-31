#![forbid(unsafe_code)]

use firetv_core::{LauncherCatalog, TargetAddress};

#[tauri::command]
fn validate_target(target: &str) -> Result<String, String> {
    TargetAddress::parse(target)
        .map(|address| address.to_string())
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn launcher_catalog() -> Result<LauncherCatalog, String> {
    LauncherCatalog::bundled().map_err(|error| error.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_target, launcher_catalog])
        .run(tauri::generate_context!())
        .expect("failed to run Universal Fire TV Manager");
}
