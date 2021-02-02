use discord_base::{commands, config_parser::Config};
use serenity::Client;

#[tokio::main]
async fn main() {
    Config::set("config.toml");
    let token = &Config::get()
        .expect("Couldn't access CONFIG to get the token")
        .token;
    let framework = commands::config::get_framework(token).await;

    let mut client = Client::builder(token)
        .event_handler(discord_base::Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        println!("Couldn't start the client: {}", e);
    }
}
