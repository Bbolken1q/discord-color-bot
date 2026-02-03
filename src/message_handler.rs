use regex::Regex;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use std::collections::HashMap;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut words: HashMap<String, String> = HashMap::new();
        words.insert("ping".to_string(), "Pong".to_string());

        for word in words {
            let re = Regex::new((r"\b(?:".to_string() + &word.0 + r")\b").as_str()).unwrap();
            if re.is_match(&msg.content.to_lowercase()) {
                if let Err(why) = msg.reply_ping(&ctx.http, &word.1).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}
