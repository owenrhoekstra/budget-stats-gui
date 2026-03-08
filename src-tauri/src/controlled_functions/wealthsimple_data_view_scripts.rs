pub mod gen_stats;
// add other script modules here
// pub mod investments;
// pub mod dividends;

#[tauri::command]
pub fn wealthsimple_data_run_built_in_script(script_name: String) -> Result<(), String> {
    match script_name.as_str() {
        "gen_stats" => {
            gen_stats::wealthsimple_balance_averages();
            Ok(())
        }

        // "investments" => {
        //     investments::run_investments();
        //     Ok(())
        // }
        _ => Err(format!("Unknown script: {}", script_name)),
    }
}

#[tauri::command]
pub fn wealthsimple_data_run_imported_script(handler: String) -> Result<(), String> {
    use std::process::Command;

    println!("Running script at: {}", handler);

    let result = Command::new("python3").arg(&handler).output();

    match result {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(format!("Failed to run imported script: {}", e)),
    }
}

#[derive(serde::Serialize)]
pub struct Script {
    name: String,
    description: String,
    handler: String,
    date_added: Option<String>,
}

#[tauri::command]
pub fn wealthsimple_data_get_built_in_scripts() -> Vec<Script> {
    vec![
        Script {
            name: "General Statistics".into(),
            description: "Run general stats such as mean, sd, median, and month over month changes to balance".into(),
            handler: "gen_stats".into(),
            date_added: None,
        },
        Script {
            name: "Data Skewness".into(),
            description: "Inspect Kurtosis, skewness, deviations and more".into(),
            handler: "data_skew".into(),
            date_added: None,
        },
    ]
}

use sqlx::Row;
use crate::db_modules::db_pool_opener::ReadPool;

#[tauri::command]
pub async fn wealthsimple_data_get_imported_scripts(pool: tauri::State<'_, ReadPool>) -> Result<Vec<Script>, String> {
    let rows = sqlx::query("SELECT name, path, description, date_added FROM imported_scripts")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| e.to_string())?;

    let scripts = rows.into_iter().map(|row| {
        Script {
            name: row.get::<String, _>("name"),
            description: row.get::<String, _>("description"),
            handler: row.get::<String, _>("path"),
            date_added: row.try_get::<String, _>("date_added").ok(),
        }
    }).collect();

    Ok(scripts)
}
