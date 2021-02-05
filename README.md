# discord-base
The repo for the crate I use to build my bots on top of, made using [Serenity](https://github.com/serenity-rs/serenity) in Rust!  

## Set up?
- IDK myself yet..
- Install and start the server for [PostgreSQL](https://www.postgresql.org/download)
- It uses the owner and description in [the application page](https://discord.com/developers/applications) so edit them
- Then check usage right under here

## Usage
### `start(config_path: &str, postgresql_port: u16)`
Does everything to start up the bot
or creates the default config file if it's not there, then you should edit that file
Use it as the first thing in your `#[tokio::main]` function
```rs
start("config.toml", 5432).await;
```
### `utils::log(ctx: &Context, msg: impl Display + AsRef<[u8]>)`
DMs the owner the message
Reverts to `print_and_write()` if it failed, also telling why it failed
```rs
utils::log(ctx, "Heya owner! How are you?").await;
```
### `utils::print_and_write(msg: impl Display)`
Prints the message and when it was called and writes them to a file called "couldnt_dm.txt"
```rs
utils::print_and_write("Magic text appearing in the text file");
/* The file couldnt_dm.txt is now created and has these in it:
5 February Friday 10:38:20: Magic text appearing in the text file

*/
// pretend five seconds passed
utils::print_and_write("And even more text here");
/* The file now has these in it:
5 February Friday 10:38:20: Magic text appearing in the text file

5 February Friday 10:38:25: And even more text here

*/
```
### `utils::send_embed(ctx: &Context, reply: &Message, is_error: bool, mut embed: CreateEmbed)`
Sends the embed to the channel reply is in, colours it with the colour you gave in your config file
Unless `is_error` is `true` then it's the error colour *(That's all that parameter does)*
If it's failed to send though, tries to tell why in the server in plain text
or DMs the reply's sender if that also fails
```rs
let mut embed = CreateEmbed::default();
embed.title("This is my title");
utils::send_embed(ctx, msg, true, embed).await;
```

## What else it does
*All these don't have a prefix so they're run with @bot help, you set your own prefix for the groups you create (I made it this way because usually only these commands collide with other bots so you can use convenient prefixes for your commands)*
- Sets the presence to `Playing a game: @BOT'S USERNAME HERE help` (This looks much better than other presences Discord allows)
- An `info` command with aliases `about, invite, inv` that gets the desciption and owner from [the application page](https://discord.com/developers/applications) and the GitHub page from the config file
*And these from Serenity's standard_framework:*
- A nice help command with aliases `commands, cmds`, listing all the other commands and their groups
- Give more information about a command with `help [command name]`
- Suggest similar commands if `help [command name]` is.. well.. similar to another command

## What it will be able to do (soon™)
- Guild specific prefixes (a `prefix` command)
- Showing the prefix with the help command
- Handling permissions
- Localisation, different languages specific to guilds, channels, users (and letting others easily translate them)

## Who am I?
Just some (currently) 17 years old girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither should you but contact me if you want to ask anything, or donate for some reason

## Contact
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com
