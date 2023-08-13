mod error_handler;
mod commands;

use poise::serenity_prelude as serenity;
use std::{env::var, time::Duration};
use error_handler::on_error;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;


#[tokio::main]
async fn main() {
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::evaluate()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("=".to_string()),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}
