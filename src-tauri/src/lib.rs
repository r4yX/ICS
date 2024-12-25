mod database;

#[tauri::command]
fn insert_budget(customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) {
   let _ = database::insert_budget(customer, vehicle, concept, kilometrage, total).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![insert_budget])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
