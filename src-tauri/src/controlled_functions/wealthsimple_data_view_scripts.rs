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

    let result = Command::new("python3")
        .arg(&handler)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
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
}

#[tauri::command]
pub fn wealthsimple_data_get_built_in_scripts() -> Vec<Script> {
    vec![
        Script {
            name: "General Statistics".into(),
            description: "Run general stats such as mean, sd, median, and month over month changes to balance".into(),
            handler: "gen_stats".into(),
        },
        Script {
            name: "Data Skewness".into(),
            description: "Inspect Kurtosis, skewness, deviations and more".into(),
            handler: "data_skew".into(),
        },
    ]
}

use rusqlite::Connection;
use std::path::Path;
#[tauri::command]
pub fn wealthsimple_data_get_imported_scripts() -> Vec<Script> {
    let conn = Connection::open("/Users/owenhoekstra/.homemade-apps/budget-stats-gui/db/budget-stats-gui-db").unwrap();

    let mut stmt = conn
        .prepare("SELECT path FROM imported_scripts")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            let path: String = row.get(0)?;

            let name = Path::new(&path)
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            Ok(Script {
                name,
                description: "Imported script".into(),
                handler: path,
            })
        })
        .unwrap();

    rows.map(|s| s.unwrap()).collect()
}