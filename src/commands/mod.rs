pub mod ip;

use serenity::all::CreateCommand;

/// All slash commands the bot registers on startup.
pub fn all() -> Vec<CreateCommand> {
    vec![ip::register()]
}
