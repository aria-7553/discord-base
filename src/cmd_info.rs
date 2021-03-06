use serenity::{
    builder::CreateEmbed,
    client::Context,
    framework::standard::{CommandResult, macros::command},
    model::channel::Message,
    prelude::Mentionable,
};

use crate::{globals::{BotConfig, BotInfo}, log, send_embed};

/// The `info` command to give info about feedback, owner, invite, GitHub etc.
/// # Errors
/// Informs the user and logs if getting BotInfo or BotConfig failed, still sending all the info
/// it could get
#[command("info")]
#[aliases("about", "invite", "inv")]
#[bucket = "general"]
#[description = "How you can add me to your server, contact my owner, find my GitHub page etc."]
async fn cmd_info(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.footer(|f| {
        f.text("I act weirdly? Want me to speak another language? Anything else? You can friend and DM my owner anytime for any feedback you have!")
    });
    let mut is_error = false;

    match BotInfo::get() {
        Some(info) => {
            embed
                .description(&info.description())
                .field("Made by:", info.owner().mention(), true);
        }
        None => {
            log(ctx, "Couldn't get BotInfo for the `info` command").await;
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
