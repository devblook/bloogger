pub mod message_delete;
pub mod message_update;

#[derive(poise::ChoiceParameter)]
pub enum Event {
    #[name = "Message Delete"]
    MessageDelete,
    #[name = "Message Update"]
    MessageUpdate,
}

impl Event {
    pub fn key(&self) -> &str {
        match self {
            Self::MessageDelete => "MD",
            Self::MessageUpdate => "MU",
        }
    }
}
