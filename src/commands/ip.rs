use std::time::Duration;

use serenity::all::{CommandInteraction, Context, CreateCommand, EditInteractionResponse};
use tracing::{error, info};

pub const NAME: &str = "get-ip";

const CHECK_IP_URL: &str = "https://checkip.amazonaws.com/";
const TIMEOUT: Duration = Duration::from_secs(10);

pub fn register() -> CreateCommand {
    CreateCommand::new(NAME).description("Check the server's public IP address")
}

async fn public_ip(http: &reqwest::Client) -> Result<String, reqwest::Error> {
    let response = http.get(CHECK_IP_URL).timeout(TIMEOUT).send().await?;
    let body = response.error_for_status()?.text().await?;
    Ok(body.trim().to_owned())
}

pub async fn run(ctx: &Context, command: &CommandInteraction, http: &reqwest::Client) {
    let guild = command
        .guild_id
        .map_or_else(|| "None".to_owned(), |id| id.to_string());
    info!("get-ip invoked by {} (guild={guild})", command.user.tag());

    // Acknowledge first, the lookup may take longer than Discord's 3 second reply window.
    if let Err(err) = command.defer(&ctx.http).await {
        error!("Failed to acknowledge get-ip: {err}");
        return;
    }

    let content = match public_ip(http).await {
        Ok(ip) => {
            info!("Resolved public IP: {ip}");
            format!("Current public IP: `{ip}`")
        }
        Err(err) => {
            error!("Failed to resolve public IP: {err}");
            "Could not determine the public IP address. Please try again later.".to_owned()
        }
    };

    let response = EditInteractionResponse::new().content(content);
    if let Err(err) = command.edit_response(&ctx.http, response).await {
        error!("Failed to send get-ip response: {err}");
    }
}
