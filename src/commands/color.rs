use palette::Srgb;
use serenity::{
    all::{Colour},
    builder::*,
};

use crate::{poise_boilerplate::*};

#[poise::command(slash_command, prefix_command)]
pub async fn color(ctx: Context<'_>, color: String) -> Result<(), Error> {
    let conn = ctx.data().conn.lock().await;
    let guild_id = ctx.guild_id();
    let author = ctx.author_member().await;
    
    if let Some(guild_id) = guild_id {
        let rgb_color: Srgb<u8> = color.parse().unwrap();
        let builder = EditRole::new()
            .name(format!("#{:x}", Srgb::<u8>::from(rgb_color)))
            .colour(Colour::from_rgb(
                rgb_color.red,
                rgb_color.green,
                rgb_color.blue,
            ))
            .mentionable(false);
        let role_result = guild_id.create_role(&ctx.http(), builder).await.ok();

        if let Some(role_result) = role_result {
            if let Some(author) = author {
                author.add_role(ctx.http(), role_result.id).await.ok();
            }
        }
    }

    ctx.say("You said: ".to_string() + color.as_str()).await?;
    Ok(())
}
