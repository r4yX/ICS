use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use directories::ProjectDirs;
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    // is stupid return 'id' field
    pub customer: String,
    pub vehicle: String,
    pub concept: String,
    pub kilometrage: f32,
    pub total: f32,
}
pub struct Order {
    pub customer: String,
    pub vehicle: String,
    pub concept: String,
    pub kilometrage: f32,
    pub total: f32,
    pub paid: f32,
}

pub fn get_db_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "syltr1x", "punto_diesel")
        .expect("No se pudo obtener el directorio de la aplicación");

    let data_dir = proj_dirs.data_local_dir();
    fs::create_dir_all(data_dir).expect("No se pudo crear la carpeta de la base de datos");

    data_dir.join("database.db")
}

pub fn insert_budget(id: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Error ({}) opening database connection", e)),
    };
    match conn.execute(
        "INSERT INTO budgets (id, client, vehicle, concept, kilometrage, total) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, customer, vehicle, concept, kilometrage, total],
    ) {
        Ok(_) => Ok("client added successfully".to_string()),
        Err(e) => Err(format!("Err ({})", e)),
    }
}
pub fn insert_customer(name: &str, phone: &str, cuit: &str, dni: &str, tipo: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    match conn.execute(
        "INSERT INTO clients (name, phone, cuil, dni, tipo) VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(dni) DO UPDATE SET phone=excluded.phone, cuil=excluded.cuil,
        name=excluded.name, tipo=excluded.tipo",
        params![name, phone, cuit, dni, tipo],
    ) {
        Ok(_) => Ok("client added successfully".to_string()),
        Err(e) => Err(format!("Err ({})", e)),
    }
}
pub fn insert_vehicle(domain: &str, maker: &str, model: &str, tipo: &str, colour: &str, year: u16, owner: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    match conn.execute(
        "INSERT INTO vehicles (domain, maker, model, tipo, colour, year, owner) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![domain, maker, model, tipo, colour, year, owner],
    ) {
        Ok(_) => Ok(format!("Vehicle {} created successfully", domain)),
        Err(e) => Err(format!("Err ({}) creating vehicle {}", e, domain)),
    }
}
pub fn update_vehicles(domain: &str, owner: &str) -> Result <String, String>{
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "UPDATE vehicles SET owner='' WHERE owner=?1",
        params![owner]
    ) {
        Ok(ok) => ok,
        Err(e) => return Err(e.to_string())
    };
    match conn.execute(
        "UPDATE vehicles SET owner=?1 WHERE domain=?2",
        params![owner, domain]
    ) {
        Ok(_) => Ok(format!("{} now is the owner of {}", owner, domain)),
        Err(e) => Err(e.to_string())
    }
}
pub fn insert_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO inventory (id, name, price, tipo, manufacturer, supplier, model, stock) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ON CONFLICT(id) DO UPDATE SET name=excluded.name, price=excluded.price, tipo=excluded.tipo, manufacturer=excluded.manufacturer, 
        supplier=excluded.supplier, model=excluded.model, stock=excluded.stock",
        params![id, name, price, tipo, manufacturer, supplier, model, stock],
    ) {
        Ok(_) => Ok(format!("Item {} created successfully.", name)),
        Err(e) => Err(format!("Err ({}) creating item {}", e, name)),
    }
}
pub fn update_balance(date: &str, tipo: &str, name: &str, amount: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO balance (date, tipo, name, amount) VALUES (?1, ?2, ?3, ?4)",
        params![date, tipo, name, amount],
    ) {
        Ok(_) => Ok("Balance updated successfully".to_string()),
        Err(_) => Err("Error updating balance".to_string())
    }
}
pub fn update_order(id: &str, paid: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "UPDATE orders SET paid=?1 WHERE id=?2",
        params![paid, id],
    ) {
        Ok(_) => Ok(format!("Pay added to order {}", id)),
        Err(e) => Err(format!("Err: ({}) paying order {}", e, id)),
    }
}
pub fn insert_order(id: &str, client: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, paid: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO orders (id, client, vehicle, concept, kilometrage, total, paid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, client, vehicle, concept, kilometrage, total, paid],
    ) {
        Ok(_) => Ok(format!("order {} created successfully", id)),
        Err(_) => Err(format!("Error creating order {}", id)),
    }
}
pub fn read_all_budgets() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = Connection::open(path).unwrap();
    let mut stmt = match conn.prepare("SELECT * FROM budgets") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let budgets_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let id: String = row.get(0)?;
        let customer: String = row.get(1)?;
        let vehicle: String = row.get(2)?;
        let concept: String = row.get(3)?;
        let kilometrage: f32 = row.get(4)?;
        let total: f32 = row.get(5)?;

        // Insert values in the HashMap
        map.insert("id".to_string(), id);
        map.insert("customer".to_string(), customer);
        map.insert("vehicle".to_string(), vehicle);
        map.insert("concept".to_string(), concept);
        map.insert("kilometrage".to_string(), kilometrage.to_string());
        map.insert("total".to_string(), total.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut budgets: Vec<HashMap<String, String>> = Vec::new();

    for budget in budgets_iter {
        match budget {
            Ok(b) => { 
                budgets.push(b)},
            Err(e) => {
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(budgets)
} 
pub fn read_all_orders() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM orders") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let orders_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let id: String = row.get(0)?;
        let customer: String = row.get(1)?;
        let vehicle: String = row.get(2)?;
        let concept: String = row.get(3)?;
        let kilometrage: f32 = row.get(4)?;
        let total: f32 = row.get(5)?;
        let paid: f32 = row.get(6)?;

        // Insert values in the HashMap
        map.insert("id".to_string(), id);
        map.insert("customer".to_string(), customer);
        map.insert("vehicle".to_string(), vehicle);
        map.insert("concept".to_string(), concept);
        map.insert("kilometrage".to_string(), kilometrage.to_string());
        map.insert("total".to_string(), total.to_string());
        map.insert("paid".to_string(), paid.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut orders: Vec<HashMap<String, String>> = Vec::new();

    for order in orders_iter {
        match order {
            Ok(o) => { 
                orders.push(o)},
            Err(e) => {
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(orders)
} 
pub fn read_all_history() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM history") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let history_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let id: String = row.get(0)?;
        let customer: String = row.get(1)?;
        let vehicle: String = row.get(2)?;
        let concept: String = row.get(3)?;
        let kilometrage: f32 = row.get(4)?;
        let total: f32 = row.get(5)?;
        let details: String = row.get(6)?;
        let pay_date: String = row.get(7)?;

        // Insert values in the HashMap
        map.insert("id".to_string(), id);
        map.insert("customer".to_string(), customer);
        map.insert("vehicle".to_string(), vehicle);
        map.insert("concept".to_string(), concept);
        map.insert("kilometrage".to_string(), kilometrage.to_string());
        map.insert("total".to_string(), total.to_string());
        map.insert("details".to_string(), details);
        map.insert("pay_date".to_string(), pay_date);
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut history: Vec<HashMap<String, String>> = Vec::new();

    for entry in history_iter {
        match entry {
            Ok(h) => { 
                history.push(h)},
            Err(e) => {
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(history)
}
pub fn read_all_details(id: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM details WHERE id=?1") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let details_iter = match stmt.query_map([id], |row| {
        let mut map = HashMap::new();
        let _: String = row.get(0)?;
        let item: String = row.get(1)?;
        let tipo: String = row.get(4)?;

        // Convertir 'kilometrage' correctamente, manejando 'REAL' como f32
        let price: f32 = row.get(2)?;
        let cant: u8 = row.get(3)?;
        let subtotal: f32 = row.get(5)?;
        let iva: f32 = row.get(6)?;
        let total: f32 = row.get(7)?;

        // Insertar los valores en el HashMap
        map.insert("item".to_string(), item);
        map.insert("price".to_string(), price.to_string());
        map.insert("cant".to_string(), cant.to_string());
        map.insert("tipo".to_string(), tipo.to_string());
        map.insert("subtotal".to_string(), subtotal.to_string());
        map.insert("iva".to_string(), iva.to_string());
        map.insert("total".to_string(), total.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut details: Vec<HashMap<String, String>> = Vec::new();

    for detail in details_iter {
        match detail {
            Ok(d) => { 
                details.push(d)},
            Err(e) => {
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(details)
} 
pub fn read_all_vehicles() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM vehicles") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };

    let vehicles_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let domain: String = row.get(0)?;
        let maker: String = row.get(1)?;
        let model: String = row.get(2)?;
        let tipo: String = row.get(3)?;
        let colour: String = row.get(4)?;
        let year: u16 = row.get(5)?;
        let owner: String = row.get(6)?;

        // Insert values in the HashMap
        map.insert("domain".to_string(), domain);
        map.insert("maker".to_string(), maker);
        map.insert("model".to_string(), model);
        map.insert("tipo".to_string(), tipo);
        map.insert("colour".to_string(), colour);
        map.insert("year".to_string(), year.to_string());
        map.insert("owner".to_string(), owner);
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut vehicles: Vec<HashMap<String, String>> = Vec::new();

    for vehicle in vehicles_iter {
        match vehicle {
            Ok(v) => vehicles.push(v),
            Err(e) => return Err(format!("Failed to process row {}", e).to_string()),
        }
    };
    Ok(vehicles)
}
pub fn read_all_items() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM inventory") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };

    let inventory_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let price: f32 = row.get(2)?;
        let tipo: String = row.get(3)?;
        let manufacturer: String = row.get(4)?;
        let supplier: String = row.get(5)?;
        let model: String = row.get(6)?;
        let stock: u16 = row.get(7)?;

        // Insert values in the HashMap
        map.insert("id".to_string(), id);
        map.insert("name".to_string(), name);
        map.insert("price".to_string(), price.to_string());
        map.insert("tipo".to_string(), tipo);
        map.insert("manufacturer".to_string(), manufacturer);
        map.insert("supplier".to_string(), supplier);
        map.insert("model".to_string(), model);
        map.insert("stock".to_string(), stock.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut inventory: Vec<HashMap<String, String>> = Vec::new();

    for item in inventory_iter {
        match item {
            Ok(i) => inventory.push(i),
            Err(e) => return Err(format!("Failed to process row {}", e).to_string()),
        }
    };
    Ok(inventory)
}
pub fn read_all_customers() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM clients") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let customers_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let name: String = row.get(0)?;
        let phone: String = row.get(1)?;
        let cuil: String = row.get(2)?;
        let dni: String = row.get(3)?;
        let tipo: String = row.get(4)?;

        // Insert values in the HashMap
        map.insert("name".to_string(), name);
        map.insert("phone".to_string(), phone);
        map.insert("cuil".to_string(), cuil);
        map.insert("dni".to_string(), dni);
        map.insert("tipo".to_string(), tipo);
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut customers: Vec<HashMap<String, String>> = Vec::new();

    for customer in customers_iter {
        match customer {
            Ok(c) => customers.push(c),
            Err(e) => return Err(format!("Failed to process row {}", e).to_string()),
        }
    };
    Ok(customers)
}
pub fn read_all_workers() -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM workers") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let workers_iter = match stmt.query_map([], |row| {
        let mut map = HashMap::new();
        let name: String = row.get(0)?;
        let phone: String = row.get(1)?;
        let dni: String = row.get(2)?;
        let address: String = row.get(3)?;
        let salary: f32 = row.get(4)?;

        // Insert values in the HashMap
        map.insert("name".to_string(), name);
        map.insert("phone".to_string(), phone);
        map.insert("dni".to_string(), dni);
        map.insert("address".to_string(), address);
        map.insert("salary".to_string(), salary.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut workers: Vec<HashMap<String, String>> = Vec::new();

    for worker in workers_iter {
        match worker {
            Ok(w) => workers.push(w),
            Err(e) => return Err(format!("Failed to process row {}", e).to_string()),
        }
    };
    Ok(workers)
}
pub fn read_all_movements(date: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM balance WHERE date LIKE ?1") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };

    let balance_iter = match stmt.query_map([format!("{}%", date)], |row| {
        let mut map = HashMap::new();
        let date: String = row.get(0)?;
        let tipo: String = row.get(1)?;
        let name: String = row.get(2)?;
        let amount: f32 = row.get(3)?;

        // Insert values in the HashMap
        map.insert("date".to_string(), date);
        map.insert("tipo".to_string(), tipo);
        map.insert("name".to_string(), name);
        map.insert("amount".to_string(), amount.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut balance: Vec<HashMap<String, String>> = Vec::new();

    for movement in balance_iter {
        match movement {
            Ok(m) => balance.push(m),
            Err(e) => return Err(format!("Failed to process row {}", e).to_string()),
        }
    };
    Ok(balance)
}
pub fn read_budget(id: &str) -> Result<Budget, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM budgets WHERE id=?") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let budget = stmt.query_row(params![id], |row| {
        Ok(Budget {
            customer: row.get(1)?,
            vehicle: row.get(2)?,
            concept: row.get(3)?,
            kilometrage: row.get(4)?,
            total: row.get(5)?,
        })
    });

    match budget {
        Ok(b) => Ok(b),
        Err(_) => Err("Budget not found".to_string()),
    }
}
pub fn read_order(id: &str) -> Result<Order, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM orders WHERE id=?") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let order = stmt.query_row(params![id], |row| {
        Ok(Order {
            customer: row.get(1)?,
            vehicle: row.get(2)?,
            concept: row.get(3)?,
            kilometrage: row.get(4)?,
            total: row.get(5)?,
            paid: row.get(6)?,
        })
    });
    match order {
        Ok(o) => Ok(o),
        Err(_) => Err("Order not found".to_string()),
    }
}
pub fn read_details(id: &str) -> Result<Vec<HashMap<String, String>>, String>{
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM details WHERE id=?") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let details_iter = match stmt.query_map([id], |row| {
        let mut map = HashMap::new();

        let _: String = row.get(0)?;
        let item: String = row.get(1)?;
        let tipo: String = row.get(4)?;

        // Manage numeric rows
        let price: f32 = row.get(2)?;
        let cant: u8 = row.get(3)?;
        let subtotal: f32 = row.get(5)?;
        let iva: f32 = row.get(6)?;
        let total: f32 = row.get(7)?;

        map.insert("item".to_string(), item);
        map.insert("price".to_string(), price.to_string());
        map.insert("cant".to_string(), cant.to_string());
        map.insert("tipo".to_string(), tipo.to_string());
        map.insert("subtotal".to_string(), subtotal.to_string());
        map.insert("iva".to_string(), iva.to_string());
        map.insert("total".to_string(), total.to_string());
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };
    let mut details: Vec<HashMap<String, String>> = Vec::new();

    for detail in details_iter {
        match detail {
            Ok(d) => details.push(d),
            Err(_) => return Err("Failed to process row".to_string()),
        }
    }

    Ok(details)
}
pub fn insert_detail(id: &str, item: &str, price: f32, cant: u8, tipo: &str, subtotal: f32, iva: f32, total: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO details (id, item, price, cant, tipo, subtotal, iva, total) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, item, price, cant, tipo, subtotal, iva, total],
    ) {
        Ok(_) => Ok("Details added successfully".to_string()),
        Err(_) => Err("Error writing details".to_string()),
    }
}
pub fn insert_worker(name: &str, dni: &str, phone: &str, address: &str, salary: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO workers (name, dni, phone, address, salary) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, dni, phone, address, salary],
    ) {
        Ok(_) => Ok("Worker added successfully".to_string()),
        Err(_) => Err("Error writing worker".to_string()),
    }
}
pub fn insert_payment(name: &str, dni: &str, date: &str, amount: f32) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO payments (name, dni, date, amount) VALUES (?1, ?2, ?3, ?4)",
        params![name, dni, date, amount],
        ) {
        Ok(_) => Ok("payment added successfully".to_string()),
        Err(_) => Err("Error writing payment".to_string()),
    }
}
pub fn insert_history(id: &str, order: Order, date: &str) -> Result<String, String> {
    // Get details with the order id
    let details = match read_details(id) {
        Ok(details) => details,
        Err(e) => return Err(format!("Err ({}) obtaining details. id: {}", e, id)),
    };
    let details_json = match serde_json::to_string(&details) {
        Ok(json) => json,
        Err(_) => return Err("Failed to serialize details to JSON".to_string()),
    };

    // Create DB connection
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO history (id, client, vehicle, concept, kilometrage, total, details, pay_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, order.customer, order.vehicle, order.concept, order.kilometrage, order.total, details_json, date]) 
    {
        Ok(_) => Ok("Order moved to history successfully".to_string()),
        Err(e) => Err(format!("Err ({}) writing history", e)),
    }
}

// Delete from DB
pub fn delete_budget(id: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string())
    };
    match conn.execute(
        "DELETE FROM budgets WHERE id=?1",
        params![id]
    ) {
        Ok(_) => Ok(format!("Budget {} deleted", id)),
        Err(e) => Err(e.to_string()),
    }
}
pub fn delete_order(id: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string())
    };
    match conn.execute(
        "DELETE FROM orders WHERE id=?1",
        params![id]
    ) {
        Ok(_) => Ok(format!("Order {} deleted", id)),
        Err(e) => Err(e.to_string()),
    }
}
pub fn delete_detail(id: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string())
    };
    match conn.execute(
        "DELETE FROM details WHERE id=?1",
        params![id]
    ) {
        Ok(_) => Ok(format!("Detail {} deleted", id)),
        Err(e) => Err(e.to_string())
    }
}
pub fn delete_from_history(id: &str) -> Result<String, String> {
    let path = get_db_path();
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };
    match conn.execute(
        "DELETE FROM history WHERE id=?1",
        params![id]
    ) {
        Ok(_) => Ok(format!("History id: {} deleted successfully", id)),
        Err(e) => Err(format!("Err ({}) deleting {} history", e, id))
    }
}
