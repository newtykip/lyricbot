use views::ViewContainer;

pub mod views;

pub type CommandSender = crossbeam_channel::Sender<Command>;

/// Commands that can be sent to the main loop.
pub enum Command {
    /// Change the current view.
    ChangeView { view: ViewContainer },
    /// Go back to the previous view.
    BackView,
    /// Stop the main loop.
    Stop,
    Confirm {
        message: String,
        previous: ViewContainer,
    },
}
