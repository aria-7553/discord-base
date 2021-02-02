use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

use crate::{log, send_embed};

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
