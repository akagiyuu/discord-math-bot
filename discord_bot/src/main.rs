mod commands;

use anyhow::Error;
use poise::serenity_prelude as serenity;
use std::{env::var, time::Duration};

pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let options = poise::FrameworkOptions {
        commands: vec![commands::help(), commands::eval()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("=".to_string()),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        ..Default::default()
    };

    poise::Framework::builder()
        .token(token)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(intents)
        .run()
        .await
        .unwrap();
}
