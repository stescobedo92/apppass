#[warn(unused_variables)]

use std::time::Duration;
use std::thread;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use crate::app::keyring::{save_to_keyring, delete_from_keyring};

/// Generates a one-time password (OTP), saves it to the keyring, and schedules automatic deletion.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for the OTP.
/// * `ttl_seconds` - The time-to-live for the OTP in seconds.
///
/// # Returns
///
/// * `Result<String, String>` - Returns the generated OTP on success, or an error message on failure.
pub fn generate_otp(app_name: &str, ttl_seconds: u64) -> Result<String, String> {
    let otp: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    // Save OTP to keyring
    match save_to_keyring(app_name, &otp) {
        Ok(_) => {
            // Spawn a background thread to delete the OTP after TTL expires
            let app_name_owned = app_name.to_string();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(ttl_seconds));
                // Attempt to delete the OTP from keyring
                // Note: The thread will continue even if the main program exits.
                // This is acceptable as the OS will clean up the thread when it completes.
                if let Err(e) = delete_from_keyring(&app_name_owned) {
                    // Log error if deletion fails (e.g., already manually deleted)
                    eprintln!("Warning: Failed to auto-delete OTP for '{}': {}", app_name_owned, e);
                }
            });

            Ok(otp)
        }
        Err(e) => Err(format!("Failed to save OTP to keyring: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::keyring::get_from_keyring;

    #[test]
    fn test_generate_otp() {
        let ttl_seconds = 2;
        let app_name = "TestOTPApp";
        
        // Generate OTP
        let result = generate_otp(app_name, ttl_seconds);
        assert!(result.is_ok());
        
        let otp = result.unwrap();
        assert_eq!(otp.len(), 10);
        
        // Verify it's saved to keyring
        let retrieved = get_from_keyring(app_name);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), otp);
        
        // Wait for TTL to expire
        thread::sleep(Duration::from_secs(ttl_seconds + 1));
        
        // Verify it's deleted from keyring
        let result_after_expiry = get_from_keyring(app_name);
        assert!(result_after_expiry.is_err());
    }
}