use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lyricbot::{
    tui::{
        views::{confirmation::Confirmation, counter::Counter, ViewContainer},
        Command,
    },
    Result, HISTORY_SIZE, POLL_TIMEOUT,
};
use ratatui::prelude::*;
use std::{io::stdout, time::Duration};

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
    let mut confirm_prev = None::<ViewContainer>;
    let (command_tx, view_rx) = crossbeam_channel::bounded(1);

    loop {
        // check if the view has changed
        if let Ok(command) = view_rx.try_recv() {
            view = match command {
                Command::ChangeView { view: new_view } => {
                    if history.len() == HISTORY_SIZE {
                        history.remove(0);
                    }
                    history.push(view);

                    new_view
                }
                Command::BackView => {
                    if let Some(ref prev) = confirm_prev {
                        let x = dyn_clone::clone_box(&**prev);
                        confirm_prev = None;
                        x
                    } else if history.len() > 0 {
                        let x = history.pop();
                        x.unwrap()
                    } else {
                        view
                    }
                }
                Command::Stop => break,
                Command::Confirm { message, previous } => {
                    confirm_prev = Some(previous);
                    Box::new(Confirmation::new(message))
                }
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
                    view.keypress(key, &command_tx)?;

                    // q and ctrl+c quit
                    if key.code == KeyCode::Char('q')
                        || (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL))
                            && confirm_prev.is_none()
                    {
                        command_tx.send(Command::Confirm {
                            message: "Are you sure you want to quit?".to_string(),
                            previous: dyn_clone::clone_box(&*view),
                        })?;
                    }
                }
                _ => {}
            }
        }
    }

    // cleanup
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
