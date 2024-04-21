use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GuildConfig {
    #[serde(rename = "c")]
    channels: HashMap<String, u64>,

    #[serde(skip_serializing, skip_deserializing)]
    has_changed: bool,
}

impl GuildConfig {
    #[instrument(skip(self))]
    pub fn has_changed(&self) -> bool {
        self.has_changed
    }

    #[instrument(skip(self))]
    pub fn get_channel(&self, key: &str) -> Option<u64> {
        self.channels.get(key).copied()
    }

    #[instrument(skip(self))]
    pub fn set_channel(&mut self, key: &str, channel_id: Option<u64>) {
        self.has_changed = true;

        if channel_id.is_none() {
            self.channels.remove(key);
            return;
        }

        self.channels.insert(
            key.to_string(),
            channel_id.expect("None case was handled earlier."),
        );
    }
}
