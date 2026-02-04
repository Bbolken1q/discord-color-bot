use palette::Srgb;
use serenity::{all::Colour, builder::*, model::id::RoleId};

use crate::{db_helper::{add_role, if_exists}, poise_boilerplate::*};

#[poise::command(slash_command, prefix_command)]
pub async fn color(ctx: Context<'_>, color: String) -> Result<(), Error> {
    let conn = ctx.data().conn.lock().await;
    let guild_id = ctx.guild_id();
    let author = ctx.author_member().await;

    if let Some(guild_id) = guild_id {
        let roles = guild_id.roles(&ctx.http()).await?;
        let rgb_color: Srgb<u8> = color.parse().unwrap();
        let role_result: Option<serenity::all::Role>;
        let role_exists = if_exists(&conn, &format!("#{:x}", Srgb::<u8>::from(rgb_color))).unwrap();
        if !role_exists.0 { // role doesnt exist
            let builder = EditRole::new()
                .name(format!("#{:x}", Srgb::<u8>::from(rgb_color)))
                .colour(Colour::from_rgb(
                    rgb_color.red,
                    rgb_color.green,
                    rgb_color.blue,
                ))
                .mentionable(false);
            role_result = guild_id.create_role(&ctx.http(), builder).await.ok();
        } else {

            println!("{:?} asd", role_exists.1);
            let role_id: u64 = role_exists.1.parse().unwrap();
            role_result = roles.get(&RoleId::from(role_id)).cloned();
        }

        if let Some(role_result) = role_result {
            if !role_exists.0 {
                let _ = add_role(&conn, role_result.id.to_string().as_str(), format!("#{:x}", Srgb::<u8>::from(rgb_color)).as_str());
            }
            if let Some(author) = author {
                author.add_role(ctx.http(), role_result.id).await.ok();
            }
        }
    }

    ctx.say("You said: ".to_string() + color.as_str()).await?;
    Ok(())
}
