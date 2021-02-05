use super::{cmd_error, cmd_help, cmd_info};
use serenity::{
    framework::{standard::buckets::LimitedFor, StandardFramework},
    model::id::UserId,
};

pub async fn get_framework(bot_id: UserId, owner_id: UserId, prefix: &'static str) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix(prefix)
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(vec![owner_id].into_iter().collect())
        })
        .on_dispatch_error(cmd_error::handle)
        .bucket("general", |b| {
            b.delay(1)
                .limit_for(LimitedFor::User)
                .await_ratelimits(1)
                .delay_action(cmd_error::delay_action)
        })
        .await
        .help(&cmd_help::CMD_HELP)
        .group(&cmd_info::GENERAL_GROUP)
}
