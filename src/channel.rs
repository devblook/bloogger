use serenity::{all::GuildId, client::Context};

use crate::{cache::error::Error, data::Data, event::Event};

pub async fn get(ctx: &Context, guild_id: GuildId, event: Event) -> Result<Option<u64>, Error> {
    let data = ctx.data.read().await;
    let data = data.get::<Data>().expect("Data should never be none.");

    let config = data.cache.get_or_insert(guild_id.get()).await?;

    Ok(config.get_channel(event.key()))
}
