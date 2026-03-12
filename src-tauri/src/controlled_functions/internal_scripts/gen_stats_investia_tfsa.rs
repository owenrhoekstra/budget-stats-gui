#[tauri::command]
pub fn investia_tfsa_balance_averages(input: String) -> String {
    format!("{}", input)
}

pub fn get_script_info() -> super::Script {
    super::Script {
        name: "General Statistics".into(),
        description: "Run general stats such as mean, sd, median, and month over month changes to balance".into(),
        handler: "gen_stats_investia_tfsa".into(),
        date_added: None,
        extension: None,
    }
}