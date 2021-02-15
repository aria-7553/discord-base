initSidebarItems({"constant":[["DEFAULT_CONFIG","The default config to be written when creating a config file"]],"fn":[["set_db","Opens a connection pool to the database file at the config file, creating it if it doesn't existRuns the query given, creating the `prefixes` table (You should add your own things to it to prepare the database)DO NOT modify the `prefixes` table yourself!PanicsIf BotConfig isn't initialisedOr if connecting to it failed"]],"static":[["BOT_CONFIG","The static to hold the struct, so that it's global"],["BOT_INFO","The static to hold `BotInfo`, so that it's global"],["CMD_INFO","The static to hold `BotInfo`, so that it's global"]],"struct":[["BotConfig","The struct to hold the values in the config file"],["BotInfo","The struct to hold the information found from the application so that we can set it to a static to avoid API requests"],["CmdInfo","The struct to hold the information about commands, found from the `Master` group so that we can set it to a static to avoid iterating every time"],["SqlitePoolKey","The struct to implement TypeMapKey for, use this to get the SqlitePool from `ctx.data`"]]});