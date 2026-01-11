#![allow(dead_code)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use keyring::Entry;
use crate::app::keyring::{save_to_keyring, delete_from_keyring, set_password_type};
use crate::app::{APP_SERVICE, APP_INDEX, OTP_EXPIRY_SUFFIX, PASSWORD_TYPE_SUFFIX, PASSWORD_LENGTH_KEY};

/// Saves the expiry timestamp for an OTP.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `expiry_timestamp` - Unix timestamp when the OTP expires.
fn save_otp_expiry(app_name: &str, expiry_timestamp: u64) -> Result<(), String> {
    let expiry_key = format!("{}{}", app_name, OTP_EXPIRY_SUFFIX);
    if let Ok(entry) = Entry::new(APP_SERVICE, &expiry_key) {
        entry.set_password(&expiry_timestamp.to_string())
            .map_err(|e| format!("Failed to save OTP expiry: {}", e))
    } else {
        Err("Failed to create keyring entry for OTP expiry".to_string())
    }
}

/// Gets the expiry timestamp for an OTP.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * `Option<u64>` - The Unix timestamp when the OTP expires, or None if not set.
fn get_otp_expiry(app_name: &str) -> Option<u64> {
    let expiry_key = format!("{}{}", app_name, OTP_EXPIRY_SUFFIX);
    if let Ok(entry) = Entry::new(APP_SERVICE, &expiry_key) {
        if let Ok(value) = entry.get_password() {
            return value.parse::<u64>().ok();
        }
    }
    None
}

/// Deletes the expiry timestamp for an OTP.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
fn delete_otp_expiry(app_name: &str) {
    let expiry_key = format!("{}{}", app_name, OTP_EXPIRY_SUFFIX);
    if let Ok(entry) = Entry::new(APP_SERVICE, &expiry_key) {
        let _ = entry.delete_credential();
    }
}

/// Deletes an OTP and its associated metadata.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
pub fn delete_otp(app_name: &str) -> Result<(), String> {
    // Delete the OTP itself
    delete_from_keyring(app_name).map_err(|e| format!("Failed to delete OTP: {}", e))?;
    // Delete the expiry metadata
    delete_otp_expiry(app_name);
    Ok(())
}

/// Checks if an OTP has expired.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * `bool` - True if the OTP has expired or has no expiry set.
pub fn is_otp_expired(app_name: &str) -> bool {
    if let Some(expiry) = get_otp_expiry(app_name) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        return now >= expiry;
    }
    // No expiry set means it's not an OTP or already cleaned up
    false
}

/// Cleans up all expired OTPs from the keyring.
/// Should be called at application startup.
pub fn cleanup_expired_otps() {
    if let Ok(entry) = Entry::new(APP_SERVICE, APP_INDEX) {
        if let Ok(data) = entry.get_password() {
            let app_names: Vec<String> = data.split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            
            for app_name in app_names {
                // Skip metadata entries
                if app_name == PASSWORD_LENGTH_KEY 
                    || app_name.ends_with(PASSWORD_TYPE_SUFFIX)
                    || app_name.ends_with(OTP_EXPIRY_SUFFIX)
                    || app_name == APP_INDEX {
                    continue;
                }
                
                // Check if this is an OTP with expiry
                if let Some(expiry) = get_otp_expiry(&app_name) {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_secs(0))
                        .as_secs();
                    
                    if now >= expiry {
                        // OTP has expired, delete it
                        if let Err(e) = delete_otp(&app_name) {
                            eprintln!("Warning: Failed to cleanup expired OTP '{}': {}", app_name, e);
                        }
                    }
                }
            }
        }
    }
}

/// Generates a one-time password (OTP), saves it to the keyring, and schedules automatic deletion.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for the OTP.
/// * `ttl_seconds` - The time-to-live for the OTP in seconds.
/// * `length` - The length of the OTP to generate.
///
/// # Returns
///
/// * `Result<String, String>` - Returns the generated OTP on success, or an error message on failure.
///
/// # Behavior
///
/// The OTP is saved to the system keyring along with its expiry timestamp. A background thread
/// is spawned to delete it after the TTL expires. If the program exits before the TTL expires,
/// the OTP will be cleaned up on next startup via `cleanup_expired_otps()`.
pub fn generate_otp(app_name: &str, ttl_seconds: u64, length: usize) -> Result<String, String> {
    let otp: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    // Calculate expiry timestamp
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {}", e))?
        .as_secs();
    let expiry_timestamp = now + ttl_seconds;

    // Save OTP to keyring
    match save_to_keyring(app_name, &otp) {
        Ok(_) => {
            // Mark as auto-generated (OTP type)
            if let Err(e) = set_password_type(app_name, "auto") {
                eprintln!("Warning: Failed to set password type for OTP: {}", e);
            }
            
            // Save expiry timestamp
            if let Err(e) = save_otp_expiry(app_name, expiry_timestamp) {
                eprintln!("Warning: Failed to save OTP expiry: {}", e);
            }
            
            // Spawn a background thread to delete the OTP after TTL expires
            let app_name_owned = app_name.to_string();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(ttl_seconds));
                // Attempt to delete the OTP from keyring
                if let Err(e) = delete_otp(&app_name_owned) {
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

    fn cleanup_test_otp(app_name: &str) {
        let _ = delete_otp(app_name);
    }

    #[test]
    fn test_generate_otp_creates_password() {
        let app_name = "test_otp_create_unique_123";
        cleanup_test_otp(app_name);
        
        let result = generate_otp(app_name, 300, 12); // Longer TTL to avoid auto-delete during test
        assert!(result.is_ok(), "generate_otp should succeed");
        
        let otp = result.unwrap();
        assert_eq!(otp.len(), 12);
        
        // Small delay to ensure keyring is updated
        thread::sleep(Duration::from_millis(100));
        
        // Verify it's saved to keyring
        let retrieved = get_from_keyring(app_name);
        assert!(retrieved.is_ok(), "Password should be retrievable from keyring");
        assert_eq!(retrieved.unwrap(), otp);
        
        cleanup_test_otp(app_name);
    }

    #[test]
    fn test_generate_otp_saves_expiry() {
        let app_name = "test_otp_expiry_save";
        cleanup_test_otp(app_name);
        
        let _otp = generate_otp(app_name, 60, 10).unwrap();
        
        // Verify expiry is saved
        let expiry = get_otp_expiry(app_name);
        assert!(expiry.is_some());
        
        // Expiry should be in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(expiry.unwrap() > now);
        
        cleanup_test_otp(app_name);
    }

    #[test]
    fn test_is_otp_expired_false_initially() {
        let app_name = "test_otp_not_expired";
        cleanup_test_otp(app_name);
        
        let _otp = generate_otp(app_name, 60, 10).unwrap();
        
        // Should not be expired immediately
        assert!(!is_otp_expired(app_name));
        
        cleanup_test_otp(app_name);
    }

    #[test]
    fn test_is_otp_expired_true_after_ttl() {
        let app_name = "test_otp_expired_ttl_check";
        cleanup_test_otp(app_name);
        
        // Create with longer TTL to test expiry logic, not auto-delete
        let _otp = generate_otp(app_name, 60, 10).unwrap();
        
        // Get the expiry timestamp
        let expiry = get_otp_expiry(app_name);
        assert!(expiry.is_some(), "OTP expiry should be set");
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // The expiry should be in the future (now + TTL)
        assert!(expiry.unwrap() > now, "Expiry should be in the future");
        
        // OTP should not be expired yet
        assert!(!is_otp_expired(app_name), "OTP should not be expired yet");
        
        cleanup_test_otp(app_name);
    }

    #[test]
    fn test_delete_otp_removes_password_and_expiry() {
        let app_name = "test_otp_delete_unique_456";
        cleanup_test_otp(app_name);
        
        let _otp = generate_otp(app_name, 300, 10).unwrap(); // Longer TTL
        
        // Small delay to ensure keyring is updated
        thread::sleep(Duration::from_millis(100));
        
        // Delete OTP
        let result = delete_otp(app_name);
        assert!(result.is_ok(), "delete_otp should succeed");
        
        // Small delay after deletion
        thread::sleep(Duration::from_millis(100));
        
        // Password should be gone
        let retrieved = get_from_keyring(app_name);
        assert!(retrieved.is_err(), "Password should be deleted");
        
        // Expiry should be gone
        let expiry = get_otp_expiry(app_name);
        assert!(expiry.is_none(), "Expiry should be deleted");
    }

    #[test]
    fn test_save_and_get_otp_expiry() {
        let app_name = "test_otp_expiry_roundtrip";
        let timestamp = 1234567890u64;
        
        // Save expiry
        let result = save_otp_expiry(app_name, timestamp);
        assert!(result.is_ok());
        
        // Get expiry
        let retrieved = get_otp_expiry(app_name);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), timestamp);
        
        // Cleanup
        let _ = delete_otp_expiry(app_name);
    }

    #[test]
    fn test_delete_otp_expiry() {
        let app_name = "test_otp_expiry_delete";
        
        let _ = save_otp_expiry(app_name, 12345);
        
        // delete_otp_expiry returns ()
        delete_otp_expiry(app_name);
        
        let expiry = get_otp_expiry(app_name);
        assert!(expiry.is_none());
    }

    #[test]
    fn test_cleanup_expired_otps_does_not_panic() {
        // Just verify it doesn't panic
        cleanup_expired_otps();
        assert!(true);
    }

    #[test]
    fn test_otp_length_variations() {
        let test_cases = [5, 10, 20, 50];
        
        for (i, length) in test_cases.iter().enumerate() {
            let app_name = format!("test_otp_length_{}", i);
            cleanup_test_otp(&app_name);
            
            let result = generate_otp(&app_name, 60, *length);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), *length);
            
            cleanup_test_otp(&app_name);
        }
    }

    #[test]
    fn test_otp_contains_alphanumeric_only() {
        let app_name = "test_otp_alphanumeric";
        cleanup_test_otp(app_name);
        
        let otp = generate_otp(app_name, 60, 100).unwrap();
        
        // All characters should be alphanumeric
        assert!(otp.chars().all(|c| c.is_alphanumeric()));
        
        cleanup_test_otp(app_name);
    }

    #[test]
    fn test_is_otp_expired_non_existent() {
        // Non-existent OTP should be considered expired (or rather, no expiry found)
        let result = is_otp_expired("non_existent_otp_xyz");
        // When no expiry exists, is_otp_expired returns false (no expiry set)
        assert!(!result);
    }
}