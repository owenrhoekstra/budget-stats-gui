mod independent_functions;
mod controlled_functions;

use independent_functions::*;
use controlled_functions::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
          carousel_alerts_data_landing_page::get_alert_data,
          csv_data_import_apple_script::csv_import_apple_script,
          wealthsimple_data_view_scripts::wealthsimple_data_run_script,
      ])
    .setup(|app| {
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
