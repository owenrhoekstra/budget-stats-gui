use crate::db_modules::db_pool_opener::WritePool;
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize)]
pub struct ImportPayload {
    name: String,
    path: std::path::PathBuf,
    description: String,
}

#[tauri::command]
pub async fn import_script_to_db(pool: tauri::State<'_, WritePool>, payload: ImportPayload) -> Result<(), String> {

    if !payload.path.exists() {
        return Err("The provided script path does not exist.".into());
    }

    let now = Utc::now().to_rfc3339();

    sqlx::query("INSERT INTO imported_scripts (path, name, description, date_added) VALUES (?, ?, ?, ?)")
        .bind(payload.path.to_str().ok_or("Invalid path encoding")?)
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(&now)
        .execute(pool.as_ref())
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}