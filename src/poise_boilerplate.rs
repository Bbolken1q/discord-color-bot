use std::sync::{Arc};
use tokio::sync::Mutex;

use rusqlite::Connection;

pub struct Data {
    pub conn: Arc<Mutex<Connection>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;