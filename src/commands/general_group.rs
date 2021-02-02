use crate::{config_parser::Config, log, send_embed};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
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
    let (description, colour) = match ctx.http.get_current_application_info().await {
        Ok(info) => (info.description, 8505220),
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
                15037299,
            )
        }
    };
    let (title, invite) = match Config::get() {
        Some(config) => (
            String::from("Want me in your server? Click here then!"),
            Some(&config.invite),
        ),
        None => {
            log(
                ctx,
                &String::from("Couldn't get Config for the `info` command"),
            )
            .await;
            (
                String::from("Oops, I lost my invite, I swear I had it right here"),
                None,
            )
        }
    };
    send_embed(ctx, msg, colour, &description, &title, invite).await;

    Ok(())
}
