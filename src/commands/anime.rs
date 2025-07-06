use std::sync::Arc;

use poise::{CreateReply, serenity_prelude::CreateAttachment};
use reqwest::Client;

use crate::types::{Context, Error};

#[poise::command(slash_command, subcommands("angry", "baka"))]
pub async fn anime(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Random angry GIF
#[poise::command(slash_command)]
pub async fn angry(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let waifu = Arc::clone(&ctx.data().waifu);

    match waifu.angry().await {
        Ok(data) => {
            let bytes = Client::new().get(data.url).send().await?.bytes().await?;
            let attachment = CreateAttachment::bytes(bytes, "angry.gif");
            let reply = CreateReply::default().attachment(attachment);
            ctx.send(reply).await?;
        }
        Err(_) => {
            ctx.reply("Failed to get angry GIF").await?;
        }
    };

    Ok(())
}

/// Random baka GIF
#[poise::command(slash_command)]
pub async fn baka(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let waifu = Arc::clone(&ctx.data().waifu);

    match waifu.baka().await {
        Ok(data) => {
            let bytes = Client::new().get(data.url).send().await?.bytes().await?;
            let attachment = CreateAttachment::bytes(bytes, "angry.gif");
            let reply = CreateReply::default().attachment(attachment);
            ctx.send(reply).await?;
        }
        Err(_) => {
            ctx.reply("Failed to get baka GIF").await?;
        }
    };

    Ok(())
}
