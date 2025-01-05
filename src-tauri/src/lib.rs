mod database;
use database::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    id: String,
    item: String,
    price: f32,
    cant: u8,
    tipo: String,
    subtotal: f32,
    iva: u8,
    total: f32
}

#[tauri::command]
fn create_budget(id: &str, date: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, details: &str) {
   insert_budget(id, customer, vehicle, concept, kilometrage, total).unwrap();

   let details: Vec<Detail> = serde_json::from_str(details).expect("Error al deserializar");
   for detail in details.iter() {
       let _ = insert_detail(id, &detail.item, detail.price, detail.cant, &detail.tipo, detail.subtotal, detail.iva, detail.total);
   }
   // Add to balance
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
#[tauri::command]
fn create_worker(name: &str, dni: &str, phone: &str, address: &str, salary: f32) -> Result<String, String> {
    let res = database::insert_worker(name, dni, phone, address, salary)?;
    Ok(res)
}
#[tauri::command]
fn create_payment(name: &str, dni: &str, date: &str, amount: f32) -> Result<String, String> {
    match database::insert_payment(name, dni, date, amount) {
        Ok(_) => "",
        Err(e) => return Err(e),
    };
    let res = database::update_balance(date, "Salida", amount, &format!("Pago a {}", name))?;
    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_budget, create_customer, create_item, create_order, create_worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
