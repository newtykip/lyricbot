pub mod confirmation;
pub mod counter;
mod prelude;

use super::CommandSender;
use crate::Result;
use crossterm::event::KeyEvent;
use dyn_clone::DynClone;
use ratatui::{layout::Rect, Frame};

pub type ViewContainer = Box<dyn View + Send>;

#[macro_export]
macro_rules! change_view {
    ($sender:expr, $view:ident) => {
        paste::paste! {
            $sender.send(Command::ChangeView {
                view: Box::new($crate::tui::views::[<$view:lower>]::$view::default())
            })
        }
    };
    ($sender:expr, $view:ident; $($param:expr),+) => {
        paste::paste! {
            $sender.send(Command::ChangeView {
                view: Box::new($crate::tui::views::[<$view:lower>]::$view::new($($param),+))
            })
        }
    };
}

pub trait View: DynClone {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn keypress(&mut self, key: KeyEvent, command_tx: &CommandSender) -> Result<()>;
}
