use rusqlite::{Connection, Result, params};

pub fn connect(db_path: String) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open(db_path+".db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS color_role (
            role_id INTEGER PRIMARY KEY,
            name    TEXT
        )",
        (), // empty list of parameters.
    )?;

    Ok(conn)
}

pub fn if_exists(conn: tokio::sync::MutexGuard<'_, rusqlite::Connection>, name: &str) -> Result<(bool, String), Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare("SELECT role_id FROM color_role WHERE name = (?1)")?;
    // let res = stmt.execute([name]).unwrap();
    let res: String = stmt.query_row([1], |row| row.get(0))?;
    println!("{:?}", res);
    Ok((true, "".to_string()))
}

pub fn add_role(conn: tokio::sync::MutexGuard<'_, rusqlite::Connection>, role_id: u64, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO color_role (role_id, name) VALUES (?1, ?2)")?;
    println!("{:?}", stmt.execute(params![role_id, name]));
    Ok(())
}