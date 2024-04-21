use serenity::all::{Message, MessageUpdateEvent};
use serenity::{
    all::{ChannelId, GuildId, MessageId},
    client::{Context, EventHandler},
};

use crate::{event, texts::Texts};

pub struct Handler {
    texts: Texts,
}

impl Handler {
    pub fn new(texts: Texts) -> Self {
        Self { texts }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message_delete(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        event::message_delete::message_delete_event(
            ctx,
            channel_id,
            message_id,
            guild_id,
            &self.texts,
        )
        .await
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        event::message_update::message_update_event(ctx, old_if_available, new, event, &self.texts)
            .await
    }
}
