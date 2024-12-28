mod database;
use database::*;

#[tauri::command]
fn create_budget(customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) {
   insert_budget(customer, vehicle, concept, kilometrage, total).unwrap();
}
#[tauri::command]
fn create_customer(name: &str, phone: &str, cuil: &str, dni: &str, tipo: &str, vehicles: Vec<&str>) {
    let _ = database::insert_customer(name, phone, cuil, dni, tipo);
    for vehicle in vehicles.iter() {
        update_vehicles(vehicle, name);
    }
}
#[tauri::command]
fn create_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) {
    insert_item(id, name, price, tipo, manufacturer, supplier, model, stock)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_budget, create_customer, create_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
