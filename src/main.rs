#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    bloogger::init().await;
}
