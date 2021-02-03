use serenity::{
    client::Context,
    model::{channel::Message, misc::Mentionable},
    utils::Colour,
};
use std::fmt::Display;

pub async fn send_embed<T: Display, U: Display, F: Display, S: Display, D: Display>(
    ctx: &Context,
    reply: &Message,
    is_error: bool,
    description: T,
    title: U,
    fields: Option<Vec<(F, S, bool)>>,
    url: Option<D>,
) {
    if let Err(err) = reply
        .channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                if let Some(fields) = fields {
                    e.fields(fields);
                };
                if let Some(url) = url {
                    e.url(url);
                };
                e.colour(match is_error {
                    true => Colour::new(15037299),
                    false => Colour::new(8505220),
                })
                .description(&description)
                .title(&title)
            })
        })
        .await
    {
        if let Err(err) = reply
            .channel_id
            .say(
                ctx,
                format!(
                "Oops, couldn't send the message 🤦‍♀️: {}\nSo here it is in ugly form:\n**{}**\n{}",
                err, &title, &description),
            )
            .await
        {
            let owner_mention = ctx.cache.current_user_id().await.mention();
            if let Err(err) = reply.author.dm(ctx, |m| {
                m.embed(|e| {
                    e.colour(Colour::new(15037299))
                     .description(format!(
                         "{}\nLet the admins know so they can fix it\n*Or if this is a weird error please let my owner know at {}* 👉👈",
                         err, owner_mention))
                     .title(format!(
                         "Looks like I can't send messages in {}",
                         reply.channel_id.mention()))
                })
            }).await {
                log(ctx, &format!(
                    "Couldn't even send the message to inform the commander: {}", err)).await
            }
        }
    }
}

pub async fn log(ctx: &Context, msg: &str) {
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
