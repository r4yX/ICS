mod schema;
use schema::*;
mod database;
use database::*;
mod output;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    id: String,
    item: String,
    price: f32,
    cant: u8,
    tipo: String,
    subtotal: f32,
    iva: f32,
    total: f32
}
#[tauri::command]
fn create_pdf(info: output::Information, client: output::Client, vehicle: output::Vehicle, details: Vec<output::Detail>) -> Result<String, String> {
    let res = output::write_pdf(info, client, vehicle, details)?;
    Ok(res)
}

// Write in database
#[tauri::command]
fn create_budget(id: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, details: Vec<Detail>) {
   insert_budget(id, customer, vehicle, concept, kilometrage, total).unwrap();

   for detail in details.iter() {
       let _ = insert_detail(id, &detail.item, detail.price, detail.cant, &detail.tipo, detail.subtotal, detail.iva, detail.total);
   }
}
#[tauri::command]
fn create_order(id: &str, paid: f32, date: &str) -> Result<String, String> {
    let budget = match read_budget(id) {
        Ok(budget) => budget,
        Err(e) => return Err(format!("Error {}. Reading budget with id {}", e, id))
    };
   // Add to balance
   if paid != 0.0 {
       update_balance(date, "Ingreso", &format!("{} - {}", budget.customer, id), budget.total).unwrap();
   };
    match insert_order(
        id, &budget.customer, &budget.vehicle, &budget.concept,
        budget.kilometrage, budget.total, paid
    ) {
        Ok(_) => "Order inserted".to_string(),
        Err(e) => return Err(format!("Error: {}", e)),
    };

    // Delete budget
    match delete_budget(id) {
        Ok(_) => Ok(format!("Budget {} is approved.", id)),
        Err(e) => Err(format!("Err ({}) deleting {} budget", e, id))
    }
}
#[tauri::command]
fn create_history(id: &str, pay_date: &str) -> Result<String, String> {
    let order = match read_order(id) {
        Ok(order) => order,
        Err(e) => return Err(format!("Err: ({}) reading order {}", e, id)),
    };
    match insert_history(id, order, pay_date) {
        Ok(_) => "History entry inserted successfully",
        Err(e) => return Err(format!("Err: ({}) creating history entry {}", e, id))
    };
    match delete_detail(id) {
        Ok(_) => "Old details info deleted successfully",
        Err(e) => return Err(format!("Err: ({}) cleaning old details({}) info", e, id ))
    };
    match delete_order(id) {
        Ok(_) => Ok("Order paid successfully and archived".to_string()),
        Err(e) => Err(format!("Err: ({}) moving order {}", e, id))
    }
}
#[tauri::command]
fn create_customer(name: &str, phone: &str, cuit: &str, dni: &str, tipo: &str, vehicles: Vec<&str>) -> Result<String, String> {
    for vehicle in vehicles.iter() {
        let _ = update_vehicles(vehicle, name);
    }
    match insert_customer(name, phone, cuit, dni, tipo) {
        Ok(_) => Ok(format!("client {} created successfully", name)),
        Err(e) => Err(format!("Err ({}) creating client {}", e, name)),
    }
}
#[tauri::command]
fn create_vehicle(domain: &str, maker: &str, model: &str, tipo: &str, colour: &str, year: u16, owner: &str) -> Result<String, String> {
    let res = insert_vehicle(domain, maker, model, tipo, colour, year, owner)?;
    Ok(res)
}
#[tauri::command]
fn create_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) -> Result<String, String> {
    let res = insert_item(id, name, price, tipo, manufacturer, supplier, model, stock)?;
    Ok(res)
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
    let res = database::update_balance(date, "Salida", &format!("Pago a {}", name), amount)?;
    Ok(res)
}

// Update database
#[tauri::command]
fn pay_order(id: &str, paid: f32) -> Result<String, String> {
    let res = database::update_order(id, paid)?;
    Ok(res)
}

// Read database
#[tauri::command]
fn obtain_budgets() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_budgets()?;
    Ok(res)
}
#[tauri::command]
fn obtain_orders() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_orders()?;
    Ok(res)
}
#[tauri::command]
fn obtain_history() -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_history()?;
    Ok(res)
}
#[tauri::command]
fn obtain_customers() -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_all_customers()?;
    Ok(res)
}
#[tauri::command]
fn obtain_items() -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_all_items()?;
    Ok(res)
}
#[tauri::command]
fn obtain_details(id: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let res = database::read_all_details(id)?;
    Ok(res)
}
#[tauri::command]
fn obtain_vehicles() -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_all_vehicles()?;
    Ok(res)
}
#[tauri::command]
fn obtain_workers() -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_all_workers()?;
    Ok(res)
}
#[tauri::command]
fn obtain_balance(date: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_all_movements(date)?;
    Ok(res)
}
#[tauri::command]
fn obtain_payments(dni: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let res = read_payments(dni)?;
    Ok(res)
}
// Delete functions
#[tauri::command]
fn delete_history(id: &str) -> Result<String, String> {
    let res = delete_from_history(id)?;
    Ok(res)
}
#[tauri::command]
fn remove_worker(dni: &str) -> Result<String, String> {
    let res = delete_worker(dni)?;
    Ok(res)
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let path = database::get_db_path();
    let _ = create_tables(path);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![create_budget, create_customer, create_vehicle,
        create_item, create_order, create_worker, create_payment, create_history, obtain_budgets,
        obtain_orders, obtain_history, obtain_details, obtain_vehicles, obtain_customers,
        obtain_items, obtain_workers, obtain_balance, obtain_payments, pay_order, create_pdf,
        delete_history, remove_worker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
