use std::process::Command;
use tauri::Manager;

#[tauri::command]
pub async fn csv_import_apple_script(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let script_path = app
            .path()
            .resolve(
                "scripts/export_numbers.scpt",
                tauri::path::BaseDirectory::Resource,
            )
            .map_err(|e| e.to_string())?;

        let output = Command::new("osascript")
            .arg(script_path)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}
