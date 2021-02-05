use super::{
    set_statics::BotConfig,
    utils::{log, send_embed},
};
use crate::set_statics::BotInfo;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
    prelude::Mentionable,
};

#[group("General Stuff")]
#[commands(cmd_info)]
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
