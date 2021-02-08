use super::globals::BotConfig;
use crate::{
    globals::{BotInfo, SqlitePoolKey},
    log, send_embed,
};
use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::Mentionable,
};
use sqlx::query;
use std::collections::HashSet;

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
#[embed_error_colour = "#b00020"]
#[embed_success_colour = "#b29ddb"]
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

#[group("Master")]
#[sub_groups(General)]
#[help_available(false)]
pub struct Master;

#[group("General Stuff")]
#[commands(cmd_info, cmd_prefix)]
struct General;

#[command("info")]
#[bucket = "general"]
#[help_available]
#[description = "How you can add me to your server, contact my owner, my GitHub page etc."]
#[aliases("about", "invite", "inv")]
async fn cmd_info(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    let mut is_error = false;

    match BotInfo::get() {
        Some(info) => {
            embed
                .description(&info.description())
                .field("Made by:", info.owner().mention(), true);
        }
        None => {
            log(ctx, "Couln't get BotInfo for the `info` command").await;
            embed.description("Awkward but I think I forgot who I am..");
            is_error = true
        }
    };

    match BotConfig::get() {
        Some(config) => {
            embed
                .title("Want me in your server? Click here then!")
                .url(&config.invite())
                .field("on GitHub:", &config.github(), true);
        }
        None => {
            log(ctx, "Couldn't get BotConfig for the `info` command").await;
            embed.title("Oops, I lost my invite, I swear I had it right here");
            is_error = true
        }
    };
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}

#[command("prefix")]
#[aliases(
    "setprefix",
    "set_prefix",
    "set-prefix",
    "changeprefix",
    "change_prefix",
    "change-prefix"
)]
#[num_args(1)]
#[required_permissions("MANAGE_GUILD")]
#[only_in("guilds")]
#[bucket = "expensive"]
#[help_available]
#[description = "Change the prefix I'll use in this server"]
async fn cmd_prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut embed = CreateEmbed::default();
    let mut is_error = true;
    let data = ctx.data.read().await;

    let db = data.get::<SqlitePoolKey>();
    let prefix = args.current();
    let guild_id = msg.guild_id;

    if let None = prefix {
        log(ctx, "args.current() is None for the prefix command").await;
        embed.title("Something very weird happened and I let you use this command without giving a prefix")
        .description("Rerun the command but this time actually type the prefix you want after the command");
    };
    if let None = guild_id {
        log(ctx, "msg.guild_id is None for the prefix command").await;
        embed
            .title("Something weird happened and I let you use this command in DMs")
            .description("We have to be in a guild to set the prefix for a guild, no?");
    };
    if let None = db {
        log(
            ctx,
            format!("Couldn't get SqlitePool for the prefix command"),
        )
        .await;
        embed
            .title("Now this is super weird and scary")
            .description("I lost my whole book where I write things down, sorry..");
    };

    if let (Some(prefix), Some(guild_id), Some(db)) = (prefix, guild_id, db) {
        if prefix.chars().count() > 10 {
            embed
                .title("Your prefix can't be longer than 10 characters")
                .description("Why would you want it that long anyway..");
        } else {
            let guild_id_int = guild_id.0 as i64;
            if let Err(err) = query!(
                "INSERT OR REPLACE INTO prefixes (guild_id, prefix)
                VALUES(?, ?);",
                guild_id_int,
                prefix
            )
            .execute(db)
            .await
            {
                log(ctx, format!("Couldn't insert to prefixes: {}", err)).await;
                embed
                    .title("Ugh, I couldn't write that down..")
                    .description(
                        "I just let my developer know, until then you could just try again",
                    );
            } else {
                is_error = false;
                embed.description(format!("Voila! Your prefix here is now `{}`", prefix));
            }
        }
    }

    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}
