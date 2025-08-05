pub mod guild_member_addition;
pub mod guild_member_removal;
pub mod message_delete;
pub mod message_update;

#[derive(poise::ChoiceParameter)]
pub enum Event {
    #[name = "Message Delete"]
    MessageDelete,
    #[name = "Message Update"]
    MessageUpdate,
    #[name = "User Join"]
    GuildMemberAddition,
    #[name = "User Left"]
    GuildMemberRemoval,
}

impl Event {
    pub fn key(&self) -> &str {
        match self {
            Self::MessageDelete => "MD",
            Self::MessageUpdate => "MU",
            Self::GuildMemberAddition => "UJ",
            Self::GuildMemberRemoval => "UL",
        }
    }
}
