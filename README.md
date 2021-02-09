# discord-base
The repo for the crate I use to build my bots on top of, made using [Serenity](https://github.com/serenity-rs/serenity) in Rust!  

## How to set it up
1. IDK myself yet..
2. Go to [the application page](https://discord.com/developers/applications), select your bot and set the description. The `info` command will use that and the account that has that application
3. Then check usage right under here

## How to use it
#### `start(config_path: &str)`
- Does everything to start up the bot
- Or creates the default config file if it's not there, then you should edit that file
- Use it as the first thing in your `#[tokio::main]` function
```rust
use discord_base::start;
start("config.toml").await;
```
#### `log(ctx: &Context, msg: impl Display + AsRef<[u8]>)`
- DMs the owner the message  
- Reverts to `print_and_write` if it failed, also telling why it failed
```rust
use discord_base::log;
log(&ctx, "Heya owner! How are you?").await;
```
#### `print_and_write(msg: impl Display)`
- Prints and writes the message and the current time in UTC to the log_file set in the config
```rust
/*Assume the config file has this in it:
log_file = "logs.txt"
*/
use discord_base::print_and_write;
print_and_write("Magic text appearing in the text file");
/* The file logs.txt is now created and has these in it:
5 February Friday 10:38:20: Magic text appearing in the text file

*/
// Pretend five seconds passed
print_and_write("And even more text here");
/* The file now has these in it:
5 February Friday 10:38:20: Magic text appearing in the text file

5 February Friday 10:38:25: And even more text here

*/
```
#### `send_embed(ctx: &Context, reply: &Message, is_error: bool, mut embed: CreateEmbed)`
- Sends the embed to the channel reply is in, colours it with the colour you gave in your config file  
- Unless `is_error` is `true` then it's the error colour (That's all that parameter does)  
- If it's failed to send though, tries to tell why in the server in plain text (without embeds)  
- Or DMs the reply's sender if that also fails
```rust
use discord_base::send_embed;
let mut embed = CreateEmbed::default();
embed.title("This is my title");
send_embed(ctx, msg, true, embed).await;
```

## What else it does
### Error handling
Everything that's done in this crate follows these principals:
- If the action isn't expected by the user, don't inform them even if it fails
- If anything else, do inform them. Fall back to DMing the user if informing the user failed
- If it's a user error, tell them the error and how to fix it if it's not obvious
- If it's a bot error, tell them how to report it and what they can do and inform the owner of the bot. Fall back to printing and logging in a file
- If we can't be sure, tell them the error and how to report it if it looks like a bot error  

These, combined with Rust's safety, ensure the best user experience. Most of these errors could be handled by falling back (Not using embeds, DMing the user etc.) but this overcomplicates the bot and makes it inconsistent, without forcing the user to fix it, so it's not a good practice
### Optimisation
- I've tried my best to use statics and avoid `await`s
- Also adding buckets and rate limit handling to ensure it isn't abused
- Combined with Rust's and SQLite's performance, the bot should be really lightweight  

I can't say fast because we'll be bottlenecked by Discord anyway. It's still definitely fast enough that the user won't notice at all
### Presence
- Sets the presence to `Playing a game: @[bot's username] help` (This looks much better than other presences Discord allows)
### Commands
All these don't have a prefix so they're run with `@bot [command]`. You set your own prefix for the groups you create  
(I made it this way because usually only these commands collide with other bots so you can use convenient prefixes for your own commands)
#### Info command
- An `info` command that gets the desciption and owner from [the application page](https://discord.com/developers/applications) and the GitHub page and invite link from the config file
#### Prefix command
- A `prefix` command that sets the prefix for the guild, which works for every command in addition to `@bot` and the prefixes you set for your groups
- This isn't as simple as it seems. It means the bot has to check if the message starts with its prefix in that server for every message that's sent
- To further optimise this, the bot first checks the message's first `max prefix length (10) + longest command's length` characters if it includes any of the commands, if not it doesn't unnecessarily check since there's no way the message includes a command
#### Help command
(Provided by Serenity's standard framework)
- A nice help command, listing all the other commands and their groups
- Gives more information about a command with `help [command]`
- Suggests similar commands if `help [command]` is.. similar to another command

## Who I am
Just some (currently) 17 years old girl from Turkey coding  
Started with Python, gave a shot to JS but now that I know Rust exists never going back  
Basically all I did was Discord bots (at least at the time of writing)  
License and stuff I don't care, neither should you but contact me if you want to ask anything

## How you can contact me
- Very Fast: Discord: aria#7553
- Slow: GitHub issues or something
- Way slower (or never, if you're unlucky): wentaas@gmail.com


## Ideas I had but decided not to implement
### Handling permissions
Too expensive, limited, bad for UX, unnecessary and inconsistent. Doing proper error checking and informing the user on an error is just a better option
### Localisation
Makes everything more expensive, you now have to check the language for every message you're sending, which means you can't use any static string. Community translation will always be inconsistent, slow and incomplete. Most users wouldn't expect or use it. Having separate bots for each language is just a better option
### Customisation
Again makes everything more expensive, since it means you can't use any static string. If someone is hosting the bot, they most likely have enough knowledge to search for a string in the source and replace it then build. It isn't necessary at all and I still tried to include customisation when it didn't mean a performance loss
