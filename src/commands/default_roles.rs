use crate::{poise_boilerplate::*, db_helper::*};


#[poise::command(slash_command)]
pub async fn setdefaultcolor(ctx: Context<'_>, color: String) -> Result<(), Error> {
    let conn = ctx.data().conn.lock().await;
    if_exists(&conn, "#ffffff").unwrap();

    ctx.say("You said: ".to_string() + color.as_str()).await?;
    Ok(())
}
