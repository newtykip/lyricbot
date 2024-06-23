// default screen needs to be accessible in main.rs
pub mod counter;

use crate::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub type ViewContainer = Box<dyn View + Send>;
pub type ViewSender = crossbeam_channel::Sender<ViewChange>;

pub enum ViewChange {
    New(ViewContainer),
    Back,
}

#[macro_export]
macro_rules! change_view {
    ($sender:expr, $view:ident) => {
        paste::paste! {
            $sender.send(ViewChange::New(Box::new($crate::tui::views::[<$view:lower>]::$view::default())))
       }
    };
}

pub trait View {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn keypress(&self, key: KeyEvent, view_tx: &ViewSender) -> Result<()>;
}
