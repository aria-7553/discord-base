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
        let owner_id = match Config::get() {
            Some(config) => Some(config.owner_id),
            None => None,
        };
        log(&ctx, owner_id, "Connected!").await;
    }
}

pub async fn log(ctx: &Context, owner_id: Option<u64>, msg: &str) {
    match owner_id {
        Some(owner_id) => {
            if let Err(err) = match UserId::from(owner_id).create_dm_channel(ctx).await {
                Ok(channel) => match channel.say(ctx, msg).await {
                    Ok(message) => Ok(message),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            } {
                println!("Couldn't log \"{}\": {}", msg, err)
            }
        }
        None => println!(
            "Couldn't get the owner_id from CONFIG when trying to log: {}",
            msg
        ),
    };
}
