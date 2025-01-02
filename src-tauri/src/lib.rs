mod database;
use database::*;

#[tauri::command]
fn create_budget(id: &str, date: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) {
   insert_budget(id, customer, vehicle, concept, kilometrage, total).unwrap();
   update_balance(date, "Ingreso", total, &format!("{} - {}", customer, concept)).unwrap();
}
#[tauri::command]
fn create_order(id: &str, paid: f32) -> Result<(), String> {

    match read_budget("1") {
        Ok(budget) => {
                let _ = insert_order(
                    id, &budget.customer, &budget.vehicle, &budget.concept,
                    budget.kilometrage, budget.total, paid
                );
            Ok(())
            }
        Err(e) => Err(format!("Error: {}", e)),
    }
}
#[tauri::command]
fn create_customer(name: &str, phone: &str, cuil: &str, dni: &str, tipo: &str, vehicles: Vec<&str>) {
    let _ = database::insert_customer(name, phone, cuil, dni, tipo);
    for vehicle in vehicles.iter() {
        let _ = update_vehicles(vehicle, name);
    }
}
#[tauri::command]
fn create_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) {
    let _ = insert_item(id, name, price, tipo, manufacturer, supplier, model, stock);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_budget, create_customer, create_item, create_order])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
