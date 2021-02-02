use std::collections::HashSet;

use discord_base::{config_parser::Config, Handler};
use serenity::{Client, framework::{StandardFramework, standard::buckets::LimitedFor}, http::Http};

#[tokio::main]
async fn main() {
    Config::set("config.toml");
    let token = &Config::get()
        .expect("Couldn't access CONFIG to get the token")
        .token;

    let http = Http::new_with_token(&token);
    let (owner_id, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owner_id = HashSet::new();
            owner_id.insert(info.owner.id);
            match http.get_current_user().await {
                Ok(bot_id) => (owner_id, bot_id.id),
                Err(err) => panic!("Could not access the bot id: {}", err),
            }
        }
        Err(err) => panic!("Could not access application info: {}", err),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(".")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(owner_id)
        })
        .on_dispatch_error(discord_base::dispatch_error)
        .bucket("general", |b| {
            b.delay(1)
                .limit_for(LimitedFor::User)
                .await_ratelimits(1)
                .delay_action(discord_base::delay_action)
        })
        .await
        .help(&discord_base::CMD_HELP)
        .group(&discord_base::GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        println!("Couldn't start the client: {}", e);
    }
}
