use rusqlite::{Connection, OptionalExtension, Result};

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

pub fn if_exists(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    name: &str,
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    let res: Option<String> = conn
        .query_row(
            "SELECT role_id FROM color_role WHERE name = ?1",
            [name],
            |row| row.get(0),
        )
        .optional()?;
    
    match res {
        Some(role_id) => Ok((true, role_id)),
        None => Ok((false, String::new())),
    }
}

pub fn add_role(conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, role_id: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO color_role (role_id, name) VALUES (?1, ?2)")?;
    println!("{:?}", stmt.execute([role_id.to_string(), name.to_string()]));
    Ok(())
}