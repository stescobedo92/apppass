use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
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

    /// Polls for the next event, blocking until one is available or timeout
    pub fn next(&mut self) -> io::Result<Event> {
        loop {
            if event::poll(self.poll_timeout)? {
                match event::read()? {
                    CrosstermEvent::Key(key) => return Ok(Event::Key(key)),
                    CrosstermEvent::Mouse(_) => return Ok(Event::Mouse),
                    CrosstermEvent::Resize(_, _) => return Ok(Event::Resize),
                    _ => {}
                }
            }
        }
    }
}
