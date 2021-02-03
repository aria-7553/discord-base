use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{macros::hook, DispatchError, Reason},
    model::channel::Message,
};

use crate::utils::send_embed;

#[hook]
pub async fn handle(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(info) = &error {
        if !info.is_first_try {
            return;
        }
    };

    let mut embed = CreateEmbed::default();
    embed
        .title("Ugh, something is wrong, I can feel it..")
        .description(match error {
            DispatchError::CheckFailed(info, reason) => {
                if let Reason::User(reason) = reason {
                    format!("Seems like you don't pass the check.. {}\n{}", reason, info)
                } else {
                    format!("Seems like you don't pass the check.. {}", info)
                }
            }
            DispatchError::Ratelimited(info) => {
                format!(
                    "Calm down and try again in {} seconds please",
                    info.as_secs()
                )
            }
            DispatchError::CommandDisabled(info) => info,
            DispatchError::LackingPermissions(perms) => format!(
                "You need these permissions to run this command and you don't have them 😤:\n{}",
                perms.get_permission_names().join("\n")
            ),
            DispatchError::NotEnoughArguments { min, given } => format!(
                "This command needs {} arguments™ after it but you only gave {}..",
                min, given
            ),
            DispatchError::TooManyArguments { max, given } => format!(
                "This command can't take more than {} arguments™ but you gave {}..",
                max, given
            ),
            DispatchError::BlockedUser => {
                "Oops, you're blocked to use this command for some reason..".to_string()
            }
            DispatchError::BlockedGuild => {
                "Oops, the guild or its owner is blocked to use this command for some reason.."
                    .to_string()
            }
            DispatchError::BlockedChannel => {
                "Oops, the channel is blocked to use this command for some reason..".to_string()
            }
            DispatchError::OnlyForDM => "You can only use this command in my DMs 😳".to_string(),
            DispatchError::OnlyForGuilds => {
                "You can only use this command in a guild 😳".to_string()
            }
            DispatchError::OnlyForOwners => "This command is dedicated to my master".to_string(),
            DispatchError::LackingRole => {
                "You don't have the roles required for this command..".to_string()
            }
            _ => "You discovered a very mysterious error".to_string(),
        });

    send_embed(ctx, msg, true, embed).await;
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '😤').await;
}
