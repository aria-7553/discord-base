use once_cell::sync::OnceCell;
use serde::Deserialize;
use serenity::{http::client::Http, model::id::UserId};
use std::{fs, io};

const DEFAULT_CONFIG: &'static str =
    "# The token of the bot: https://discordpy.readthedocs.io/en/latest/discord.html#creating-a-bot-account
token = \"TOKEN HERE\"

# The invite link for the bot: https://discordpy.readthedocs.io/en/latest/discord.html#inviting-your-bot
invite = \"https://discord.com/api/oauth2/ THE REST OF THE LINK HERE\"

# The link of the bot's repo's GitHub's page
github = \"https://github.com/ USER NAME HERE/REPO NAME HERE\"";

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub invite: String,
    pub github: String,
}

static BOT_CONFIG: OnceCell<BotConfig> = OnceCell::new();

impl BotConfig {
    pub fn set(config_path: &str) {
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
}

pub struct BotInfo {
    pub owner_id: UserId,
    pub bot_id: UserId,
    description: String,
}

static BOT_INFO: OnceCell<BotInfo> = OnceCell::new();

impl BotInfo {
    pub async fn set(token: &str) {
        let app_info = Http::new_with_token(token)
            .get_current_application_info()
            .await
            .expect("Couldn't access application info:");

        let info = BotInfo {
            owner_id: app_info.owner.id,
            bot_id: app_info.id,
            description: app_info.description,
        };

        BOT_INFO
            .set(info)
            .unwrap_or_else(|_| panic!("Couldn't set the info to BOT_INFO"))
    }

    pub fn get() -> Option<&'static BotInfo> {
        BOT_INFO.get()
    }
}
