pub mod averages;
// add other script modules here
// pub mod investments;
// pub mod dividends;

#[tauri::command]
pub fn wealthsimple_data_run_script(script_name: String) -> Result<(), String> {
    match script_name.as_str() {
        "averages" => {
            averages::wealthsimple_balance_averages();
            Ok(())
        }

        // "investments" => {
        //     investments::run_investments();
        //     Ok(())
        // }

        _ => Err(format!("Unknown script: {}", script_name)),
    }
}