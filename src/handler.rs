use serenity::client::EventHandler;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {}
