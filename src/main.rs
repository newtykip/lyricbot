use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use lyricbot::{
    tui::views::{counter::Counter, ViewChange, ViewContainer},
    Result, HISTORY_LIMIT, POLL_TIMEOUT,
};
use ratatui::prelude::*;
use std::{io::stdout, time::Duration};

fn main() -> Result<()> {
    // prepare terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // clear the terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main logic
    let mut history = vec![];
    let mut view: ViewContainer = Box::new(Counter::default());
    let (view_tx, view_rx) = crossbeam_channel::bounded(1);

    loop {
        // check if the view has changed
        if let Ok(command) = view_rx.try_recv() {
            view = match command {
                ViewChange::New(new_view) => {
                    if history.len() == HISTORY_LIMIT {
                        history.remove(0);
                    }
                    history.push(view);
                    new_view
                }
                ViewChange::Back if history.len() > 0 => history.pop().unwrap(),
                _ => view,
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
                    view.keypress(key, &view_tx)?;

                    // global keybinds
                    match key.code {
                        // q and ctrl+c quit
                        KeyCode::Char('q') => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break
                        }
                        _ => {}
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
