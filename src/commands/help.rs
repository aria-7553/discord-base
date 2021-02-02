use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

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
