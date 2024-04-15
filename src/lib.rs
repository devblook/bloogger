use std::{env, sync::Arc};

use poise::{samples::register_globally, Framework, FrameworkOptions};
use serenity::{all::GatewayIntents, cache::Settings, Client};
use tracing::{error, info, instrument};

use data::Data;
use handler::Handler;

mod cache;
mod colors;
mod config;
mod data;
mod handler;
mod messages;
mod texts;

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

    let data = Arc::new(match Data::new() {
        Ok(data) => data,
        Err(err) => {
            error!("Failed to create Data: {err:?}");
            return;
        }
    });

    let cloned_data = data.clone();

    let runtime = tokio::runtime::Handle::current();
    let shard_manager = Arc::downgrade(&client.shard_manager);
    let handler = ctrlc::set_handler(move || {
        runtime.block_on(async {
            shard_manager.upgrade().unwrap().shutdown_all().await;
            cloned_data.cache.invalidate_all();
            cloned_data.cache.run_pending_tasks().await;
        });
    });

    if let Err(err) = handler {
        error!("Failed to set CtrlC handler: {err}");
    }

    {
        let mut client_data = client.data.write().await;
        client_data.insert::<Data>(data);
    }

    if let Err(err) = client.start().await {
        error!("Client error: {err}");
    }
}
