use once_cell::sync::OnceCell;
use serde::Deserialize;
use serenity::{http::client::Http, model::id::UserId};
use std::{fs, io};

const DEFAULT_CONFIG: &'static str =
    "# The token of the bot: https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account
token = \"TOKEN HERE\"

# The prefix for your bot (help and info commands won't use this prefix):
prefix = \".\"

# The invite link for the bot: https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot
invite = \"https://discord.com/api/oauth2/THE REST OF THE LINK HERE\"

# The link of the bot's repo's GitHub's page
github = \"https://github.com/USER NAME HERE/REPO NAME HERE\"

# The colour utils::send_embed() will use if is_error is false: https://www.checkyourmath.com/convert/color/rgb_decimal.php
colour = 11771355";

#[derive(Deserialize)]
pub(crate) struct BotConfig {
    token: String,
    prefix: String,
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
    pub fn prefix(&self) -> &String {
        &self.prefix
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

pub(crate) struct BotInfo {
    owner: UserId,
    user: UserId,
    description: String,
    error_colour: u32,
}

static BOT_INFO: OnceCell<BotInfo> = OnceCell::new();

impl BotInfo {
    pub(crate) async fn set(token: &str) {
        let app_info = Http::new_with_token(token)
            .get_current_application_info()
            .await
            .expect("Couldn't access application info:");

        let info = BotInfo {
            owner: app_info.owner.id,
            user: app_info.id,
            description: app_info.description,
            error_colour: 11534368,
        };

        BOT_INFO
            .set(info)
            .unwrap_or_else(|_| panic!("Couldn't set the info to BOT_INFO"))
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
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn error_colour(&self) -> u32 {
        self.error_colour
    }
}
