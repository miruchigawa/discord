use crate::types::{Context, Error};

/// Reply pong
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Pong!").await?;
    Ok(())
}
