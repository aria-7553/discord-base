pub mod cmd_error;
pub mod cmd_help;
pub mod cmd_info;
pub mod cmd_options;
pub mod set_statics;
pub mod utils;

use serenity::{
    client::{Context, EventHandler},
    model::prelude::Ready,
    Client,
};
use set_statics::{BotConfig, BotInfo};

pub async fn start(config_path: &str) {
    BotConfig::set(config_path);
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(&config.token).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    let framework = cmd_options::get_framework(bot_info.user, bot_info.owner, &config.prefix).await;

    let mut client = Client::builder(&config.token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        utils::print_and_write(format!("Couldn't start the client: {}", e));
    }
}

pub struct Handler;
#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Connected!");
        utils::log(&ctx, &String::from("Connected!")).await;
    }
}
