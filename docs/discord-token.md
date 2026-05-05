# Creating a Discord Bot Token

This guide walks through creating a Discord application, setting up a bot, and obtaining a token.

## 1. Create an application

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Click **New Application**, give it a name (e.g. `CheckIpBot`), and confirm

## 2. Create a bot user

1. In the left sidebar, click **Bot**
2. Click **Add Bot** and confirm
3. Under **Privileged Gateway Intents**, no additional intents are required for this bot — leave them all off

## 3. Copy the token

1. On the **Bot** page, click **Reset Token** (you may need to confirm with 2FA)
2. Copy the token immediately — it is only shown once
3. Store it somewhere safe; treat it like a password

If you lose the token, reset it again from the same page. Any previously issued token is immediately invalidated.

## 4. Invite the bot to your server

1. In the left sidebar, click **OAuth2 → URL Generator**
2. Under **Scopes**, select `applications.commands`
3. No **Bot Permissions** are needed (the bot uses slash commands only)
4. Copy the generated URL, open it in a browser, and select the server to invite the bot to

## 5. Use the token

Pass the token to the bot via the `DISCORD_TOKEN` environment variable:

```bash
# Local development
DISCORD_TOKEN=<your-token> uv run python bot.py

# Kubernetes — see README.md for Helm installation options
```

Never commit the token to version control.
