use poise::serenity_prelude::Channel;
use serenity::all::ChannelType;
use tracing::{error, instrument};

use crate::{data::Data, event::Event, messages::Messages, Context, Error};

#[instrument(skip(ctx, event, channel))]
#[poise::command(slash_command)]
pub async fn set(ctx: Context<'_>, event: Event, channel: Option<Channel>) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let data = data.get::<Data>().expect("Data should never be none.");

    if ctx.guild_id().is_none() {
        ctx.not_in_guild(data).await;
        return Ok(());
    }

    let member = match ctx.author_member().await {
        Some(member) => member,
        None => return Ok(()),
    };

    if !member.permissions.unwrap().administrator() {
        ctx.missing_administrator(data).await;
        return Ok(());
    }

    let guild_id = ctx
        .guild_id()
        .expect("None case was handled earlier.")
        .get();

    let mut config = match data.cache.get_or_insert(guild_id).await {
        Err(err) => {
            error!("Failed to get GuildConfig: {err:?}");
            ctx.internal_error(data).await;
            return Ok(());
        }
        Ok(config) => config,
    };

    let id = match channel {
        Some(channel) => {
            let Some(channel) = channel.guild() else {
                ctx.internal_error(data).await;
                return Ok(());
            };

            let kind = channel.kind;

            if kind != ChannelType::Voice && kind != ChannelType::Text {
                ctx.not_valid_channel(data).await;
                return Ok(());
            }

            Some(channel.id.get())
        }
        None => None,
    };

    config.set_channel(event.key(), id);
    data.cache.insert(guild_id, config).await;
    ctx.channel_set(data).await;

    Ok(())
}

#[instrument(skip(ctx, event))]
#[poise::command(slash_command)]
pub async fn unset(ctx: Context<'_>, event: Event) -> Result<(), Error> {
    let data = ctx.serenity_context().data.read().await;
    let data = data.get::<Data>().expect("Data should never be none.");

    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.not_in_guild(data).await;
            return Ok(());
        }
    };

    let member = match ctx.author_member().await {
        Some(member) => member,
        None => return Ok(()),
    };

    if !member.permissions.unwrap().administrator() {
        ctx.missing_administrator(data).await;
        return Ok(());
    }

    let mut config = match data.cache.get_or_insert(guild_id).await {
        Err(err) => {
            error!("Failed to get GuildConfig: {err:?}");
            ctx.internal_error(data).await;
            return Ok(());
        }
        Ok(config) => config,
    };

    config.set_channel(event.key(), None);
    data.cache.insert(guild_id, config).await;
    ctx.channel_unset(data).await;

    Ok(())
}
