use dotenv;
use std::{env, sync::{Arc}};

use poise::serenity_prelude::*;
use rusqlite::{Result};
use tokio;
use tokio::sync::Mutex;

mod message_handler;
mod poise_boilerplate;
mod db_helper;
mod commands {
    pub mod color;
    pub mod default_roles;
}


use poise_boilerplate::*;
use message_handler::Handler;
use commands::{color::color as color, default_roles::setdefaultcolor};
use db_helper::*;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db_path = env::var("DB_PATH_NAME").expect("Expected a db path+name in the environment");

    let data = Data {
        conn: Arc::new(Mutex::new(connect(db_path).expect("Unable to estabilish database connection")))
    };

    

    
    let token = env::var("DISCORD_AUTH_TOKEN").expect("Expected a token in the environment");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![color(), setdefaultcolor()],
            manual_cooldowns: true,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
