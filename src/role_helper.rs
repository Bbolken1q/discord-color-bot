use palette::Srgb;
use serenity::{builder::EditRole, model::{Colour, id::GuildId}};

use crate::poise_boilerplate::{Context};
use crate::db_helper;


// creates discord role and adds it to db
pub async fn create_role(rgb_color: Srgb<u8>, guild_id: GuildId, ctx: &Context<'_>, conn: &tokio::sync::MutexGuard<'_, rusqlite::Connection>, hex_color: &String) -> Result<poise::serenity_prelude::Role, serenity::Error> {
    let builder = EditRole::new()
        .name(format!("#{:x}", Srgb::<u8>::from(rgb_color)))
        .colour(Colour::from_rgb(
            rgb_color.red,
            rgb_color.green,
            rgb_color.blue,
        ))
        .mentionable(false);
    let role_result =  guild_id.create_role(ctx.http(), builder).await.ok();

    if let Some(role_result) = role_result {
        let _ = db_helper::add_role(conn, role_result.id, hex_color.as_str());
        return Ok(role_result)
    } else {
        return Err(serenity::Error::Other("Role not found"))
    }
 
}
pub fn delete_role() {}
pub fn add_role() {}
pub fn remove_role() {}
pub fn get_role() {}
pub fn get_user_server_roles() {}
