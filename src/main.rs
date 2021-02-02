use discord_base::{
    commands,
    set_statics::{BotConfig, BotInfo},
};
use serenity::Client;

#[tokio::main]
async fn main() {
    BotConfig::set("config.toml");
    let token = &BotConfig::get()
        .expect("Couldn't access BOT_CONFIG to get the token")
        .token;

    BotInfo::set(token).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    let framework = commands::options::get_framework(bot_info.bot_id, bot_info.owner_id).await;

    let mut client = Client::builder(token)
        .event_handler(discord_base::Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        println!("Couldn't start the client: {}", e);
    }
}
