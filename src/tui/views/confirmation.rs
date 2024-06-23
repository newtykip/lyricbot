use super::{prelude::*, ViewContainer};
use ratatui::widgets::{block::Title, Block, Padding, Tabs};
use tokio::sync::mpsc::Sender;

type Action = Box<dyn FnOnce() -> Option<Command> + Send>;

pub struct ConfirmData {
    pub action: Action,
    pub previous: ViewContainer,
}

#[derive(Clone)]
pub struct Confirmation {
    message: String,
    yes: bool,
    sender: Sender<()>,
}

impl Confirmation {
    pub fn new(message: String, sender: Sender<()>) -> Self {
        Self {
            message,
            yes: true,
            sender,
        }
    }
}

#[async_trait]
impl View for Confirmation {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical(Constraint::from_fills([1, 2, 1])).split(area);
        let block = Block::new()
            .title(Title::from(self.message.clone()).alignment(Alignment::Center))
            .padding(Padding::left(
                area.width / 2 - self.message.len() as u16 / 4,
            ));
        let tabs = Tabs::new(vec![" Yes ", " No "])
            .select(if self.yes { 0 } else { 1 })
            .divider("")
            .block(block);
        frame.render_widget(tabs, layout[1]);
    }

    async fn keypress(&mut self, key: KeyEvent, view_tx: &CommandSender) -> Result<()> {
        match key.code {
            KeyCode::Left | KeyCode::Char('a') => {
                self.yes = true;
            },
            KeyCode::Right | KeyCode::Char('d') => {
                self.yes = false;
            },
            KeyCode::Enter => {
                if self.yes {
                    self.sender.send(()).await?;
                } else {
                    view_tx.send(Command::BackView).await?;
                }
            },
            KeyCode::Esc => {
                view_tx.send(Command::BackView).await?;
            },
            _ => {},
        }

        Ok(())
    }
}
