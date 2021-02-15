# discord-base
[![](https://img.shields.io/static/v1?color=f48fb1&labelColor=f48fb1&label=discord&message=join&logo=discord&logoColor=ffffff&style=for-the-badge)](https://discord.gg/u6NyRUnNED)  
[![](https://img.shields.io/static/v1?color=f48fb1&labelColor=f48fb1&label=docs‎‎‎‎‎‎‎‎‎‎‎&message=read&logo=rust&style=for-the-badge)](https://aria-7553.github.io/discord-base/discord_base)

The repo I clone and build my bots on top of. Made with love using [Serenity](https://github.com/serenity-rs/serenity) in Rust!

## Use it
*This assumes you have Rust installed on your system and at least basic knowledge about the language since this repo is for developers only*
1. Clone this repository (Download all the files into a folder)
2. Open your terminal and `cd` into to the folder with `Cargo.toml` in it
3. Type `cargo run` in the terminal and wait for it to build and run
4. It'll create the config file ending with `config.toml` at `target/debug/` *(If it isn't there, read the stuff in the window to see the errors)*
5. Edit it with your text editor *(Notepad, TextEdit, nano etc.)* *(Instructions are inside)* *(The end user is able to edit it too, if you'll be sharing your bot)*
6. Go to [the application page](https://discord.com/developers/applications), select your bot and set the description. The `info` command will use that and the account that has that application
7. [Read the docs](https://aria-7553.github.io/discord-base/discord_base/)
8. Do whatever you want with the code to build your bot on top
9. Build it with `cargo build` as usual. It'll open in a terminal, close it and the bot shuts down

## What it lets you do
*This provides the basic information about what it can do, usage on how to implement these are documented in `.rs` files*
### Database
- Loads the SQLite database from the name in the config file or creates it if it doesn't exists *(This repo uses [SQLx](https://github.com/launchbadge/sqlx) because of its stable `async` features)*
- Saves the connection pool into `ctx.data` so you can access the database anywhere
### Embeds
- Provides a function to send an embed directly
- Or if it fails, inform the author of the message and the owner of the bot if necessary, falling back to logging into a file *(See [Error Handling](#error-handling) for more)*

## What it does

### Presence
- Sets the presence to `Playing a game: @[bot's username] help` (This looks much better than other presences Discord allows)

### General Commands
All these don't have a prefix so they're run with `@bot [command]`. You set your own prefix for the groups you create

*(I made it this way because usually only these commands collide with other bots so that you can use `.` as the prefix for your own commands)*

#### Help command

*(Provided by Serenity's standard framework)*

- A nice help command, listing all the other commands and their groups
- Gives more information about a command with `help [command]`
- Suggests similar commands if `help [command]` is.. similar to another command

#### Info command
- An `info` command that gets the description and owner from [the application page](https://discord.com/developers/applications) and the GitHub page and invite link from the config file

#### Prefix command
- A `prefix` command that sets the prefix for the guild, which works for every command in addition to `@bot` and the prefixes you set for your groups
- This isn't as simple as it seems. It means the bot has to check if the message starts with its prefix in that server for every message that's sent
- To further optimise this, the bot first checks the message's first `max prefix length (10) + longest command's length` characters if it includes any of the commands, if not it doesn't unnecessarily check since there's no way the message includes a command

### Optimisation
- I've tried my best to use statics and avoid `await`s
- Also adding buckets and rate limit handling to ensure it isn't abused
- Combined with Rust's and SQLite's performance, the bot should be really lightweight

*I can't say fast because we'll be bottlenecked by Discord anyway. It's still as light and fast as it can be*

### Error handling
Everything that's done follows these principals:
- If the action isn't expected by the user, don't inform them even if it fails
- If anything else, do inform them. Fall back to DMing the user if informing the user failed
- If it's a user error, tell them the error and how to fix it if it's not obvious
- If it's a bot error, tell them how to report it and what they can do and inform the owner of the bot. Fall back to printing and logging in a file
- If we can't be sure, tell them the error and how to report it if it looks like a bot error  

These, combined with Rust's safety, ensure the best user experience. Most of these errors could be handled by falling back (Not using embeds, DMing the user etc.) but this overcomplicates the bot and makes it inconsistent, instead of forcing the user to fix it, so it's not a good practice

## Who I am
Just some (currently) 17 years old girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither should you but contact me if you want to ask anything

## How you can contact me
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com

## Thank yous
A huge thank you to everyone in the [Serenity Discord Server](https://discord.gg/9X7vCus) and the [Rust Community Discord Server](https://discord.com/invite/aVESxV8) for their continuous help and bearing with my smooth brain

## Ideas I had but decided not to implement

### Handling permissions
Too expensive, limited, bad for UX, unnecessary and inconsistent. Doing proper error checking and informing the user on an error is just a better option

### Localisation
Makes everything more expensive, you now have to check the language for every message you're sending, which means you can't use any static string. Community translation will always be inconsistent, slow and incomplete. Most users wouldn't expect or use it. Having separate bots for each language is just a better option

### Customisation
Again makes everything more expensive, since it means you can't use any static string. If someone is hosting the bot, they most likely have enough knowledge to search for a string in the source and replace it then build. It isn't necessary at all and I still tried to include customisation when it didn't mean a performance loss
