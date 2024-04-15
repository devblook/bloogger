#[derive(poise::ChoiceParameter)]
pub enum Event {
    #[name = "Message Delete"]
    MessageDelete,
}

impl Event {
    pub fn key(&self) -> &str {
        match self {
            Self::MessageDelete => "MESSAGE_DELETE",
        }
    }
}
