use serde::{Deserialize, Serialize};
use serenity::{
    all::{ChannelId, CreateEmbedFooter, Member, Mentionable},
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
}

impl Default for Texts {
    fn default() -> Self {
        Self {
            title: String::from("%user% joined this guild"),
            id: String::from("ID"),
            account_creation: String::from("Account Creation"),
            member_count: String::from("Member Count: %count%"),
        }
    }
}

#[instrument(skip(ctx, member, texts))]
pub async fn guild_member_addition_event(ctx: Context, member: Member, texts: &GlobalTexts) {
    let guild_id = member.guild_id;

    let Ok(Some(config_id)) = channel::get(&ctx, guild_id, Event::GuildMemberAddition).await else {
        debug!(
            "ChannelId of guild '{}' was either None or Err.",
            guild_id.get()
        );
        return;
    };

    if member.user.bot {
        debug!("The user who joined was a bot.");
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
            texts.guild_member_addition.title.replace(
                "%user%",
                member
                    .user
                    .global_name
                    .as_ref()
                    .unwrap_or(&member.user.name),
            ),
        )
        .description(member.user.mention().to_string())
        .color(Colors::PRIMARY)
        .field(
            &texts.guild_member_addition.id,
            format!("@{} ({})", member.user.name, member.user.id),
            true,
        )
        .field(
            &texts.guild_member_addition.account_creation,
            format!("<t:{}>", member.user.created_at().timestamp()),
            true,
        )
        .footer(CreateEmbedFooter::new(
            texts
                .guild_member_addition
                .member_count
                .replace("%count%", &member_count.to_string()),
        ));

    if let Some(url) = member.user.avatar_url() {
        embed = embed.thumbnail(url);
    }

    if let Err(err) = channel
        .send_message(&ctx.http, CreateMessage::default().embed(embed))
        .await
    {
        error!("Failed to send message: {err}");
    }
}
