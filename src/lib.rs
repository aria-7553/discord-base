use std::collections::HashSet;

use serenity::{
    client::{Context, EventHandler},
    framework::standard::{
        help_commands,
        macros::{command, group, help, hook},
        Args, CommandGroup, CommandResult, DispatchError, HelpOptions, Reason,
    },
    model::{channel::Message, id::UserId, misc::Mentionable, prelude::Ready},
    utils::Colour,
};

pub mod config_parser;

pub struct Handler;

#[group("General Stuff")]
#[commands(cmd_info)]
struct General;

#[command("info")]
#[bucket = "general"]
#[help_available]
#[description = "How you can add me to your server, contact my owner, my GitHub page etc."]
#[aliases("about", "invite")]
async fn cmd_info(ctx: &Context, msg: &Message) -> CommandResult {
    let (description, colour) = match ctx.http.get_current_application_info().await {
        Ok(info) => (info.description, 7855479),
        Err(err) => {
            log(
                ctx,
                &format!(
                    "Couln't get the application info for the `info` command: {}",
                    err
                ),
            )
            .await;
            (
                String::from("Awkward but I think I forgot who I am.."),
                16738657,
            )
        }
    };
    send_embed(
        ctx,
        msg,
        colour,
        &description,
        &String::from("Want me in your server? Click here then!"),
    )
    .await;

    Ok(())
}

#[help("help", "commands", "cmds")]
#[suggestion_text = "**Maybe you meant one of these:**\n{}"]
#[no_help_available_text = "I don't know this command :("]
#[usage_label = "You use it like:"]
#[usage_sample_label = "For example:"]
#[description_label = "↝"]
#[aliases_label = "Alternatives"]
#[guild_only_text = "You can't use this in my DMs though"]
#[checks_label = "Only if"]
#[dm_only_text = "You can only use this in my DMs!"]
#[command_not_found_text = "I don't know the command `{}` :("]
#[individual_command_tip = "Want more info about a command? Type `help [command name]`"]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = "*If a command is ~~stricken through~~ it means you're in the wrong channel for that command*"]
#[lacking_role = "Hide"]
#[lacking_permissions = "Hide"]
#[lacking_ownership = "Hide"]
#[lacking_conditions = "Hide"]
#[wrong_channel = "Strike"]
#[embed_error_colour = "#FF6961"]
#[embed_success_colour = "#77DD77"]
#[max_levenshtein_distance(3)]
#[indention_prefix = "♡"]
async fn cmd_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn delay_action(ctx: &Context, msg: &Message) {
    let _ = msg.react(ctx, '😤').await;
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
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
        )
        .await;
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Connected!");
        log(&ctx, &String::from("Connected!")).await;
    }
}

async fn send_embed(
    ctx: &Context,
    reply: &Message,
    colour: u32,
    description: &String,
    title: &String,
) {
    if let Err(err) = reply
        .channel_id
        .send_message(ctx, |m| {
            m.reference_message(reply);

            m.embed(|e| {
                e.colour(Colour::new(colour));
                e.description(description);
                e.title(title)
            })
        })
        .await
    {
        if let Err(err) = reply.channel_id.say(ctx, format!("Oops, couldn't send the message 🤦‍♀️: {}\nSo here it is in ugly form:\n**{}**\n{}", err, title, description)).await {
            let owner_mention = ctx.cache.current_user_id().await.mention();
            if let Err(err) = reply.author.dm(ctx, |m| {
                m.embed(|e| {
                    e.colour(Colour::new(16738657))
                     .description(format!("{}\nLet the admins know so they can fix it\n*Or if this is a weird error please let my owner know at {}* 👉👈", err, owner_mention))
                     .title(format!("Looks like I can't send messages in {}", reply.channel_id.mention()))
                })
            }).await {
                log(ctx, &format!("Couldn't even send the message to inform the commander: {}", err)).await
            }
        }
    }
}

async fn log(ctx: &Context, msg: &String) {
    match ctx.http.get_current_application_info().await {
        Ok(info) => match info.owner.create_dm_channel(ctx).await {
            Ok(channel) => {
                if let Err(err) = channel.say(ctx, msg).await {
                    println!(
                        "Couldn't DM the owner when trying to log: {}\nLog: {}",
                        err, msg
                    );
                }
            }
            Err(err) => println!(
                "Couldn't get the DM channel with the owner when trying to log: {}\nLog: {}",
                err, msg
            ),
        },
        Err(err) => println!(
            "Couldn't get the application info when trying to log: {}\nLog: {}",
            err, msg
        ),
    }
}
