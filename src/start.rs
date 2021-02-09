use crate::{
    command_options::set_framework,
    globals::{set_db, BotConfig, BotInfo, CmdInfo, SqlitePoolKey},
    print_and_write, Handler,
};
use serenity::{client::bridge::gateway::GatewayIntents, Client};

pub async fn start(config_path: &str) {
    BotConfig::set(config_path);
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(config.token()).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    CmdInfo::set();

    let db = set_db().await;

    let client_builder = Client::builder(&config.token())
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILDS
        )
        .event_handler(Handler)
        .type_map_insert::<SqlitePoolKey>(db);

    let framework = set_framework(bot_info.user(), bot_info.owner()).await;

    let mut client = client_builder
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        print_and_write(format!("Couldn't start the client: {}", e));
    }
}
