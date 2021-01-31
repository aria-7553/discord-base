# discord-base
The repo I clone and build my bots on top of, made using [Serenity](https://github.com/serenity-rs/serenity) in Rust!  
Yes, this could be a crate but I find it more convenient this way and I do realise no one else is going to use this

## Set up?
Just clone this repo, and in your terminal `cd` into the folder with `Cargo.toml`, and run it with `cargo run` (Yes that's it, it'll guide you through the rest!)  
Then you code on top of it of course

## Customise it? You can:
- obviously edit the config file
- rename the folder and edit `Cargo.toml` however you want (don't touch the dependencies though)
- change the path/name of the config file at `Config::set(path: &str)` in `main.rs`
- change the default config that's written at `DEFAULT_CONFIG` in `config_parser.rs`
- generally edit any string ever used by the program by just searching for it and editing it
- and just do whatever else you want, make sure to edit though, of course

## What it can do (for now)
- Print and DM the owner when it's ready

## To do list:
- Set the panic hook (what to do when the program panics)
- Global prefix from the config file
- `help`, `commands`, `info`, `about`, `invite` commands
- Setting the bot's activity status to `Listening to {prefix}help`
- Guild specific prefixes (a `prefix` command)
- Not requiring a prefix if the bot is mentioned (Editing the bot's activity status accordingly)
- Handling permissions
- Localisation, different languages specific to guilds, channels, users (and letting others easily translate them)

## Who am I?
Just some 17 years old (currently) girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither do you but contact me if you want to ask anything, or donate for some reason

## Contact
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com
