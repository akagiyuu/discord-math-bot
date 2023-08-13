use poise::serenity_prelude::AttachmentType;

use crate::{Context, Error};

/// Evaluate an expression using mathematica
///
/// Usgae: =eval "`expression`"
/// * `expression`: visit [https://example.com](https://example.com) for more information
#[poise::command(prefix_command, slash_command, track_edits, aliases("eval"))]
pub async fn evaluate(
    ctx: Context<'_>,
    #[description = "A valid mathematica expression"] expression: String,
) -> Result<(), Error> {
    let generated_image_path = mathematica::evaluate(&expression).expect("fail to create image");
    ctx.send(|reply| {
        reply.embed(|embed| embed.image(format!("attachment://{}", generated_image_path.file_name().unwrap().to_string_lossy().to_string())));
        reply.attachment(AttachmentType::Path(generated_image_path.as_path()));
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
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
