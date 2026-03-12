#[tauri::command]
pub fn tangerine_chequing_balance_averages(input: String) -> String {
    format!("{}", input)
}

pub fn get_script_info() -> super::Script {
    super::Script {
        name: "General Statistics".into(),
        description: "Run general stats such as mean, sd, median, and month over month changes to balance".into(),
        handler: "gen_stats_tangerine_chequing".into(),
        date_added: None,
        extension: None,
    }
}