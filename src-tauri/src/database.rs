use rusqlite::{params, Connection, Result};

pub fn insert_budget(customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) -> Result<()> {
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
pub fn insert_vehicles(domain: &str, maker: &str, model: &str, tipo: &str, colour: &str, year: u8, owner: &str) {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO vehicles (domain, maker, model, tipo, colour, year, owner) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![domain, maker, model, tipo, colour, year, owner],
    )?;
    Ok(())
}
pub fn update_vehicles(domain: &str, owner: &str) {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        format!("UPDATE vehicles SET owner='{}' WHERE domain='{}'" owner, domain)
    )?;
}
pub fn insert_item(id, name, price, tipo, manufacturer, supplier, model, stock) {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO inventory (id, name, price, tipo, manufacturer, supplier, model, stock) VALUES (?1, ?2, ?3, ?4, ?5, ?7, ?8)",
        params![id, name, price, tipo, manufacturer, supplier, model, stock],
    )?;
    Ok(())
}
