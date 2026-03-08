use crate::db_modules::db_pool_opener::WritePool;
use chrono::{Utc};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct ImportPayload {
    name: String,
    path: PathBuf,
    description: String,
    extension: String,
}

#[tauri::command]
pub async fn import_script_to_db(app: AppHandle, pool: tauri::State<'_, WritePool>, payload: ImportPayload) -> Result<(), String> {

    if !payload.path.exists() {
        return Err("The provided script path does not exist.".into());
    }

    // Build destination folder: <app_data_dir>/imported_scripts
    let mut dest_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {}", e))?;
    dest_dir.push("imported_scripts");

    // Ensure folder exists
    fs::create_dir_all(&dest_dir)
        .map_err(|e| format!("Failed to create imported_scripts dir: {}", e))?;

    // Generate safe filename using UUID while preserving extension
    let filename = format!("{}.{}", Uuid::new_v4(), payload.extension);
    let mut dest_path: PathBuf = dest_dir;
    dest_path.push(filename);

    // Copy original file to app-owned location
    fs::copy(&payload.path, &dest_path)
        .map_err(|e| format!("Failed to copy script: {}", e))?;

    let now = Utc::now();
    let date_only = now.date_naive().format("%Y-%m-%d").to_string();


    sqlx::query("INSERT INTO imported_scripts (path, name, description, date_added, extension) VALUES (?, ?, ?, ?, ?)")
        .bind(dest_path.to_str().ok_or("Invalid path encoding")?)
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&date_only)
        .bind(&payload.extension)
        .execute(pool.as_ref())
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}