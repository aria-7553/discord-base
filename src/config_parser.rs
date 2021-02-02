use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{fs, io};

const DEFAULT_CONFIG: &'static str =
    "# The token of the bot: https://discordpy.readthedocs.io/en/latest/discord.html
token = \"TOKEN HERE\"";

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub invite: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    /// Reads the config and saves it to CONFIG or creates the DEFAULT_CONFIG
    pub fn set(config_path: &str) {
        let config: Config =
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

        CONFIG
            .set(config)
            .unwrap_or_else(|_| panic!("Couldn't set the config to CONFIG"));
    }

    pub fn get() -> Option<&'static Config> {
        CONFIG.get()
    }
}
