pub mod views;

use views::{confirmation::ConfirmData, ViewContainer};

pub type CommandSender = tokio::sync::mpsc::Sender<Command>;

/// Commands that can be sent to the main loop.
pub enum Command {
    /// Change the current view.
    ChangeView { view: ViewContainer },
    /// Go back to the previous view.
    BackView,
    /// Create a confirmation dialog.
    Confirm(String, ConfirmData),
    /// Quit the application.
    Quit,
}
