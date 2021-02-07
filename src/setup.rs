use crate::{command_error, commands, log, print_and_write, Handler};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::{standard::buckets::LimitedFor, StandardFramework},
    http::client::Http,
    model::{channel::Message, id::UserId},
    prelude::TypeMapKey,
    Client,
};
use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
use std::{fs, io};

const DEFAULT_CONFIG: &'static str =
    "# The token of the bot: https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account
token = \"TOKEN HERE\"

# The invite link for the bot: https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot
invite = \"https://discord.com/api/oauth2/THE REST OF THE LINK HERE\"

# The link of the bot's repo's GitHub's page
github = \"https://github.com/USER NAME HERE/REPO NAME HERE\"

# The colour utils::send_embed() will use if is_error is false: https://www.checkyourmath.com/convert/color/rgb_decimal.php
colour = 11771355";

pub struct SqlitePoolKey;
impl TypeMapKey for SqlitePoolKey {
    type Value = SqlitePool;
}

pub async fn start(config_path: &str) {
    BotConfig::set(config_path);
    let config = BotConfig::get().expect("Couldn't access BOT_CONFIG to get the token");

    BotInfo::set(config.token()).await;
    let bot_info = BotInfo::get().expect("Couldn't access BOT_INFO to get the owner and bot ID");

    let db = set_db().await;

    let framework = set_framework(&db, bot_info.user(), bot_info.owner()).await;

    let mut client = Client::builder(&config.token())
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<SqlitePoolKey>(db)
        .await
        .expect("Couldn't create the client");

    if let Err(e) = client.start_autosharded().await {
        print_and_write(format!("Couldn't start the client: {}", e));
    }
}

#[derive(Deserialize)]
pub struct BotConfig {
    token: String,
    invite: String,
    github: String,
    colour: u32,
}

static BOT_CONFIG: OnceCell<BotConfig> = OnceCell::new();

impl BotConfig {
    pub(crate) fn set(config_path: &str) {
        let config: BotConfig =
            toml::from_str(&fs::read_to_string(config_path).unwrap_or_else(|err| {
                if err.kind() == io::ErrorKind::NotFound {
                    fs::write(config_path, DEFAULT_CONFIG).expect(&format!(
                        "Couldn't write the default config, write it manually please:\n{}",
                        DEFAULT_CONFIG
                    ));
                    panic!("Created the default config, edit it and restart please");
                } else {
                    panic!(err)
                }
            }))
            .expect("Looks like something is wrong with your config");

        BOT_CONFIG
            .set(config)
            .unwrap_or_else(|_| panic!("Couldn't set the config to BOT_CONFIG"));
    }

    pub fn get() -> Option<&'static BotConfig> {
        BOT_CONFIG.get()
    }

    pub fn token(&self) -> &String {
        &self.token
    }
    pub fn invite(&self) -> &String {
        &self.invite
    }
    pub fn github(&self) -> &String {
        &self.github
    }
    pub fn colour(&self) -> u32 {
        self.colour
    }
}

pub struct BotInfo {
    owner: UserId,
    user: UserId,
    name: String,
    description: String,
}

static BOT_INFO: OnceCell<BotInfo> = OnceCell::new();

impl BotInfo {
    pub(crate) async fn set(token: &str) {
        let http = Http::new_with_token(token);
        let app_info = http
            .get_current_application_info()
            .await
            .expect("Couldn't get application info:");
        let name = http
            .get_current_user()
            .await
            .expect("Couldn't get current user")
            .name;

        let info = BotInfo {
            owner: app_info.owner.id,
            user: app_info.id,
            name,
            description: app_info.description,
        };

        BOT_INFO
            .set(info)
            .unwrap_or_else(|_| panic!("Couldn't set BotInfo to BOT_INFO"))
    }

    pub fn get() -> Option<&'static BotInfo> {
        BOT_INFO.get()
    }

    pub fn owner(&self) -> UserId {
        self.owner
    }
    pub fn user(&self) -> UserId {
        self.user
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn description(&self) -> &String {
        &self.description
    }
}

async fn set_db() -> SqlitePool {
    let db = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("database.sqlite")
            .create_if_missing(true),
    )
    .await
    .expect("Couldn't connect to the database");

    query!(
        "CREATE TABLE IF NOT EXISTS prefixes (
        guild_id INTEGER PRIMARY KEY,
        prefix TEXT
    ) WITHOUT ROWID"
    )
    .execute(&db)
    .await
    .expect("Couldn't create the prefix table");

    db
}

async fn prefix_check(db: &SqlitePool, ctx: &Context, msg: &Message) -> Option<String> {
    let data = ctx.data.read().await;

    if let Some(guild_id) = msg.guild_id {
        let guild_id_int = guild_id.0 as i64;
        match query!(
            "SELECT prefix FROM prefixes WHERE guild_id = ?",
            guild_id_int
        )
        .fetch_optional(db)
        .await
        {
            Err(err) => {
                log(
                    ctx,
                    format!(
                        "Couldn't fetch prefix from the database for the prefix check: {}",
                        err
                    ),
                )
                .await;
                None
            }
            Ok(row) => match row {
                Some(row) => row.prefix,
                None => None,
            },
        }
    } else {
        None
    }
}

async fn set_framework(db: &SqlitePool, bot_id: UserId, owner_id: UserId) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.prefix("")
                .no_dm_prefix(true)
                .case_insensitivity(true)
                .on_mention(Some(bot_id))
                .owners(vec![owner_id].into_iter().collect())
                .dynamic_prefix(|ctx, msg| Box::pin(prefix_check(db, ctx, msg)))
        })
        .on_dispatch_error(command_error::handle)
        .bucket("general", |b| {
            b.limit_for(LimitedFor::Channel)
                .await_ratelimits(1)
                .delay_action(command_error::delay_action)
                .time_span(600)
                .limit(10)
        })
        .await
        .bucket("expensive", |b| {
            b.limit_for(LimitedFor::Guild)
                .await_ratelimits(1)
                .delay_action(command_error::delay_action)
                .time_span(3600)
                .limit(10)
        })
        .await
        .help(&commands::CMD_HELP)
        .group(&commands::GENERAL_GROUP)
}
