use serenity::{
    Client,
    client::bridge::gateway::GatewayIntents,
    framework::{standard::buckets::LimitedFor, StandardFramework},
};

use discord_base::{cmd_error, cmd_help::CMD_HELP, cmd_prefix::prefix_check, GENERAL_GROUP, globals::{BotConfig, BotInfo, CmdInfo, set_db, SqlitePoolKey}, Handler, MASTER_GROUP, print_and_write, set_dir};

/// Sets every global, creates the framework and the client, and starts the client in auto
/// sharded mode
/// - You should add your own requirements to get the bot started here
/// # Panics
/// If getting the BotConfig, BotInfo or creating the client failed
/// # Errors
/// If starting the client failed, probably meaning an error on Discord's side
#[tokio::main]
async fn main() {
    set_dir();

    /// You can change the `config_path` here to customise, using directories or not using `
    /// .toml` as the extension isn't recommended!
    BotConfig::set("config.toml");
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(config.token()).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    CmdInfo::set();

    let db = set_db().await;

    /// 1. Creates the framework and the general and expensive buckets. You can add your own buckets
    /// to it or customise them. Customising anything else isn't recommended!
    /// 2. You should add your groups here the same way!
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_info.user()))
                .owners(vec![bot_info.owner()].into_iter().collect())
                .dynamic_prefix(|ctx, msg| Box::pin(prefix_check(ctx, msg)))
        })
        .on_dispatch_error(cmd_error::handle)
        .bucket("general", |b| {
            b.limit_for(LimitedFor::Channel)
                .await_ratelimits(1)
                .delay_action(cmd_error::delay_action)
                .time_span(600)
                .limit(10)
        })
        .await
        .bucket("expensive", |b| {
            b.limit_for(LimitedFor::Guild)
                .await_ratelimits(1)
                .delay_action(cmd_error::delay_action)
                .time_span(3600)
                .limit(10)
        })
        .await
        .help(&CMD_HELP)
        .group(&GENERAL_GROUP)
        .group(&MASTER_GROUP);

    /// Creates the client with the required intents, you should add more intents as you require
    /// them. Customising anything else isn't recommended!
    let mut client = Client::builder(&config.token())
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILDS,
        )
        .event_handler(Handler)
        .type_map_insert::<SqlitePoolKey>(db)
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        print_and_write(format!("Couldn't start the client: {}", e));
    }
}
