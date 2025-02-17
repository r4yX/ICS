use std::path::PathBuf;
use rusqlite::Connection;

pub fn create_tables(path: PathBuf) -> Result<(), String> {
    let conn = match Connection::open(path) {
        Ok(conn) => conn,
        Err(e) => return Err(e.to_string()),
    };

    match conn.execute_batch("
    CREATE TABLE IF NOT EXISTS vehicles (
        domain TEXT NOT NULL UNIQUE,
        maker TEXT NOT NULL,
        model TEXT NOT NULL,
        tipo TEXT NOT NULL,
        colour TEXT NOT NULL,
        year INT NOT NULL,
        owner TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS clients (
        name TEXT NOT NULL,
        phone TEXT NOT NULL,
        cuil TEXT NOT NULL,
        dni TEXT NOT NULL UNIQUE,
        tipo TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS inventory (
        id TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL,
        price DECIMAL(18, 2) NOT NULL,
        tipo TEXT NOT NULL,
        manufacturer TEXT NOT NULL,
        supplier TEXT NOT NULL,
        model TEXT NOT NULL,
        stock INTEGER DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS workers (
        name TEXT NOT NULL,
        dni TEXT NOT NULL UNIQUE,
        phone TEXT NOT NULL,
        address TEXT NOT NULL,
        salary DECIMAL(18, 2) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS payments (
        name TEXT NOT NULL,
        dni TEXT NOT NULL,
        date DATETIME NOT NULL,
        amount DECIMAL(18, 2) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS budgets (
        id TEXT UNIQUE NOT NULL,
        client TEXT NOT NULL,
        vehicle TEXT NOT NULL,
        concept TEXT NOT NULL,
        kilometrage DECIMAL(18, 2) NOT NULL,
        total DECIMAL(18, 2) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS details (
        id INTEGER NOT NULL,
        item TEXT NOT NULL,
        price DECIMAL(18, 2) NOT NULL,
        cant INT DEFAULT 1,
        tipo TEXT NOT NULL,
        subtotal DECIMAL(18, 2) NOT NULL,
        iva DECIMAL(18, 2) NOT NULL,
        total DECIMAL(18, 2) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS orders (
        id TEXT UNIQUE NOT NULL,
        client TEXT NOT NULL,
        vehicle TEXT NOT NULL,
        concept TEXT NOT NULL,
        kilometrage DECIMAL(9, 2) NOT NULL,
        total DECIMAL(18, 2) NOT NULL,
        paid DECIMAL(18, 2) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS history (
        id TEXT UNIQUE NOT NULL,
        client TEXT NOT NULL,
        vehicle TEXT NOT NULL,
        concept TEXT NOT NULL,
        kilometrage DECIMAL(9, 2) NOT NULL,
        total DECIMAL(18, 2) NOT NULL,
        details LONGTEXT NOT NULL,
        pay_date DATETIME NOT NULL
    );
    CREATE TABLE IF NOT EXISTS balance (
        date TEXT NOT NULL,
        tipo TEXT NOT NULL,
        name TEXT NOT NULL,
        amount DECIMAL(18, 2) NOT NULL
    );
    ") {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
