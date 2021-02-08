use crate::commands::MASTER_GROUP;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serenity::{http::client::Http, model::id::UserId, prelude::TypeMapKey};
use sqlx::{query, sqlite::SqliteConnectOptions, SqlitePool};
use std::{convert::TryFrom, fs, io};

const DEFAULT_CONFIG: &'static str =
    "# The token of the bot: https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account
token = \"TOKEN HERE\"

# The invite link for the bot: https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot
invite = \"https://discord.com/api/oauth2/THE REST OF THE LINK HERE\"

# The link of the bot's repo's GitHub's page
github = \"https://github.com/USER NAME HERE/REPO NAME HERE\"

# The colour utils::send_embed() will use if is_error is false: https://www.checkyourmath.com/convert/color/rgb_decimal.php
colour = 11771355";

#[derive(Deserialize)]
pub struct BotConfig {
    token: String,
    log_file: String,
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
    pub fn log_file(&self) -> &String {
        &self.log_file
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

pub struct CmdInfo {
    commands: Vec<&'static str>,
    longest_len: u8,
}

static CMD_INFO: OnceCell<CmdInfo> = OnceCell::new();

impl CmdInfo {
    pub(crate) fn set() {
        let mut commands = MASTER_GROUP
            .options
            .sub_groups
            .iter()
            .flat_map(|g| g.options.commands.iter().flat_map(|c| c.options.names))
            .copied()
            .collect::<Vec<_>>();
        commands.push("help");

        let longest_len = u8::try_from(
            commands
                .iter()
                .map(|s| s.chars().count())
                .max()
                .expect("No commands found"),
        )
        .expect("Command name too long")
            + 10;

        CMD_INFO
            .set(CmdInfo {
                commands,
                longest_len,
            })
            .unwrap_or_else(|_| panic!("Couldn't set CmdInfo to CMD_INFO"))
    }

    pub fn get() -> Option<&'static CmdInfo> {
        CMD_INFO.get()
    }

    pub fn commands(&self) -> &Vec<&'static str> {
        &self.commands
    }
    pub fn longest_len(&self) -> u8 {
        self.longest_len
    }
}

pub struct SqlitePoolKey;
impl TypeMapKey for SqlitePoolKey {
    type Value = SqlitePool;
}

pub(crate) async fn set_db() -> SqlitePool {
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
