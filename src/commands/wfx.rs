use std::sync::Arc;

use poise::{
    ChoiceParameter, CreateReply,
    serenity_prelude::{
        CreateAttachment, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp,
    },
};

use crate::{
    types::{Context, Error},
    utils::sd::GenerateBody,
};

#[derive(ChoiceParameter)]
enum GuideTask {
    #[name = "Prompt Guide"]
    Prompt,
}

#[poise::command(slash_command, subcommands("dream", "guide"))]
pub async fn wfx(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Generate images using AnimagineXL 4
#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command)]
pub async fn dream(
    ctx: Context<'_>,
    #[description = "Describe the image to be generated in the danbooru tag (see how in the wfx guide)"]
    prompt: String,
    #[description = "List of unwanted features tags"] negative_prompt: Option<String>,
    #[description = "Output width (pixels). Must be a multiple of 8. (default: 1024)"]
    width: Option<u16>,
    #[description = "Output height (pixels). Must be a multiple of 8. (default: 1024)"]
    height: Option<u16>,
    #[description = "RNG seed. Use `-1` for a random seed chosen by the server. (default: -1)"]
    seed: Option<i64>,
    #[description = "Number of denoising steps. (default: 25)"] steps: Option<u8>,
    #[description = "Higher values follow the prompt more strictly but may reduce diversity. (default: 7.0)"]
    cfg_scale: Option<f32>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let sd = Arc::clone(&ctx.data().sd);
    let payload = GenerateBody {
        prompt,
        negative_prompt: negative_prompt.unwrap_or("lowres, bad anatomy, bad hands, text, error, missing finger, extra digits, fewer digits, cropped, worst quality, low quality, low score, bad score, average score, signature, watermark, username, blurry".to_string()),
        width: width.unwrap_or(1024),
        height: height.unwrap_or(1024),
        steps: steps.unwrap_or(25),
        seed: seed.unwrap_or(-1),
        cfg_scale: cfg_scale.unwrap_or(7.0),
    };

    let message = match sd.generate(payload).await {
        Ok(content) => {
            if content.images.is_empty() {
                ctx.reply("Unexpected error, expected > 1 but got 0")
                    .await?;
                return Ok(());
            }

            let author = ctx.author();
            let attachment = CreateAttachment::bytes(&*content.images[0], "image.png");
            let embed = CreateEmbed::new()
                .title("🌟✨ Image Created! ✨🌟")
                .description("*:･ﾟ✧ Your magical creation has come to life! ✧ﾟ･:*")
                .field("🎨 Image Prompt ୨୧", content.info.prompt, false)
                .field(
                    "🌙 Negative Enchantments ❀",
                    content.info.negative_prompt,
                    false,
                )
                .field(
                    "🎐 Settings ⋆｡˚",
                    format!(
                        "✧ Steps: {}\n🌈 CFG Scale: {}\n🎲 Seed: {}\n📐 Canvas: {}x{}",
                        content.info.steps,
                        content.info.cfg_scale,
                        content.info.seed,
                        content.info.width,
                        content.info.height
                    ),
                    false,
                )
                .field(
                    "⭐ Model ੭",
                    format!(
                        "{} ({})",
                        content.info.sd_model_name, content.info.sd_model_hash
                    ),
                    false,
                )
                .author(
                    CreateEmbedAuthor::new(format!("⋆｡˚ Requested by {} ⋆｡˚", author.name))
                        .icon_url(author.avatar_url().unwrap_or_default()),
                )
                .attachment(&attachment.filename)
                .color(0xFFD6A5)
                .timestamp(Timestamp::now());

            CreateReply::default()
                .embed(embed)
                .attachment(attachment)
                .reply(true)
        }

        Err(e) => {
            tracing::error!("error found on wfx dream: {}", e);
            CreateReply::default()
                .content("Unexpected error, try again later")
                .reply(true)
        }
    };

    ctx.send(message).await?;
    Ok(())
}

/// Usage guidelines
#[poise::command(slash_command)]
pub async fn guide(
    ctx: Context<'_>,
    #[description = "Section"] section: GuideTask,
) -> Result<(), Error> {
    let footer = CreateEmbedFooter::new("🌈 Happy creating~! 🖌️");
    let reply = match section {
        GuideTask::Prompt => {
            let attachment = CreateAttachment::path("./assets/images/prompt_guide.png").await?;
            let embed = CreateEmbed::new()
                .title("💫✨ Prompt Guide ✨💫")
                .description("Need a little ✨ magic ✨ in your prompt crafting?\n\n🌷 Here's your go-to guide!\n\n🔗 [Open the guide here](https://cagliostrolab.net/posts/optimizing-animagine-xl-40-in-depth-guideline-and-update)")
                .color(0xFFD6A5)
                .attachment(&attachment.filename)
                .footer(footer)
                .timestamp(Timestamp::now());
            CreateReply::default()
                .embed(embed)
                .reply(true)
                .attachment(attachment)
        }
    };

    ctx.send(reply).await?;
    Ok(())
}
