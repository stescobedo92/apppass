use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind};
use std::io;
use std::time::Duration;

/// Terminal events
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event (currently unused)
    Mouse,
    /// Resize event
    Resize,
}

/// Event handler for terminal events
pub struct EventHandler {
    /// Poll timeout in milliseconds
    poll_timeout: Duration,
}

impl EventHandler {
    /// Creates a new EventHandler with the specified poll timeout
    pub fn new(poll_timeout_ms: u64) -> Self {
        Self {
            poll_timeout: Duration::from_millis(poll_timeout_ms),
        }
    }

    /// Polls for the next event, blocking until one is available or timeout.
    /// Returns `None` if no event is available within the timeout period.
    pub fn next(&mut self) -> io::Result<Option<Event>> {
        if !event::poll(self.poll_timeout)? {
            return Ok(None);
        }

        match event::read()? {
            CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => {
                // Only process key press events, ignore release and repeat
                // This prevents duplicate characters when typing
                Ok(Some(Event::Key(key)))
            }
            CrosstermEvent::Mouse(_) => Ok(Some(Event::Mouse)),
            CrosstermEvent::Resize(_, _) => Ok(Some(Event::Resize)),
            _ => Ok(None), // Ignore other event types
        }
    }
}
