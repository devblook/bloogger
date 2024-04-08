use std::env;

use data::Data;
use poise::{samples::register_globally, Framework, FrameworkOptions};
use serenity::{all::GatewayIntents, cache::Settings, Client};
use tracing::{error, info, instrument};

use handler::Handler;

mod cache;
mod colors;
mod config;
mod data;
mod handler;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command, guild_only, subcommands())]
pub async fn set(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[instrument]
pub async fn init() {
    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(err) => {
            error!("Missing discord token: {err}");
            return;
        }
    };

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::<(), Box<dyn std::error::Error + Send + Sync>>::builder()
        .options(FrameworkOptions {
            commands: vec![set()],
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                info!("Registering commands globally...");
                if let Err(err) = register_globally(&ctx.http, &framework.options().commands).await
                {
                    error!("Failed to register commands: {err}");
                } else {
                    info!("Commands registered.");
                };
                Ok(())
            })
        })
        .build();

    let mut settings = Settings::default();
    settings.max_messages = 500;

    let client = Client::builder(token, intents)
        .framework(framework)
        .cache_settings(settings)
        .event_handler(Handler)
        .await;

    if let Err(err) = client {
        error!("Failed to create client: {err}");
        return;
    }

    let mut client = client.expect("Err case was handled earlier.");

    {
        let mut data = client.data.write().await;
        data.insert::<Data>(Data::default());
    }

    if let Err(err) = client.start().await {
        error!("Client error: {err}");
    }

    let data = client.data.read().await;
    let data = data.get::<Data>().expect("Data should never be none.");
    data.cache.run_pending_tasks().await;
}
