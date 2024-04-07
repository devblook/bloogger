use serenity::prelude::TypeMapKey;

use crate::cache::GuildConfigCache;

#[derive(Default)]
pub struct Data {
    pub cache: GuildConfigCache,
}

impl TypeMapKey for Data {
    type Value = Self;
}
