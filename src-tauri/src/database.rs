use std::collections::HashMap;
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
pub fn insert_budget(id: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) -> Result<()> {
    let conn = Connection::open("/home/syltr1x/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO budgets (id, client, vehicle, concept, kilometrage, total) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, customer, vehicle, concept, kilometrage, total],
    )?;
    Ok(())
}
pub fn insert_customer(name: &str, phone: &str, tipo: &str, cuit: &str, dni: &str) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO clients (name, phone, cuil, dni, tipo) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![name, phone, tipo, cuit, dni],
    )?;
    Ok(())
}
pub fn insert_vehicles(domain: &str, maker: &str, model: &str, tipo: &str, colour: &str, year: u8, owner: &str) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO vehicles (domain, maker, model, tipo, colour, year, owner) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![domain, maker, model, tipo, colour, year, owner],
    )?;
    Ok(())
}
pub fn update_vehicles(domain: &str, owner: &str) -> Result <()>{
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "UPDATE vehicles SET owner='?1' WHERE domain='?2'",
        params![owner, domain]
    )?;
    Ok(())
}
pub fn insert_item(id: &str, name: &str, price: f32, tipo: &str, manufacturer: &str, supplier: &str, model: &str, stock: u16) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO inventory (id, name, price, tipo, manufacturer, supplier, model, stock) VALUES (?1, ?2, ?3, ?4, ?5, ?7, ?8)",
        params![id, name, price, tipo, manufacturer, supplier, model, stock],
    )?;
    Ok(())
}
pub fn update_balance(date: &str, tipo: &str, amount: f32, name: &str) -> Result<String, String> {
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT balance (date, tipo, amount, name) VALUES (?1, ?2, ?3, ?4)",
        params![date, tipo, amount, name],
    ) {
        Ok(_) => Ok("Balance updated successfully".to_string()),
        Err(_) => Err("Error updating balance".to_string())
    }
}
pub fn insert_order(id: &str, client: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, paid: f32) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO orders (id, client, vehicle, concept, kilometrage, total, paid) VALUES (?1, ?2, ?3, ?4)",
        params![id, client, vehicle, concept, kilometrage, total, paid],
    )?;
    Ok(())
}
pub fn read_all_budgets() -> Result<Vec<HashMap<String, String>>, String> {
    let conn = match Connection::open("/home/syltr1x/work_dir/Punto_Diesel/src/debug.db") {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
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

        // Convertir 'kilometrage' correctamente, manejando 'REAL' como f32
        let kilometrage: f32 = row.get(4)?;  // Asegúrate de que la columna 'kilometrage' sea un f32
        let total: f32 = row.get(5)?;  // Asumiendo que 'total' también es un f32

        // Insertar los valores en el HashMap
        map.insert("id".to_string(), id);
        map.insert("customer".to_string(), customer);
        map.insert("vehicle".to_string(), vehicle);
        map.insert("concept".to_string(), concept);
        map.insert("kilometrage".to_string(), kilometrage.to_string());  // Convertir a string si es necesario
        map.insert("total".to_string(), total.to_string());  // Convertir a string si es necesario
        Ok(map)
    }) {
        Ok(iter) => iter,
        Err(_) => return Err("Failed to execute query".to_string()),
    };

    let mut budgets: Vec<HashMap<String, String>> = Vec::new();

    for budget in budgets_iter {
        match budget {
            Ok(b) => { 
                println!("Processed budget: {:?}", b);
                budgets.push(b)},
            Err(e) => {
                println!("Error processing row");
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(budgets)
} 
pub fn read_all_details(id: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let conn = match Connection::open("/home/syltr1x/work_dir/Punto_Diesel/src/debug.db") {
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
            Ok(b) => { 
                details.push(b)},
            Err(e) => {
                return Err(format!("Failed to process row {}", e).to_string())},
        }
    };
    Ok(details)
} 
pub fn read_budget(id: &str) -> Result<Budget, String> {
    let conn = match Connection::open("/home/syltr1x/work_dir/Punto_Diesel/src/debug.db") {
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
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
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
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    let mut stmt = match conn.prepare("SELECT * FROM details WHERE id=?") {
        Ok(stmt) => stmt,
        Err(_) => return Err("Failed to prepare SQL statement".to_string()),
    };
    let details_iter = match stmt.query_map([id], |row| {
        let mut map = HashMap::new();
        map.insert("id".to_string(), row.get(0)?);
        map.insert("item".to_string(), row.get(1)?);
        map.insert("price".to_string(), row.get(2)?);
        map.insert("cant".to_string(), row.get(3)?);
        map.insert("tipo".to_string(), row.get(4)?);
        map.insert("subtotal".to_string(), row.get(5)?);
        map.insert("iva".to_string(), row.get(6)?);
        map.insert("total".to_string(), row.get(7)?);
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
    let conn = match Connection::open("/home/syltr1x/work_dir/Punto_Diesel/src/debug.db") {
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
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
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
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
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
        Err(_) => return Err("Error obtaining details".to_string()),
    };
    let details_json = match serde_json::to_string(&details) {
        Ok(json) => json,
        Err(_) => return Err("Failed to serialize details to JSON".to_string()),
    };

    // Create DB connection
    let conn = match Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db") {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to open database connection".to_string()),
    };
    match conn.execute(
        "INSERT INTO history (id, client, vehicle, concept, kilometrage, total, details, pay_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, order.customer, order.vehicle, order.concept, order.kilometrage, order.total, details_json, date]) 
    {
        Ok(_) => Ok("Order moved to history successfully".to_string()),
        Err(_) => Err("Error writing history".to_string()),
    }
}
