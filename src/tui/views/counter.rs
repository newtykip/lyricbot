use crate::{
    change_view,
    tui::views::{View, ViewChange, ViewSender},
    Result,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::Paragraph};
use std::cell::Cell;

#[derive(Default)]
pub struct Counter {
    count: Cell<i8>,
}

impl View for Counter {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.count.get().to_string()), area);
    }

    fn keypress(&self, key: KeyEvent, view_tx: &ViewSender) -> Result<()> {
        match key.code {
            KeyCode::Char('=') | KeyCode::Char('-') => {
                let dx = if key.code == KeyCode::Char('=') {
                    1
                } else {
                    -1
                };
                self.count.set(self.count.get() + dx);
            }
            KeyCode::Backspace => {
                change_view!(view_tx, Counter)?;
            }
            KeyCode::Esc => {
                view_tx.send(ViewChange::Back)?;
            }
            _ => {}
        }

        Ok(())
    }
}
