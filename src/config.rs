use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct GuildConfig {
    #[serde(skip_serializing, skip_deserializing)]
    pub has_changed: bool,
}
