use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use crate::db_modules::db_pool_opener::WritePool;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn delete_archive_imported_scripts(pool: tauri::State<'_, WritePool>, path: String) -> Result<(), String> {
    // Determine original file path
    let original_path = PathBuf::from(&path);
    if !original_path.exists() {
        return Err("Original script file does not exist".into());
    }

    // Determine archive folder adjacent to imported_scripts folder
    let imported_dir = original_path.parent()
        .ok_or("Cannot determine imported_scripts folder")?
        .to_path_buf();
    let mut archive_dir = imported_dir.parent() // move up to app data dir
        .ok_or("Cannot determine app data dir")?
        .to_path_buf();
    archive_dir.push("archived_imported_scripts");

    fs::create_dir_all(&archive_dir)
        .map_err(|e| format!("Failed to create archive folder: {}", e))?;

    // Build new file path in archive folder
    let file_name = original_path.file_name()
        .ok_or("Cannot determine file name")?;
    let mut archived_path = archive_dir;
    archived_path.push(file_name);

    // Move the file
    fs::rename(&original_path, &archived_path)
        .map_err(|e| format!("Failed to move file to archive: {}", e))?;

    let now = Utc::now().format("%Y-%m-%d").to_string();
    let pool: &SqlitePool = &*pool; // Deref the Tauri state wrapper to get the pool

    // Insert into archive table
    sqlx::query("INSERT INTO archived_imported_scripts (path, name, description, date_added, extension, date_archived) SELECT ?, name, description, date_added, extension, ? FROM imported_scripts WHERE path = ?")
        .bind(archived_path.to_str().ok_or("Invalid archived path")?)
        .bind(&now)
        .bind(&path)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to insert into archive table: {}", e))?;

    // Delete original entry
    sqlx::query("DELETE FROM imported_scripts WHERE path = ?")
        .bind(&path)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete from imported_scripts table: {}", e))?;

    Ok(())
}