use std::borrow::Cow;

use crate::{set_statics::BotConfig, log, send_embed};
use serenity::{
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
    let (description, owner, colour) = match ctx.http.get_current_application_info().await {
        Ok(info) => (
            Cow::from(info.description),
            info.owner.id.mention().to_string().into(),
            8505220,
        ),
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
                "Awkward but I think I forgot who I am..".into(),
                "Oh, I forgot my creator too.. Sorry creator..".into(),
                15037299,
            )
        }
    };
    let (title, invite, github) = match BotConfig::get() {
        Some(config) => (
            "Want me in your server? Click here then!",
            Some(&config.invite),
            Cow::from(&config.github),
        ),
        None => {
            log(ctx, "Couldn't get Config for the `info` command").await;
            (
                "Oops, I lost my invite, I swear I had it right here",
                None,
                "I forgot my GitHub page too..".into(),
            )
        }
    };
    let fields = Some(vec![
        ("Made by:", owner, true),
        ("On GitHub:", github, true),
    ]);
    send_embed(ctx, msg, colour, description, title, fields, invite).await;

    Ok(())
}
