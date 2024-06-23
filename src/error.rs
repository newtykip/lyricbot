use tokio::sync::mpsc::error::SendError;

use crate::tui::Command;

//? perhaps consider adding color-eyre to make the output nicer, or an adjacent crate?
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("There was an error while changing views.")]
    ChangeView(#[from] SendError<Command>),
    #[error("There was an error while checking the state of the confirmation prompt.")]
    Confirmation(#[from] SendError<()>),
    #[error("There was an error while decoding profile data.")]
    Decode(#[from] prost::DecodeError),
    #[error("No authorization token found for {0}.")]
    NoAuth(String),
    #[cfg(feature = "discord")]
    #[error("There was an error while creating a Discord client.")]
    DiscordClient(#[from] poise::serenity_prelude::Error),
}

#[cfg(feature = "discord")]
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Serenity(#[from] poise::serenity_prelude::Error),
}
