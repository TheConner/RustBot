# Discord Instructions
These instructions are a quick summary of what you need to do to get a Discord API token for the bot.

1. Log into the [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a new application, name it whatever you wish. `rustbot-dev` for example
3. Go to the bot section, and then click *Add Bot* to create a bot user
4. Once the user is created, *copy* the token. This is your secret that authenticates the bot, do not leak it anywhere
5. Under authorization flow, make sure *public bot* is disabled so that other people can't add your bot to their servers.

You now have a secret that can be used to authenticate your bot with discord. Next you need to add the bot to your own server:
6. Go to the oauth section, and then click on *url generator*
7. Under *scopes* click *bot*
8. Under the bot permissions section that pops up, select the following:
- Send messages
- Embed links
- Read messages / view channels
9. Copy the generated URL and open it
10. Add the bot to your server

And you should be off to the races!