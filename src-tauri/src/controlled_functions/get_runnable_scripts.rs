use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db_modules::db_pool_opener::ReadPool;
use crate::internal_scripts::get_internal;

#[derive(Serialize, Deserialize, Clone)]
pub struct Script {
    pub name: String,
    pub description: String,
    pub handler: String,
    pub date_added: Option<String>,
    pub extension: Option<String>,
}

#[tauri::command]
pub fn get_built_in_scripts(view_name: String) -> Vec<Script> {
    get_internal(&view_name)
}

#[tauri::command]
pub async fn get_imported_scripts(pool: tauri::State<'_, ReadPool>) -> Result<Vec<Script>, String> {
    let rows = sqlx::query("SELECT name, path, description, date_added, extension FROM imported_scripts")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    let scripts = rows.into_iter().map(|row| {
        Script {
            name: row.get::<String, _>("name"),
            description: row.get::<String, _>("description"),
            handler: row.get::<String, _>("path"),
            date_added: row.try_get::<String, _>("date_added").ok(),
            extension: row.try_get::<String, _>("extension").ok(),
        }
    }).collect();

    Ok(scripts)
}
