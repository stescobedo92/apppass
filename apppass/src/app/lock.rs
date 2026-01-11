use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Starts an auto-lock mechanism that locks the application after a specified timeout.
///
/// # Arguments
///
/// * `timeout_seconds` - The number of seconds to wait before locking the application.
#[allow(dead_code)]
pub fn start_auto_lock(timeout_seconds: u64) {
    let is_active = Arc::new(Mutex::new(true));
    let is_active_clone = Arc::clone(&is_active);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(timeout_seconds));
        let mut active = is_active_clone.lock().unwrap();
        if *active {
            *active = false;
            println!("Application locked due to inactivity.");
        }
    });

    println!("Auto-lock set to {} seconds.", timeout_seconds);
}