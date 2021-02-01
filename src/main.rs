use discord_base::{config_parser::Config, Handler};
use serenity::Client;

#[tokio::main]
async fn main() {
    Config::set("config.toml");

    let mut client = Client::builder(
        &Config::get()
            .expect("Couldn't access CONFIG to get the token")
            .token,
    )
    .event_handler(Handler)
    .await
    .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        println!("Couldn't start the client: {}", e);
    }
}
