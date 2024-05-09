use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, Message, MessageUpdateEvent},
    builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage},
    client::Context,
};
use tracing::{debug, error, instrument};

use crate::{channel, colors::Colors, event::Event, texts::Texts as GlobalTexts};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Texts {
    description: String,
    now: String,
    previous: String,
    date: String,
    id: String,
    id_body: String,
}

impl Default for Texts {
    fn default() -> Self {
        Self {
            description: String::from("Updated their [message](%link%) in %channel%"),
            now: String::from("Now"),
            previous: String::from("Previous"),
            date: String::from("Date"),
            id: String::from("ID"),
            id_body: String::from("```toml\nUser = %user_id%\nMessage = %message_id%\n```"),
        }
    }
}

#[instrument(skip(ctx, old_if_available, new, event, texts))]
pub async fn message_update_event(
    ctx: Context,
    old_if_available: Option<Message>,
    new: Option<Message>,
    event: MessageUpdateEvent,
    texts: &GlobalTexts,
) {
    let timestamp = chrono::Utc::now().timestamp();

    let Some(guild_id) = event.guild_id else {
        debug!("Message updated in DMs.");
        return;
    };

    let Some(old) = old_if_available else {
        debug!("Old message data was not present.");
        return;
    };

    let Some(new) = new else {
        debug!("New message data was not present.");
        return;
    };

    if new.content == old.content {
        debug!("The content of the old message was the same as the content of the new message.");
        return;
    }

    if new.author.bot {
        debug!("The message author was a bot.");
        return;
    }

    let Ok(Some(config_id)) = channel::get(&ctx, guild_id, Event::MessageUpdate).await else {
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
        new.author.nick_in(&ctx.http, guild_id).await.unwrap_or(
            new.author
                .global_name
                .clone()
                .unwrap_or(new.author.name.clone()),
        ),
    );

    if let Some(avatar) = new.author.avatar_url() {
        embed_author = embed_author.icon_url(avatar);
    }

    let description = texts
        .message_update
        .description
        .replace("%channel%", &format!("<#{}>", event.channel_id.get()))
        .replace("%link%", &new.link());

    let id_body = texts
        .message_update
        .id_body
        .replace("%user_id%", &new.author.id.get().to_string())
        .replace("%message_id%", &new.id.get().to_string());

    if let Err(err) = channel
        .send_message(
            &ctx.http,
            CreateMessage::default().embed(
                CreateEmbed::default()
                    .color(Colors::PRIMARY)
                    .author(embed_author)
                    .description(description)
                    .fields([
                        (&texts.message_update.now, new.content, false),
                        (&texts.message_update.previous, old.content, false),
                        (
                            &texts.message_update.date,
                            format!("<t:{timestamp}:F>"),
                            false,
                        ),
                        (&texts.message_update.id, id_body, false),
                    ]),
            ),
        )
        .await
    {
        error!("Failed to send message: {err}");
    }
}
