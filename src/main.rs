use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lyricbot::{
    tui::{
        views::{
            confirmation::{ConfirmData, Confirmation},
            counter::Counter,
            ViewContainer,
        },
        Command,
    },
    Result, HISTORY_SIZE, POLL_TIMEOUT,
};
use ratatui::prelude::*;
use std::{io::stdout, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // prepare terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // clear the terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main logic
    let mut history = vec![];
    let mut view: ViewContainer = Box::new(Counter::default());
    let mut confirm_data = None::<ConfirmData>;
    let (confirm_tx, mut confirm_rx) = mpsc::channel(1);
    let (command_tx, mut view_rx) = mpsc::channel(1);

    loop {
        // check if there is a new confirmation update
        if let Ok(()) = confirm_rx.try_recv() {
            if let Some(ConfirmData { ref mut action, .. }) = confirm_data {
                let action = std::mem::replace(action, Box::new(|| None));
                if let Some(command) = action() {
                    command_tx.send(command).await?;
                }
            }
        }

        // check if the view has changed
        if let Ok(command) = view_rx.try_recv() {
            view = match command {
                Command::ChangeView { view: new_view } => {
                    if history.len() == HISTORY_SIZE {
                        history.remove(0);
                    }
                    history.push(view);

                    new_view
                },
                Command::BackView => {
                    if let Some(ConfirmData { previous, .. }) = confirm_data {
                        confirm_data = None;
                        previous
                    } else if history.len() > 0 {
                        let x = history.pop();
                        x.unwrap()
                    } else {
                        view
                    }
                },
                Command::Confirm(message, ConfirmData { previous, action }) => {
                    confirm_data = Some(ConfirmData { action, previous });
                    Box::new(Confirmation::new(message, confirm_tx.clone()))
                },
                Command::Quit => {
                    if confirm_data.is_some() {
                        break;
                    } else {
                        confirm_data = Some(ConfirmData {
                            previous: view,
                            action: Box::new(|| Some(Command::Quit)),
                        });
                        Box::new(Confirmation::new(
                            "Are you sure you want to quit?".to_string(),
                            confirm_tx.clone(),
                        ))
                    }
                },
            };
        }

        // draw the view
        terminal.draw(|frame| {
            let area = frame.size();
            view.draw(frame, area);
        })?;

        // handle events
        if event::poll(Duration::from_millis(POLL_TIMEOUT))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    view.keypress(key, &command_tx).await?;

                    // q and ctrl+c quit
                    if key.code == KeyCode::Char('q')
                        || (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL))
                            && confirm_data.is_none()
                    {
                        let previous = dyn_clone::clone_box(&*view);
                        command_tx
                            .send(Command::Confirm(
                                "Are you sure you want to quit?".to_string(),
                                ConfirmData {
                                    previous,
                                    action: Box::new(|| Some(Command::Quit)),
                                },
                            ))
                            .await?;
                    }
                },
                _ => {},
            }
        }
    }

    // cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
