pub mod gen_stats;
// add other script modules here
// pub mod investments;
// pub mod dividends;

use tokio::process::Command;

#[tauri::command]
pub fn wealthsimple_data_run_built_in_script(script_name: String) -> Result<String, String> {
    match script_name.as_str() {
        "gen_stats" => {
            // Capture output from the module function
            let output = gen_stats::wealthsimple_balance_averages();
            Ok(output)
        }

        // "investments" => {
        //     let output = investments::run_investments();
        //     Ok(output)
        // }

        _ => Err(format!("Unknown script: {}", script_name)),
    }
}

#[tauri::command]
pub async fn wealthsimple_data_run_imported_script(pool: tauri::State<'_, ReadPool>, handler: String, db_path_state: tauri::State<'_, std::path::PathBuf>) -> Result<String, String> {
    use std::collections::HashMap;
    use std::path::Path;
    use std::process::Stdio;
    use chrono::{Utc};

    let db_parent_path = std::fs::canonicalize(&*db_path_state)
        .map_err(|e| format!("Failed to canonicalize path: {}", e))?
        .to_string_lossy()
        .to_string();

    // Determine file extension
    let extension = Path::new(&handler)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| "Failed to determine file extension".to_string())?;

    // Map extensions to interpreters
    let mut interpreter_map: HashMap<&str, &str> = HashMap::new();
    interpreter_map.insert("py", "python3");
    interpreter_map.insert("R", "RScript");

    let interpreter = interpreter_map
        .get(extension)
        .ok_or_else(|| format!("Unsupported file extension: {}", extension))?;

    let script_row = sqlx::query("SELECT name FROM imported_scripts WHERE path = ?")
        .bind(&handler)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|e| format!("Failed to fetch script row: {}", e))?;

    let mut imported_script_payload_json = serde_json::json! ({
        "name": script_row.get::<String, _>("name"),
        "db_parent_path": db_parent_path,
        "caller": "wealthsimple_data_view",
        "expected_db_path_extension": "db/budget-stats-gui.db",
        "expected_table_name": "wealthsimple",
        "time": Utc::now().to_rfc3339()
    });

    let payload_to_hash = imported_script_payload_json.to_string();

    use sha2::{Digest, Sha256};
    use hex;

    let mut hasher = Sha256::new();
    hasher.update(payload_to_hash.as_bytes());
    let hash_result = hasher.finalize();
    let hash = hex::encode(hash_result);

    imported_script_payload_json["hash"] = serde_json::Value::String(hash);

    // Spawn the Python script with db path as argument
    let child: tokio::process::Child = Command::new(interpreter)
        .arg(&handler)
        .arg(imported_script_payload_json.to_string())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn process: {}", e))?;

    let output = child
        .wait_with_output()
        .await
        .map_err(|e| format!("Failed while waiting for process: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!("{}\nError:\n{}", stdout, stderr))
    }
}

#[derive(serde::Serialize)]
pub struct Script {
    name: String,
    description: String,
    handler: String,
    date_added: Option<String>,
    extension: Option<String>,
}

#[tauri::command]
pub fn wealthsimple_data_get_built_in_scripts() -> Vec<Script> {
    vec![
        Script {
            name: "General Statistics".into(),
            description: "Run general stats such as mean, sd, median, and month over month changes to balance".into(),
            handler: "gen_stats".into(),
            date_added: None,
            extension: None,
        },
    ]
}

use sqlx::Row;
use crate::db_modules::db_pool_opener::ReadPool;

#[tauri::command]
pub async fn wealthsimple_data_get_imported_scripts(pool: tauri::State<'_, ReadPool>) -> Result<Vec<Script>, String> {
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
