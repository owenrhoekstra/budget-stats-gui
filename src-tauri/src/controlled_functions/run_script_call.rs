use std::collections::HashMap;
use std::process::Stdio;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use hex;
use sqlx::Row;
use tokio::process::Command;
use crate::db_modules::db_pool_opener::ReadPool;
use crate::internal_scripts::run_internal;

#[derive(Deserialize)]
#[serde(tag = "type")] // This tells Serde to look for a "type" field in the JSON
pub enum CommandPayload {
    BuiltIn {
        function_name: String,
        view_name: String,
    },
    External {
        handler: String,
        view_name: String,
    }
}


#[tauri::command]
pub async fn execute_script(db_path_state: tauri::State<'_, std::path::PathBuf>, pool: tauri::State<'_, ReadPool>, payload: CommandPayload) -> Result<String, String> {
    match payload {
        CommandPayload::BuiltIn { function_name, view_name} => {
            // Direct execution, no overhead
            run_internal(&function_name, &view_name).await
        },
        CommandPayload::External { handler, view_name } => {
            // Run your verification logic here
            run_external_script(db_path_state, pool, &handler, &view_name).await
        }
    }
}

async fn run_external_script(
    db_path_state: tauri::State<'_, std::path::PathBuf>,
    pool: tauri::State<'_, ReadPool>,
    handler: &String,
    view_name: &String,
) -> Result<String, String> {

    let path = handler.clone();

    let script_info = sqlx::query("SELECT name, extension FROM imported_scripts WHERE path = ?")
    .bind(&path)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| format!("Failed to fetch script row: {}", e))?;

    let name = script_info.get::<String, _>("name");
    let extension = script_info.get::<String, _>("extension");

    let app_directory_parent_path = db_path_state.to_string_lossy().to_string();

    let db_extension = "db/budget-stats-gui.db";

    let table_name = view_name.clone();

    let mut json_payload_external_script = serde_json::json! ({
        "name": name,
        "db_parent_path": app_directory_parent_path,
        "caller": table_name,
        "expected_db_path_extension": db_extension,
        "expected_table_name": table_name,
    });

    let payload_to_hash = json_payload_external_script.to_string();

    let mut hasher = Sha256::new();
    hasher.update(payload_to_hash.as_bytes());
    let hash_result = hasher.finalize();
    let hash = hex::encode(hash_result);

    json_payload_external_script["hash"] = serde_json::Value::String(hash);

    let mut interpreter_map = HashMap::new();
    interpreter_map.insert("py", "python3");
    interpreter_map.insert("R", "RScript");

    let interpreter = interpreter_map
        .get(extension.as_str())
        .ok_or_else(|| format!("Unsupported file extension: {}", extension))?;

    let child= Command::new(interpreter)
        .arg(&handler)
        .arg(json_payload_external_script.to_string())
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