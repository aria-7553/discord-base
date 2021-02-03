use super::utils::send_embed;
use serenity::{
    client::Context,
    framework::standard::{macros::hook, DispatchError, Reason},
    model::channel::Message,
};
use std::borrow::Cow;

#[hook]
pub async fn handle(ctx: &Context, msg: &Message, error: DispatchError) {
    let info = match &error {
        DispatchError::CheckFailed(info, reason) => {
            if let Reason::User(reason) = reason {
                Some(Cow::from(format!(
                    "Seems like you don't pass the check.. {}\n{}",
                    reason, info
                )))
            } else {
                Some(format!("Seems like you don't pass the check.. {}", info).into())
            }
        }
        DispatchError::Ratelimited(info) => {
            if info.is_first_try {
                Some(
                    format!(
                        "Calm down and try again in {} seconds please",
                        info.as_secs()
                    )
                    .into(),
                )
            } else {
                None
            }
        }
        DispatchError::CommandDisabled(info) => Some(info.into()),
        DispatchError::BlockedUser => {
            Some("Oops, you're blocked to use this command for some reason..".into())
        }
        DispatchError::BlockedGuild => Some(
            "Oops, the guild or its owner is blocked to use this command for some reason..".into(),
        ),
        DispatchError::BlockedChannel => {
            Some("Oops, the channel is blocked to use this command for some reason..".into())
        }
        DispatchError::OnlyForDM => Some("You can only use this command in my DMs 😳".into()),
        DispatchError::OnlyForGuilds => Some("You can only use this command in a guild".into()),
        DispatchError::OnlyForOwners => Some("This command is dedicated to my master".into()),
        DispatchError::LackingRole => {
            Some("You don't have the roles required for this command..".into())
        }
        DispatchError::LackingPermissions(perms) => Some(
            format!(
                "You need these permissions to run this command and you don't have them 😤:\n{}",
                perms.get_permission_names().join("\n")
            )
            .into(),
        ),
        DispatchError::NotEnoughArguments { min, given } => Some(
            format!(
                "This command needs {} arguments™ after it but you only gave {}..",
                min, given
            )
            .into(),
        ),
        DispatchError::TooManyArguments { max, given } => Some(
            format!(
                "This command can't take more than {} arguments™ but you gave {}..",
                max, given
            )
            .into(),
        ),
        _ => Some("You discovered a very mysterious error".into()),
    };
    if let Some(info) = info {
        send_embed(
            ctx,
            msg,
            15037299,
            info,
            "Ugh, something is wrong, I can feel it..",
            None::<Vec<(&str, &str, bool)>>,
            None::<&str>,
        )
        .await;
    }
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '😤').await;
}
