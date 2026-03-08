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
pub async fn wealthsimple_data_run_imported_script(handler: String, db_path_state: tauri::State<'_, std::path::PathBuf>) -> Result<String, String> {
    use std::process::Stdio;

    let db_path = db_path_state.to_str().unwrap();

    // Spawn the Python script with db path as argument
    let child: tokio::process::Child = Command::new("python3")
        .arg(&handler)
        .arg(&db_path)
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
