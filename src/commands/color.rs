use std::vec;

use palette::Srgb;
use poise::CreateReply;
use serenity::{all::Colour, builder::*, model::id::RoleId};

use crate::{db_helper::*, poise_boilerplate::*, role_helper};

#[poise::command(slash_command, prefix_command)]
pub async fn color(ctx: Context<'_>, color: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn: tokio::sync::MutexGuard<'_, rusqlite::Connection> = ctx.data().conn.lock().await;
    let guild_id = ctx.guild_id();
    let author = ctx.author_member().await;
    let rgb_color: Srgb<u8> = color.parse().unwrap();
    let hex_color: String = format!("#{:x}", Srgb::<u8>::from(rgb_color));

    // need this to be a server, otherwise TODO: tell user that its not a server
    if let Some(guild_id) = guild_id {
        let roles = guild_id.roles(&ctx.http()).await?;
        let role_exists = if_role_exists(&conn, &hex_color).unwrap();

        // create discord role if doesnt exist
        let role = if !role_exists {
            role_helper::create_role(rgb_color, guild_id, &ctx, &conn, &hex_color).await?
        } else {
            // fetch existing role instead
            roles
                .values()
                .find(|r| r.name == hex_color)
                .cloned()
                .ok_or_else(|| serenity::Error::Other("Role not found"))?
        };

        let role_result = role;
        if !role_exists {}
        if let Some(author) = author {
            let user = if_user_exists(&conn, author.user.id).unwrap();
            if !user {
                let _ = add_user(&conn, author.user.id, role_result.id);
            } else {
                let user_previous_role = get_user_role(&conn, author.user.id).unwrap();

                if let Some(user_previous_role) = user_previous_role {
                    let user_previous_color: String = user_previous_role.1.parse().unwrap();
                    let user_previous_role: u64 = user_previous_role.0.parse().unwrap();
                    if get_role_users_count(&conn, RoleId::from(user_previous_role)).unwrap() == 1
                        && user_previous_color != hex_color
                    {
                        guild_id
                            .delete_role(ctx.http(), user_previous_role)
                            .await
                            .ok();
                        let _ = remove_role(&conn, RoleId::from(user_previous_role));
                    }
                    author
                        .remove_role(ctx.http(), user_previous_role)
                        .await
                        .ok();
                    let _ = edit_user(&conn, author.user.id, role_result.id);
                }
            }

            author.add_role(ctx.http(), role_result.id).await.ok();
        }
    }

    let embed = CreateEmbed::new()
        .description(format!("Succesfully set color to {}", hex_color))
        .color(Colour::from_rgb(
            rgb_color.red,
            rgb_color.green,
            rgb_color.blue,
        ));

    ctx.send(CreateReply {
        content: None,
        embeds: vec![embed],
        attachments: vec![],
        ephemeral: Some(true),
        components: None,
        allowed_mentions: None,
        reply: true,
        __non_exhaustive: (),
    })
    .await?;

    Ok(())
}
