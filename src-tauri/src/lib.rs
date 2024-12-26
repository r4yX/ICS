mod database;

#[tauri::command]
fn insert_budget(customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) {
   database::insert_budget(customer, vehicle, concept, kilometrage, total).unwrap();
}
#[tauri::command]
fn create_customer(name: &str, phone: &str, cuil: &str, dni: &str, tipo: &str, vehicles: Vec<&str>) {
    let _ = database::insert_customer(name, phone, cuil, dni, tipo);
    for vehicle in vehicles.iter() {
        // insert vehicle in vehicles table
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![insert_budget])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
