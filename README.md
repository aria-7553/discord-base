# discord-base
The repo for the crate I use to build my bots on top of, made using [Serenity](https://github.com/serenity-rs/serenity) in Rust!  

## Set up?
IDK myself yet..
It uses the owner and description in [the application page](https://discord.com/developers/applications)
And you can only use `start()` and `utils::log()` and `utils::send_embed()`

## Customise it? You can:
- Obviously edit the config file
- Change the path or name of the config file at `start(path: &str)`
- It isn't very customisable yet, probably going to implement it later

## What it can do
- Print and DM the owner when it's ready
- `utils::log()` to DM the owner and `utils::send_embed()` to send a message with an embed
- Informs the commander (if available) and owner on an unexpected error if using `send_embed`
- A nice help command with aliases `commands, cmds`, listing all the other commands and their groups
- Give more information about a command with `help [command name]`
- Suggest similar commands if `[command name]` is.. well.. similar to another command
- An `info` command with aliases `about, invite` that gets the desciption and owner from [the application page](https://discord.com/developers/applications) and the GitHub page from the config file

## What it will be able to do (soon™)
- Logging into a file
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
