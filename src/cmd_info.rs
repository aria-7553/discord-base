use super::{
    set_statics::BotConfig,
    utils::{log, send_embed},
};
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
    match ctx.http.get_current_application_info().await {
        Ok(info) => {
            embed
                .description(info.description)
                .field("Made by:", info.owner.id.mention(), true);
        }
        Err(err) => {
            log(
                ctx,
                format!(
                    "Couln't get the application info for the `info` command: {}",
                    err
                ),
            )
            .await;
            embed.description("Awkward but I think I forgot who I am..");
            is_error = true
        }
    };

    match BotConfig::get() {
        Some(config) => {
            embed
                .title("Want me in your server? Click here then!")
                .url(&config.invite)
                .field("on GitHub:", &config.github, true);
        }
        None => {
            log(ctx, "Couldn't get Config for the `info` command").await;
            embed.title("Oops, I lost my invite, I swear I had it right here");
            is_error = true
        }
    };
    send_embed(ctx, msg, is_error, embed).await;
    Ok(())
}
