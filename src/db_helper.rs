use rusqlite::{Connection, Result};

pub fn connect(db_path: String) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open(db_path+".db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS color_role (
            role_id TEXT PRIMARY KEY,
            name    TEXT
        )",
        (), // empty list of parameters.
    )?;

    Ok(conn)
}

pub fn if_exists(conn: tokio::sync::MutexGuard<'_, rusqlite::Connection>, name: &str) -> Result<bool, Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare("SELECT role_id FROM color_role WHERE name = (?1)")?;
    println!("{:?}", stmt.execute([name]).unwrap());

    Ok(true)
}

pub fn add_role(conn: tokio::sync::MutexGuard<'_, rusqlite::Connection>, role_id: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO color_role (role_id), (name) VALUES (?1), (?2)")?;
    let _ = stmt.execute([role_id, name]);
    Ok(())
}