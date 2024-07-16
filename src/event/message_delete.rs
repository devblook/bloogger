use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, GuildId, MessageId},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage},
    client::Context,
};
use tracing::{debug, error, instrument};

use crate::{
    channel, colors::Colors, event::Event, texts::Texts as GlobalTexts, utils::text::into_blocks,
};

const MAX_FIELD_SIZE: usize = 1024;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Texts {
    description: String,
    content: String,
    date: String,
    id: String,
    id_body: String,
}

impl Default for Texts {
    fn default() -> Self {
        Self {
            description: String::from("Message deleted in %channel%"),
            content: String::from("Content N.%i%"),
            date: String::from("Date"),
            id: String::from("ID"),
            id_body: String::from("```toml\nUser = %user_id%\nMessage = %message_id%\n```"),
        }
    }
}

#[instrument(skip(ctx, channel_id, message_id, guild_id, texts))]
pub async fn message_delete_event(
    ctx: Context,
    channel_id: ChannelId,
    message_id: MessageId,
    guild_id: Option<GuildId>,
    texts: &GlobalTexts,
) {
    let timestamp = chrono::Utc::now().timestamp();

    let Some(guild_id) = guild_id else {
        debug!("Message deleted in DMs.");
        return;
    };

    let (content, author) = {
        let Some(message) = ctx.cache.message(channel_id, message_id) else {
            debug!("Message was not in cache.");
            return;
        };

        if message.author.bot {
            debug!("The message author was a bot.");
            return;
        }

        (message.content.clone(), message.author.clone())
    };

    let Ok(Some(config_id)) = channel::get(&ctx, guild_id, Event::MessageDelete).await else {
        debug!(
            "ChannelId of guild '{}' was either None or Err.",
            guild_id.get()
        );
        return;
    };

    let channel = {
        let Some(guild) = guild_id.to_guild_cached(&ctx) else {
            debug!("Failed to get cached guild '{}'.", guild_id.get());
            return;
        };

        let Some(channel) = guild.channels.get(&ChannelId::from(config_id)) else {
            debug!(
                "Failed to get guild '{}' channel '{config_id}'.",
                guild_id.get()
            );
            return;
        };

        channel.clone()
    };

    let mut embed_author = CreateEmbedAuthor::new(
        author
            .nick_in(&ctx.http, guild_id)
            .await
            .unwrap_or(author.global_name.clone().unwrap_or(author.name.clone())),
    );

    if let Some(avatar) = author.avatar_url() {
        embed_author = embed_author.icon_url(avatar);
    }

    let description = texts
        .message_delete
        .description
        .replace("%channel%", &format!("<#{}>", channel_id.get()));

    let id_body = texts
        .message_delete
        .id_body
        .replace("%user_id%", &author.id.get().to_string())
        .replace("%message_id%", &message_id.get().to_string());

    let mut fields = into_blocks(&content, MAX_FIELD_SIZE)
        .into_iter()
        .enumerate()
        .map(|(i, content)| {
            (
                texts
                    .message_delete
                    .content
                    .replace("%i%", &(i + 1).to_string()),
                content,
                false,
            )
        })
        .collect::<Vec<_>>();

    fields.push((
        texts.message_delete.date.clone(),
        format!("<t:{timestamp}:F>"),
        false,
    ));
    fields.push((texts.message_delete.id.clone(), id_body, false));

    if let Err(err) = channel
        .send_message(
            &ctx.http,
            CreateMessage::default().embed(
                CreateEmbed::default()
                    .color(Colors::PRIMARY)
                    .author(embed_author)
                    .description(description)
                    .fields(fields),
            ),
        )
        .await
    {
        error!("Failed to send message: {err}");
    }
}
