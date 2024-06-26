pub use crate::{
    change_view,
    tui::{views::View, Command, CommandSender},
    Result,
};
pub use async_trait::async_trait;
pub use crossterm::event::{KeyCode, KeyEvent};
pub use ratatui::{prelude::*, widgets::Paragraph};
