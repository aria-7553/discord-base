use crate::{
    command_error, commands,
    globals::{CmdInfo, SqlitePoolKey},
    log,
};
use serenity::{
    client::Context,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    model::{channel::Message, id::UserId},
};
use sqlx::{query, Row};
use std::cmp::min;

async fn prefix_check(ctx: &Context, msg: &Message) -> Option<String> {
    let guild_id = msg.guild_id?;

    let cmd_info = CmdInfo::get()?;

    let boundary = min(msg.content.chars().count(), cmd_info.longest_len().into());
    if !cmd_info
        .commands()
        .iter()
        .any(|s| msg.content[..boundary].contains(s))
    {
        return None;
    }

    let data = ctx.data.read().await;
    let db = match data.get::<SqlitePoolKey>() {
        Some(db) => db,
        None => {
            log(ctx, "Couldn't get the database for the prefix check").await;
            return None;
        }
    };

    match query("SELECT prefix FROM prefixes WHERE guild_id = ?")
        .bind(guild_id.0 as i64)
        .fetch_optional(db)
        .await
    {
        Err(err) => {
            log(
                ctx,
                format!(
                    "Couldn't fetch prefix from the database for the prefix check: {}",
                    err
                ),
            )
            .await;
            None
        }
        Ok(row) => match row?.try_get(0) {
            Ok(prefix) => prefix,
            Err(err) => {
                log(
                    ctx,
                    format!("Couldn't get the prefix column for the guild: {}", err),
                )
                .await;
                None
            }
        },
    }
}

pub(crate) async fn set_framework(bot_id: UserId, owner_id: UserId) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix("")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(vec![owner_id].into_iter().collect())
                .dynamic_prefix(|ctx, msg| Box::pin(prefix_check(ctx, msg)))
        })
        .on_dispatch_error(command_error::handle)
        .bucket("general", |b| {
            b.limit_for(LimitedFor::Channel)
                .await_ratelimits(1)
                .delay_action(command_error::delay_action)
                .time_span(600)
                .limit(10)
        })
        .await
        .bucket("expensive", |b| {
            b.limit_for(LimitedFor::Guild)
                .await_ratelimits(1)
                .delay_action(command_error::delay_action)
                .time_span(3600)
                .limit(10)
        })
        .await
        .help(&commands::CMD_HELP)
        .group(&commands::GENERAL_GROUP)
        .group(&commands::MASTER_GROUP)
}
