pub use crate::get_runnable_scripts::Script;

pub mod gen_stats_wealthsimple;
pub mod gen_stats_tangerine_chequing;
pub mod gen_stats_investia_tfsa;

pub fn get_internal(view_name: &str) -> Vec<Script> {
    match view_name {
        "wealthsimple" => vec![gen_stats_wealthsimple::get_script_info()],
        "tangerine_chequing" => vec![gen_stats_tangerine_chequing::get_script_info()],
        "investia_tfsa" => vec![gen_stats_investia_tfsa::get_script_info()],
        _ => vec![],
    }
}

pub async fn run_internal(function_name: &str, view_name: &str) -> Result<String, String> {
    match (function_name, view_name) {
        ("gen_stats_wealthsimple", "wealthsimple") => {
            Ok(gen_stats_wealthsimple::wealthsimple_balance_averages("wealthsimple".into()))
        }
        ("gen_stats_tangerine_chequing", "tangerine_chequing") => {
            Ok(gen_stats_tangerine_chequing::tangerine_chequing_balance_averages("tangerine_chequing".into()))
        }
        ("gen_stats_investia_tfsa", "investia_tfsa") => {
            Ok(gen_stats_investia_tfsa::investia_tfsa_balance_averages("investia_tfsa".into()))
        }
        _ => Err(format!("Internal script {} not found for view {}", function_name, view_name)),
    }
}