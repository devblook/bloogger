use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Texts {
    error_embed_title: String,
    missing_administrator_permission: String,
    error_command_executed_in_dm: String,
    internal_error: String,
    not_valid_channel: String,
    success_embed_title: String,
    channel_setted_message: String,
}

impl Texts {
    pub fn error_embed_title(&self) -> &str {
        &self.error_embed_title
    }

    pub fn missing_administrator_permission(&self) -> &str {
        &self.missing_administrator_permission
    }

    pub fn error_command_executed_in_dm(&self) -> &str {
        &self.error_command_executed_in_dm
    }

    pub fn internal_error(&self) -> &str {
        &self.internal_error
    }

    pub fn not_valid_channel(&self) -> &str {
        &self.not_valid_channel
    }

    pub fn success_embed_title(&self) -> &str {
        &self.success_embed_title
    }

    pub fn channel_setted_message(&self) -> &str {
        &self.channel_setted_message
    }
}

impl Default for Texts {
    fn default() -> Self {
        Self {
            error_embed_title: String::from("Error"),
            missing_administrator_permission: String::from(
                ":x: You must be an administrator to execute this command.",
            ),
            error_command_executed_in_dm: String::from(
                ":x: This command can only be executed in a guild.",
            ),
            internal_error: String::from(
                "An internal error has occurred, please contact the administrators.",
            ),
            not_valid_channel: String::from("Channel is not a text or voice channel."),
            success_embed_title: String::from("Success"),
            channel_setted_message: String::from("Channel was set."),
        }
    }
}
