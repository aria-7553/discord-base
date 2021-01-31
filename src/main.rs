use discord_base::{Handler, config_parser::Config};
use serenity::Client;

#[tokio::main]
async fn main() {
    Config::set("config.toml");

    let mut client = Client::builder(&Config::get().token)
        .event_handler(Handler)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start().await {
        println!("Couldn't start the client: {}", e);
    }
}
