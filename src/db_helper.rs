use rusqlite::{Connection, OptionalExtension, Result};
use serenity::all::{RoleId, UserId};

pub fn connect(db_path: String) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open(db_path+".db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS color_role ( 
            role_id TEXT PRIMARY KEY,
            name    TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users ( 
            user_id TEXT PRIMARY KEY NOT NULL,
            role_id TEXT NOT NULL,
            FOREIGN KEY(role_id) REFERENCES color_role(role_id)    
        )",
        (), // empty list of parameters.
    )?;

    Ok(conn)
}

// roles

pub fn if_role_exists(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let res: Option<String> = conn
        .query_row(
            "SELECT role_id FROM color_role WHERE name = ?1",
            [name],
            |row| row.get(0),
        )
        .optional()?;
    
    match res {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn add_role(conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, role_id: RoleId, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO color_role (role_id, name) VALUES (?1, ?2)")?;
    let _ = stmt.execute([role_id.to_string(), name.to_string()]);
    Ok(())
}

pub fn get_role(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let res: Option<String> = conn
        .query_row(
            "SELECT role_id FROM color_role WHERE name = ?1",
            [name],
            |row| row.get(0),
        )
        .optional()?;
    Ok(res)
}

pub fn remove_role(conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, role_id: RoleId) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("DELETE FROM color_role WHERE role_id = ?1)")?;
    let _ = stmt.execute([role_id.to_string()]);
    Ok(())
}


// users

pub fn if_user_exists(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    user_id: UserId,
) -> Result<bool, Box<dyn std::error::Error>> {
    let res: Option<String> = conn
        .query_row(
            "SELECT role_id FROM users WHERE user_id = ?1",
            [user_id.to_string()],
            |row| row.get(0),
        )
        .optional()?;
    
    match res {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}


// adds user to database
// TODO: make everything server based
pub fn add_user(conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, user_id: UserId, role_id: RoleId) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO users (user_id, role_id) VALUES (?1, ?2)")?;
    let _ = stmt.execute([user_id.to_string(), role_id.to_string()]);
    Ok(())
}

pub fn edit_user(conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, user_id: UserId, role_id: RoleId) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("UPDATE users SET role_id = ?1 WHERE user_id = ?2")?;
    let _ = stmt.execute([role_id.to_string(), user_id.to_string()]);
    Ok(())
}

pub fn get_user_role(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    user_id: UserId,
) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    let res: Option<(String, String)> = conn
        .query_row(
            "SELECT users.role_id, color_role.name FROM users, color_role WHERE user_id = ?1 AND users.role_id = color_role.role_id",
            [user_id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
    Ok(res)
}

pub fn get_role_users_count(
    conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>,
    role_id: RoleId,
) -> Result<u32, Box<dyn std::error::Error>> {
    let res: Option<u32> = conn
        .query_row(
            "SELECT COUNT(*) FROM users WHERE role_id = ?1",
            [role_id.to_string()],
            |row| row.get(0),
        ).optional()?;
        
    match res {
        Some(res) => Ok(res),
        None => Ok(0),
    }
}