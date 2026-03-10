mod controlled_functions;
mod db_modules;
mod independent_functions;

use controlled_functions::*;
use db_modules::*;
use independent_functions::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            carousel_alerts_data_landing_page::get_alert_data,
            csv_data_import_apple_script::csv_import_apple_script,
            wealthsimple_data_view_scripts::wealthsimple_data_run_built_in_script,
            wealthsimple_data_view_scripts::wealthsimple_data_get_built_in_scripts,
            wealthsimple_data_view_scripts::wealthsimple_data_get_imported_scripts,
            wealthsimple_data_view_scripts::wealthsimple_data_run_imported_script,
            import_script_to_db::import_script_to_db,
            delete_script_from_db::delete_archive_imported_scripts,
            tangerine_chequing_data_view_scripts::tangerine_chequing_data_run_built_in_script,
            tangerine_chequing_data_view_scripts::tangerine_chequing_data_get_built_in_scripts,
            tangerine_chequing_data_view_scripts::tangerine_chequing_data_get_imported_scripts,
            tangerine_chequing_data_view_scripts::tangerine_chequing_data_run_imported_script,
            investia_tfsa_data_view_scripts::investia_tfsa_data_run_built_in_script,
            investia_tfsa_data_view_scripts::investia_tfsa_data_get_built_in_scripts,
            investia_tfsa_data_view_scripts::investia_tfsa_data_get_imported_scripts,
            investia_tfsa_data_view_scripts::investia_tfsa_data_run_imported_script,
        ])
        .setup(|app| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let path = app.path().app_data_dir().unwrap();
                let (read_pool, write_pool) = db_modules::db_pool_opener::open_pools(&path)
                    .await
                    .expect("Failed to open database pools");
                app.manage(read_pool);
                app.manage(write_pool);
                app.manage(path);
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
