use std::{env, sync::Arc};

use poise::{samples::register_globally, Framework, FrameworkOptions};
use serenity::{all::GatewayIntents, cache::Settings, Client};
use tracing::{error, info, instrument};

use command::set;
use data::Data;
use handler::Handler;

mod cache;
mod channel;
mod colors;
mod command;
mod config;
mod data;
mod event;
mod handler;
mod messages;
mod texts;
mod utils;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), Error>;

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

    let data = Arc::new(match Data::new() {
        Ok(data) => data,
        Err(err) => {
            error!("Failed to create Data: {err:?}");
            return;
        }
    });

    let client = Client::builder(token, intents)
        .framework(framework)
        .cache_settings(settings)
        .event_handler(Handler::new(data.texts.clone()))
        .await;

    if let Err(err) = client {
        error!("Failed to create client: {err}");
        return;
    }

    let mut client = client.expect("Err case was handled earlier.");

    let handler = ctrlc::set_handler(|| {});
    if let Err(err) = handler {
        error!("Failed to set CtrlC handler: {err}");
    }

    let shard_manager = Arc::downgrade(&client.shard_manager);
    let cloned_data = data.clone();

    tokio::spawn(async move {
        loop {
            let stdin = std::io::stdin();
            let mut input = String::new();

            stdin.read_line(&mut input).expect("Failed to read line.");

            if input.trim() == "stop" {
                cloned_data.cache.invalidate_all();
                cloned_data.cache.run_pending_tasks().await;
                let _ = Data::save_texts(&cloned_data.texts);
                shard_manager.upgrade().unwrap().shutdown_all().await;
                break;
            }
        }
    });

    {
        let mut client_data = client.data.write().await;
        client_data.insert::<Data>(data);
    }

    info!("Bot started.");

    if let Err(err) = client.start().await {
        error!("Client error: {err}");
    }
}
