use super::prelude::*;

#[derive(Default, Clone)]
pub struct Counter {
    count: i8,
}

#[async_trait]
impl View for Counter {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.count.to_string()), area);
    }

    async fn keypress(&mut self, key: KeyEvent, view_tx: &CommandSender) -> Result<()> {
        match key.code {
            KeyCode::Char('=') | KeyCode::Char('-') => {
                let dx = if key.code == KeyCode::Char('=') {
                    1
                } else {
                    -1
                };
                self.count = self.count.saturating_add(dx);
            }
            KeyCode::Backspace => {
                change_view!(view_tx, Counter).await?;
            }
            KeyCode::Esc => {
                view_tx.send(Command::BackView).await?;
            }
            _ => {}
        }

        Ok(())
    }
}
