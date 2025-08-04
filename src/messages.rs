use poise::CreateReply;
use serenity::builder::CreateEmbed;
use tracing::{error, instrument};

use crate::{colors::Colors, data::Data, Context};

#[instrument(skip(ctx))]
pub async fn send_ephemeral_message(ctx: &Context<'_>, color: u32, title: &str, description: &str) {
    if let Err(err) = ctx
        .send(
            CreateReply::default().ephemeral(true).embed(
                CreateEmbed::default()
                    .color(color)
                    .title(title)
                    .description(description),
            ),
        )
        .await
    {
        error!("Failed to send message: {err}");
    }
}

pub trait Messages {
    async fn not_in_guild(&self, data: &Data);
    async fn internal_error(&self, data: &Data);
    async fn not_valid_channel(&self, data: &Data);
    async fn channel_set(&self, data: &Data);
    async fn channel_unset(&self, data: &Data);
    async fn missing_administrator(&self, data: &Data);
}

impl Messages for Context<'_> {
    #[instrument(skip(self, data))]
    async fn not_in_guild(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::ERROR,
            data.texts.error_embed_title(),
            data.texts.error_command_executed_in_dm(),
        )
        .await;
    }

    #[instrument(skip(self, data))]
    async fn internal_error(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::ERROR,
            data.texts.error_embed_title(),
            data.texts.internal_error(),
        )
        .await;
    }

    #[instrument(skip(self, data))]
    async fn not_valid_channel(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::ERROR,
            data.texts.error_embed_title(),
            data.texts.not_valid_channel(),
        )
        .await;
    }

    #[instrument(skip(self, data))]
    async fn channel_set(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::PRIMARY,
            data.texts.success_embed_title(),
            data.texts.channel_set(),
        )
        .await;
    }

    #[instrument(skip(self, data))]
    async fn channel_unset(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::PRIMARY,
            data.texts.success_embed_title(),
            data.texts.channel_unset(),
        )
        .await;
    }

    #[instrument(skip(self, data))]
    async fn missing_administrator(&self, data: &Data) {
        send_ephemeral_message(
            self,
            Colors::ERROR,
            data.texts.error_embed_title(),
            data.texts.missing_administrator_permission(),
        )
        .await;
    }
}
