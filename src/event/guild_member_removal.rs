use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, CreateEmbedFooter, GuildId, Member, Mentionable, User},
    builder::{CreateEmbed, CreateMessage},
    client::Context,
};
use tracing::{debug, error, instrument};

use crate::{channel, colors::Colors, event::Event, texts::Texts as GlobalTexts};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Texts {
    title: String,
    id: String,
    account_creation: String,
    member_count: String,
    joined: String,
}

impl Default for Texts {
    fn default() -> Self {
        Self {
            title: String::from("%user% left this guild"),
            id: String::from("ID"),
            account_creation: String::from("Account Creation"),
            member_count: String::from("Member Count: %count%"),
            joined: String::from("Joined"),
        }
    }
}

#[instrument(skip(ctx, guild_id, user, member, texts))]
pub async fn guild_member_removal_event(
    ctx: Context,
    guild_id: GuildId,
    user: User,
    member: Option<Member>,
    texts: &GlobalTexts,
) {
    let Ok(Some(config_id)) = channel::get(&ctx, guild_id, Event::GuildMemberRemoval).await else {
        debug!(
            "ChannelId of guild '{}' was either None or Err.",
            guild_id.get()
        );
        return;
    };

    if user.bot {
        debug!("The user who left was a bot.");
        return;
    }

    let (member_count, channel) = {
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

        (guild.member_count, channel.clone())
    };

    let mut embed = CreateEmbed::default()
        .title(
            texts
                .guild_member_removal
                .title
                .replace("%user%", user.global_name.as_ref().unwrap_or(&user.name)),
        )
        .description(user.mention().to_string())
        .color(Colors::PRIMARY)
        .field(
            &texts.guild_member_removal.id,
            format!("@{} ({})", user.name, user.id),
            true,
        )
        .field(
            &texts.guild_member_removal.account_creation,
            format!("<t:{}>", user.created_at().timestamp()),
            true,
        )
        .footer(CreateEmbedFooter::new(
            texts
                .guild_member_removal
                .member_count
                .replace("%count%", &member_count.to_string()),
        ));

    if let Some(url) = user.avatar_url() {
        embed = embed.thumbnail(url);
    }

    if let Some(Member {
        joined_at: Some(joined_at),
        ..
    }) = member
    {
        embed = embed.field(
            &texts.guild_member_removal.joined,
            format!("<t:{}:R>", joined_at.timestamp()),
            false,
        );
    }

    if let Err(err) = channel
        .send_message(&ctx.http, CreateMessage::default().embed(embed))
        .await
    {
        error!("Failed to send message: {err}");
    }
}
