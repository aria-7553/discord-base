var searchIndex = JSON.parse('{\
"discord_base":{"doc":"","i":[[3,"Master","discord_base","The hidden group for all the commands to be added to",null,null],[3,"General","","The group for the commands provided by default",null,null],[3,"Handler","","The event handler struct to implement EventHandler for",null,null],[5,"send_embed","","Sets the colour of the `embed` to `11534368` (The baseline…",null,[[["context",3],["createembed",3],["message",3]]]],[5,"log","","DMs the owner of the bot, as in the application info, with…",null,[[["context",3]]]],[5,"print_and_write","","Prints the `msg` and the timestamp and appends it (or…",null,[[]]],[5,"set_dir","","Sets the working directory to the directory of the binary,…",null,[[]]],[0,"cmd_error","","The module for error handling of the commands",null,null],[5,"handle","discord_base::cmd_error","The function to run on a user-related command error.…",null,[[["message",3],["dispatcherror",4],["context",3]],["boxfuture",6]]],[5,"delay_action","","The function to run if the user exceeded the bucket…",null,[[["message",3],["context",3]],["boxfuture",6]]],[0,"cmd_help","discord_base","The module for the `help` command",null,null],[5,"cmd_help","discord_base::cmd_help","",null,[[["args",3],["userid",3],["message",3],["context",3],["hashset",3],["helpoptions",3]],[["commandresult",6],["boxfuture",6]]]],[7,"CMD_HELP__OPTIONS","","",null,null],[7,"CMD_HELP","","",null,null],[0,"cmd_info","discord_base","The module for the `info` command",null,null],[5,"cmd_info","discord_base::cmd_info","",null,[[["message",3],["args",3],["context",3]],[["commandresult",6],["boxfuture",6]]]],[7,"CMD_INFO_COMMAND_OPTIONS","","",null,null],[7,"CMD_INFO_COMMAND","","",null,null],[0,"cmd_prefix","discord_base","The module for the `prefix` command",null,null],[5,"cmd_prefix","discord_base::cmd_prefix","",null,[[["message",3],["args",3],["context",3]],[["commandresult",6],["boxfuture",6]]]],[5,"prefix_check","","The function to run to get the dynamic prefix",null,[[["context",3],["message",3]]]],[7,"CMD_PREFIX_COMMAND_OPTIONS","","",null,null],[7,"CMD_PREFIX_COMMAND","","",null,null],[0,"globals","discord_base","The module for the statics and structs to save the statics…",null,null],[3,"SqlitePoolKey","discord_base::globals","The struct to implement TypeMapKey for, use this to get…",null,null],[3,"BotConfig","","The struct to hold the values in the config file",null,null],[12,"token","","",0,null],[12,"log_file","","",0,null],[12,"database_file","","",0,null],[12,"invite","","",0,null],[12,"github","","",0,null],[12,"colour","","",0,null],[3,"BotInfo","","The struct to hold the information found from the…",null,null],[12,"owner","","",1,null],[12,"user","","",1,null],[12,"name","","",1,null],[12,"description","","",1,null],[3,"CmdInfo","","The struct to hold the information about commands, found…",null,null],[12,"cmds","","",2,null],[12,"longest_len","","",2,null],[12,"custom_cmds","","",2,null],[5,"set_db","","Opens a connection pool to the database file at the config…",null,[[]]],[7,"BOT_CONFIG","","The static to hold the struct, so that it\'s global",null,null],[7,"BOT_INFO","","The static to hold `BotInfo`, so that it\'s global",null,null],[7,"CMD_INFO","","The static to hold `BotInfo`, so that it\'s global",null,null],[17,"DEFAULT_CONFIG","","The default config to be written when creating a config file",null,null],[11,"set","","Serialises the values in the config file at the…",0,[[]]],[11,"get","","The getter for BOT_CONFIG, returning a shared reference to…",0,[[],[["botconfig",3],["option",4]]]],[11,"token","","The getter for the `token` field, to be used with `get()`",0,[[],["string",3]]],[11,"log_file","","The getter for the `log_file` field, to be used with `get()`",0,[[],["string",3]]],[11,"invite","","The getter for the `invite` field, to be used with `get()`",0,[[],["string",3]]],[11,"github","","The getter for the `github` field, to be used with `get()`",0,[[],["string",3]]],[11,"colour","","The getter for the `colour` field, to be used with `get()`",0,[[]]],[11,"set","","Creates an Http instance with the `token`Gets the…",1,[[]]],[11,"get","","The getter for BOT_INFO, returning a shared reference to…",1,[[],[["botinfo",3],["option",4]]]],[11,"owner","","The getter for the `owner` field, to be used with `get()`",1,[[],["userid",3]]],[11,"user","","The getter for the `user` field, to be used with `get()`",1,[[],["userid",3]]],[11,"name","","The getter for the `name` field, to be used with `get()`",1,[[],["string",3]]],[11,"description","","The getter for the `description` field, to be used with…",1,[[],["string",3]]],[11,"set","","Iterates through the sub groups of `Master`, flattening…",2,[[]]],[11,"get","","The getter for BOT_INFO, returning a shared reference to…",2,[[],[["option",4],["cmdinfo",3]]]],[11,"cmds","","The getter for the `cmds` field, to be used with `get()`",2,[[],["vec",3]]],[11,"longest_len","","The getter for the `longest_len` field, to be used with…",2,[[]]],[11,"custom_cmds","","The getter for the `custom_cmds` field, to be used with…",2,[[],["vec",3]]],[7,"MASTER_GROUP_OPTIONS","discord_base","The hidden group for all the commands to be added to",null,null],[7,"MASTER_GROUP","","The hidden group for all the commands to be added to",null,null],[7,"GENERAL_GROUP_OPTIONS","","The group for the commands provided by default",null,null],[7,"GENERAL_GROUP","","The group for the commands provided by default",null,null],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"from","discord_base::globals","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"cache_ready","discord_base","Triggered once when the bot is ready, unlike `ready`,…",5,[[["context",3],["vec",3],["guildid",3]],[["pin",3],["box",3]]]]],"p":[[3,"BotConfig"],[3,"BotInfo"],[3,"CmdInfo"],[3,"Master"],[3,"General"],[3,"Handler"],[3,"SqlitePoolKey"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);