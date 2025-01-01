use rusqlite::{params, Connection, Result};

pub struct Budget  {
    // is stupid return 'id' field
    customer: &str,
    vehicle: &str,
    concept: &str,
    kilometrage: f32,
    total: f32,
}

pub fn insert_budget(id: &str, customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO budgets (client, vehicle, concept, kilometrage, total) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![customer, vehicle, concept, kilometrage, total],
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
pub fn update_balance(date: &str, tipo: &str, amount: f32, name: &str) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT balance (date, tipo, amount, name) VALUES (?1, ?2, ?3, ?4)",
        params![date, tipo, amount, name],
    )?;
    Ok(())
}
pub fn insert_order(id: &str, client: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32, paid: f32) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO orders (id, client, vehicle, concept, kilometrage, total, paid) VALUES (?1, ?2, ?3, ?4)",
        params![id, client, vehicle, concept, kilometrage, total, paid],
    )?;
    Ok(())
}
pub fn read_budget(id: &str) -> Result<Budget, String> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    let mut stmt  = conn.prepare("SELECT * FROM budget WHERE id=?")?;
    let budget = stmt.query_row(params![id], |row| {
        Ok(Budget {
            customer: row.get(1)?,
            vehicle: row.get(2)?,
            concept: row.get(3)?,
            kilometrage: row.get(4)?,
            total: row.get(5)?,
        });
    })?;
    Ok(budget);
}
