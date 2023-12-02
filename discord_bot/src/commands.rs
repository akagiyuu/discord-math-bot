use crate::Data;
use anyhow::{Error, Result};
use core::{OutputFormat, eval};
use poise::serenity_prelude::AttachmentType;
use std::path::Path;

pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Evaluate an expression using mathematica
///
/// Usgae: =eval "`expression`"
/// * `expression`: visit `https://reference.wolfram.com/language/` for more information
#[poise::command(prefix_command, track_edits)]
pub async fn eval(
    ctx: Context<'_>,
    #[description = "A valid mathematica expression"] expression: String,
) -> Result<()> {
    let result_image_path = eval(expression, OutputFormat::Image);
    let result_image_abs_path = format!("/tmp/{}", result_image_path);
    ctx.send(|reply| {
        reply.embed(|embed| {
            embed.image(format!("attachment://{}", result_image_path));
            embed
        });
        reply.attachment(AttachmentType::Path(Path::new(&result_image_abs_path)));
        reply
    })
    .await?;
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<()> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}
