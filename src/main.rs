use std::{env, sync::Arc};

use anyhow::Result;
use dotenvy::dotenv;
use poise::{
    Framework, FrameworkOptions, builtins,
    serenity_prelude::{ClientBuilder, GatewayIntents},
};
use tracing::info;

use discord::{
    commands::{anime, ping, wfx},
    types::Data,
    utils::{sd, waifu::Waifu},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged();
    let waifu = Waifu::new(env::var("WAIFUIT_TOKEN")?)?;
    let sd = sd::Client::new(env::var("STABLE_DIFUSION_URL")?)?;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![ping(), anime(), wfx()],
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Client connected as {}", ready.user.name);
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    waifu: Arc::new(waifu),
                    sd: Arc::new(sd),
                })
            })
        })
        .build();

    ClientBuilder::new(token, intents)
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}
