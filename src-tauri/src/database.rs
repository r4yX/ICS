use rusqlite::{params, Connection, Result};

pub fn insert_budget(customer: &str, vehicle: &str, concept: &str, kilometrage: f32, total: f32) -> Result<()> {
    let conn = Connection::open("C:/Users/r4y/Desktop/work_dir/Punto_Diesel/src/debug.db")?;
    conn.execute(
        "INSERT INTO budgets (client, vehicle, concept, kilometrage, total) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![customer, vehicle, concept, kilometrage, total],
    )?;
    Ok(())
}