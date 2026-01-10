mod app;
mod event;
mod ui_render;

pub use app::App;
pub use event::{Event, EventHandler};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Runs the interactive TUI application
pub fn run_tui() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let event_handler = EventHandler::new(250);

    // Main loop
    let res = run_app(&mut terminal, &mut app, event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

/// Main application loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut event_handler: EventHandler,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui_render::render(f, app))?;

        if let Event::Key(key) = event_handler.next()? {
            app.handle_key(key)?;
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
