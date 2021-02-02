use super::{error, general_group, help};
use serenity::{
    framework::{standard::buckets::LimitedFor, StandardFramework},
    model::id::UserId,
};

pub async fn get_framework(bot_id: UserId, owner_id: UserId) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix(".")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(vec![owner_id].into_iter().collect())
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
