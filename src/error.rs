use crate::tui::Command;

// todo: make the error more friendly
// perhaps consider adding color-eyre to make the output nicer, or an adjacent crate?
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("There was an error while changing views.")]
    ChangeView(#[from] crossbeam_channel::SendError<Command>),
}
