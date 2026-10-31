#![forbid(unsafe_code)]

use firetv_core::{
    AdbTransport, ConnectionState, DeviceFacts, LauncherCatalog, RsAdbTransport, TargetAddress,
    inspect_device,
};
use tauri::Manager;

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

#[tauri::command]
async fn inspect_target(app: tauri::AppHandle, target: &str) -> Result<DeviceFacts, String> {
    let target = TargetAddress::parse(target).map_err(|error| error.to_string())?;
    let key_directory = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("adb-keys");
    std::fs::create_dir_all(&key_directory).map_err(|error| error.to_string())?;
    let transport = RsAdbTransport::new(&key_directory);
    match transport
        .connect(target)
        .await
        .map_err(|error| error.to_string())?
    {
        ConnectionState::Connected => inspect_device(&transport)
            .await
            .map_err(|error| error.to_string()),
        ConnectionState::Unauthorized => {
            Err("Approve the RSA authorization prompt on the Fire TV, then retry.".into())
        }
        ConnectionState::Offline => {
            Err("The Fire TV is offline or unreachable at this address.".into())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Starts the native desktop or mobile application shell.
///
/// # Panics
///
/// Panics if the operating-system webview cannot be initialized.
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            validate_target,
            launcher_catalog,
            inspect_target
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Universal Fire TV Manager");
}
