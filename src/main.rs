use discord_base::{
    cmd_error,
    cmd_help::CMD_HELP,
    cmd_prefix::prefix_check,
    globals::{set_db, BotConfig, BotInfo, CmdInfo, SqlitePoolKey},
    print_and_write, Handler, GENERAL_GROUP, MASTER_GROUP,
};
use serenity::{
    client::bridge::gateway::GatewayIntents,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    Client,
};

#[tokio::main]
async fn main() {
    BotConfig::set("config.toml");
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(config.token()).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    CmdInfo::set();

    let db = set_db().await;

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
