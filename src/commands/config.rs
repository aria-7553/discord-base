use std::collections::HashSet;

use serenity::{
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::Http,
};

use super::{error, general_group, help};

pub async fn get_framework(token: &String) -> StandardFramework {
    let http = Http::new_with_token(token);
    let (owner_id, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owner_id = HashSet::new();
            owner_id.insert(info.owner.id);
            match http.get_current_user().await {
                Ok(bot_id) => (owner_id, bot_id.id),
                Err(err) => panic!("Couldn't access the bot id: {}", err),
            }
        }
        Err(err) => panic!("Couldn't access application info: {}", err),
    };

    StandardFramework::new()
        .configure(|c| {
            c.prefix(".")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(owner_id)
        })
        .on_dispatch_error(error::handle)
        .bucket("general", |b| {
            b.delay(1)
                .limit_for(LimitedFor::User)
                .await_ratelimits(1)
                .delay_action(error::delay_action)
        })
        .await
        .help(&help::CMD_HELP)
        .group(&general_group::GENERAL_GROUP)
}
