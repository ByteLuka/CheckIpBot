mod commands;
mod logging;

use std::env;
use std::process::ExitCode;

use serenity::all::{Client, Command, Context, EventHandler, GatewayIntents, Interaction, Ready};
use serenity::async_trait;
use tracing::{error, info, warn};

struct Handler {
    http: reqwest::Client,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        for command in commands::all() {
            if let Err(err) = Command::create_global_command(&ctx.http, command).await {
                error!("Failed to register slash command: {err}");
            }
        }
        info!("Bot online: {}", ready.user.tag());
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };
        match command.data.name.as_str() {
            commands::ip::NAME => commands::ip::run(&ctx, &command, &self.http).await,
            other => warn!("Received unknown command: {other}"),
        }
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm = signal(SignalKind::terminate()).expect("install SIGTERM handler");
        tokio::select! {
            _ = sigterm.recv() => {}
            _ = tokio::signal::ctrl_c() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    logging::init();

    let Ok(token) = env::var("DISCORD_TOKEN") else {
        error!("DISCORD_TOKEN environment variable is not set");
        return ExitCode::FAILURE;
    };

    let handler = Handler {
        http: reqwest::Client::new(),
    };
    // Slash commands are delivered without any gateway intents.
    let mut client = match Client::builder(&token, GatewayIntents::empty())
        .event_handler(handler)
        .await
    {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to create Discord client: {err}");
            return ExitCode::FAILURE;
        }
    };

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        shutdown_signal().await;
        info!("Shutdown signal received, disconnecting");
        shard_manager.shutdown_all().await;
    });

    if let Err(err) = client.start().await {
        error!("Discord client stopped: {err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
