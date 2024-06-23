use super::prelude::*;
use ratatui::widgets::{block::Title, Block, BorderType, Borders, Padding, Tabs};

#[derive(Clone)]
pub struct Confirmation {
    message: String,
    yes: bool,
}

impl Confirmation {
    pub fn new(message: String) -> Self {
        Self { message, yes: true }
    }
}

impl View for Confirmation {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical(Constraint::from_fills([1, 2, 1])).split(area);
        let block = Block::new()
            .title(Title::from(self.message.clone()).alignment(Alignment::Center))
            .padding(Padding::left(
                area.width / 2 - self.message.len() as u16 / 4,
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let tabs = Tabs::new(vec![" Yes ", " No "])
            .select(if self.yes { 0 } else { 1 })
            .divider("")
            .block(block);
        frame.render_widget(tabs, layout[1]);
    }

    fn keypress(&mut self, key: KeyEvent, view_tx: &CommandSender) -> Result<()> {
        match key.code {
            KeyCode::Left | KeyCode::Char('a') => {
                self.yes = true;
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.yes = false;
            }
            KeyCode::Enter => {
                view_tx.send(if self.yes {
                    Command::Stop
                } else {
                    Command::BackView
                })?;
            }
            KeyCode::Esc => {
                view_tx.send(Command::BackView)?;
            }
            _ => {}
        }

        Ok(())
    }
}
