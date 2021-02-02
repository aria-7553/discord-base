use serenity::{
    client::Context,
    framework::standard::{macros::hook, DispatchError, Reason},
    model::channel::Message,
};

use crate::send_embed;

#[hook]
pub async fn handle(ctx: &Context, msg: &Message, error: DispatchError) {
    let info = match error {
        DispatchError::CheckFailed(info, reason) => {
            if let Reason::User(reason) = reason {
                Some(format!(
                    "Seems like you don't pass the check.. {}\n{}",
                    reason, info
                ))
            } else {
                Some(format!("Seems like you don't pass the check.. {}", info))
            }
        }
        DispatchError::Ratelimited(info) => {
            if info.is_first_try {
                Some(format!(
                    "Calm down and try again in {} seconds please",
                    info.as_secs()
                ))
            } else {
                None
            }
        }
        DispatchError::CommandDisabled(info) => Some(info),
        DispatchError::BlockedUser => Some(String::from(
            "Oops, you're blocked to use this command for some reason..",
        )),
        DispatchError::BlockedGuild => Some(String::from(
            "Oops, the guild or its owner is blocked to use this command for some reason..",
        )),
        DispatchError::BlockedChannel => Some(String::from(
            "Oops, the channel is blocked to use this command for some reason..",
        )),
        DispatchError::OnlyForDM => {
            Some(String::from("You can only use this command in my DMs 😳"))
        }
        DispatchError::OnlyForGuilds => {
            Some(String::from("You can only use this command in a guild"))
        }
        DispatchError::OnlyForOwners => {
            Some(String::from("This command is dedicated to my master"))
        }
        DispatchError::LackingRole => Some(String::from(
            "You don't have the roles required for this command..",
        )),
        DispatchError::LackingPermissions(perms) => Some(format!(
            "You need these permissions to run this command and you don't have them 😤:\n{}",
            perms.get_permission_names().join("\n")
        )),
        DispatchError::NotEnoughArguments { min, given } => Some(format!(
            "This command needs {} arguments™ after it but you only gave {}..",
            min, given
        )),
        DispatchError::TooManyArguments { max, given } => Some(format!(
            "This command can't take more than {} arguments™ but you gave {}..",
            max, given
        )),
        _ => Some(String::from("You discovered a very mysterious error")),
    };
    if let Some(info) = info {
        send_embed(
            ctx,
            msg,
            16738657,
            &info,
            &String::from("Ugh, something is wrong, I can feel it.."),
            None,
        )
        .await;
    }
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '😤').await;
}
