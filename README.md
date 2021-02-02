# discord-base
The repo I clone and build my bots on top of, made using [Serenity](https://github.com/serenity-rs/serenity) in Rust!  
Yes, this could be a crate but I find it more convenient this way and I do realise no one else is going to use this

## Set up?
Just clone this repo, and in your terminal `cd` into the folder with `Cargo.toml`, and run it with `cargo run` (Yes that's it, it'll guide you through the rest!)
It uses the owner and description in [the application page](https://discord.com/developers/applications)
Then you code on top of it of course

## Customise it? You can:
- Obviously edit the config file
- Rename the folder and edit `Cargo.toml` however you want (don't touch the dependencies though)
- Change the path or name of the config file at `Config::set(path: &str)` in `main.rs`
- Change the default config that's written at `DEFAULT_CONFIG` in `config_parser.rs`
- Change the prefix etc. in the `framework` variable in `main.rs`
- Generally edit any string ever used by the program by just searching for it and editing it
- Or search for `colour` to change them (It's decimal if it's a weird number)
- And just do whatever else you want, of course make sure to test it though

## What it can do (for now)
- Print and DM the owner when it's ready
- `log()` to DM the owner and `send_embed()` to send a message with an embed
- Informs the commander (if available) and owner on an unexpected error
- A nice help command with aliases `commands, cmds`, listing all the other commands and their groups
- Give more information about a command with `help [command name]`
- Suggest similar commands if `[command name]` is.. well.. similar to another command
- A WIP `info` command with aliases `about, invite` that gets the desciption from [the application page](https://discord.com/developers/applications)

## To do list:
- Making `help` and `info` run only if the bot is mentioned (no prefix)
- Setting the bot's activity status to `Listening to @mention help`
- Guild specific prefixes (a `prefix` command)
- Handling permissions
- Localisation, different languages specific to guilds, channels, users (and letting others easily translate them)

## Who am I?
Just some (currently) 17 years old girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither do you but contact me if you want to ask anything, or donate for some reason

## Contact
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com
