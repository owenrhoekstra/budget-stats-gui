#[tauri::command]
pub fn get_alert_data() {
    println!("I was invoked from JavaScript!");
}