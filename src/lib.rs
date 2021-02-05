mod cmd_error;
mod cmd_help;
mod cmd_info;
mod cmd_options;
mod set_statics;
mod utils;

use serenity::{Client, client::{Context, EventHandler}, model::prelude::{Activity, Ready}};
use set_statics::{BotConfig, BotInfo};

pub async fn start(config_path: &str) {
    BotConfig::set(config_path);
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(config.token()).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    let framework = cmd_options::get_framework(bot_info.user(), bot_info.owner()).await;

    let mut client = Client::builder(&config.token())
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
    async fn ready(&self, ctx: Context, info: Ready) {
        ctx.set_activity(Activity::playing(format!("@{} help", info.user.name).as_str())).await;

        println!("Connected!");
        utils::log(&ctx, &String::from("Connected!")).await;
    }
}
