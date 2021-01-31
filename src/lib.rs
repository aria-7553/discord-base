use config_parser::Config;
use serenity::{
    client::{Context, EventHandler},
    model::{id::UserId, prelude::Ready},
};

pub mod config_parser;

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Connected!");
        log(&ctx, Config::get(), "Connected!").await;
    }
}

pub async fn log(ctx: &Context, config: &Config, msg: &str) {
    match UserId::from(config.owner_id).create_dm_channel(ctx).await {
        Err(err) => println!(
            "Couldn't create DM with owner when trying to log \"{}\": {}",
            msg, err
        ),
        Ok(channel) => {
            if let Err(err) = channel.say(ctx, msg).await {
                println!("Couldn't log \"{}\": {}", msg, err);
            }
        }
    }
}
