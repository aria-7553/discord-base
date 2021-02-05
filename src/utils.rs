use crate::set_statics::{BotConfig, BotInfo};
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{channel::Message, misc::Mentionable},
    utils::Colour,
};
use std::{fmt::Display, io::Write};

pub async fn send_embed(ctx: &Context, reply: &Message, is_error: bool, mut embed: CreateEmbed) {
    let channel = reply.channel_id;
    if is_error {
        match BotInfo::get() {
            Some(info) => {
                embed.colour(info.error_colour());
            }
            None => log(ctx, "Couldn't get BotInfo to get error_colour").await,
        }
    } else {
        match BotConfig::get() {
            Some(config) => {
                embed.colour(config.colour());
            }
            None => log(ctx, "Couldn't get BotConfig to get colour").await,
        }
    }

    if let Err(err) = channel.send_message(ctx, |m| m.set_embed(embed)).await {
        if let Err(err) = channel
            .say(ctx, format!("Oops, couldn't send the message 🤦‍♀️: {}", err))
            .await
        {
            if let Err(err) = reply
                .author
                .dm(ctx, |m| {
                    m.embed(|e| {
                        e.colour(Colour::new(15037299))
                            .description(format!(
                                "{}\nLet the admins know so they can fix it\n",
                                err
                            ))
                            .title(format!(
                                "Looks like I can't send messages in {} :(",
                                reply.channel_id.mention()
                            ))
                    })
                })
                .await
            {
                log(
                    ctx,
                    format!(
                        "Couldn't even send the message to inform the commander: {}",
                        err
                    ),
                )
                .await
            }
        }
    }
}

pub async fn log(ctx: &Context, msg: impl Display + AsRef<[u8]>) {
    match BotInfo::get() {
        Some(info) => match info.owner().create_dm_channel(ctx).await {
            Ok(channel) => {
                if let Err(err) = channel.say(ctx, &msg).await {
                    print_and_write(format!(
                        "Couldn't DM the owner when trying to log: {}\nMessage: {}",
                        err, msg
                    ));
                }
            }
            Err(err) => print_and_write(format!(
                "Couldn't get the DM channel with the owner when trying to log: {}\nMessage: {}",
                err, msg
            )),
        },
        None => print_and_write(format!(
            "Couldn't get BotInfo when trying to log\nMessage: {}",
            msg
        )),
    };
}

pub fn print_and_write(msg: impl Display) {
    let print_and_write = format!(
        "{}: {}\n\n",
        chrono::Utc::now().format("%e %B %A %H:%M:%S"),
        msg
    );
    println!("{}", print_and_write);

    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("couldnt_dm.txt")
    {
        Ok(mut file) => {
            if let Err(err) = file.write(print_and_write.as_bytes()) {
                println!("Couldn't write to the log file: {}", err)
            }
        }
        Err(err) => println!("Couldn't open the log file: {}", err),
    }
}
