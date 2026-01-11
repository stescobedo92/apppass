#[warn(unused_imports)]

use keyring::{Entry, Error as KeyringError};
use std::collections::{HashSet};
use crate::app::{APP_INDEX, APP_SERVICE};

/// Saves the given password to the keyring for the specified application.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `password` - The password to save.
///
/// # Returns
///
/// * `Result<(), KeyringError>` - Returns `Ok(())` if the password is saved successfully, otherwise returns a `KeyringError`.
pub fn save_to_keyring(app_name: &str, password: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.set_password(password)?;
    update_index(app_name, true)?;
    Ok(())
}

/// Retrieves the password from the keyring for the specified application.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * `Result<String, KeyringError>` - Returns the password as a `String` if found, otherwise returns a `KeyringError`.
pub fn get_from_keyring(app_name: &str) -> Result<String, KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.get_password()
}

/// Deletes the password from the keyring for the specified application.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * `Result<(), KeyringError>` - Returns `Ok(())` if the password is deleted successfully, otherwise returns a `KeyringError`.
pub fn delete_from_keyring(app_name: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.delete_credential()?;
    update_index(app_name, false)?;
    
    // Also delete type metadata if it exists
    let type_key = format!("{}{}", app_name, crate::app::PASSWORD_TYPE_SUFFIX);
    let type_entry = Entry::new(APP_SERVICE, &type_key);
    if let Ok(entry) = type_entry {
        let _ = entry.delete_credential(); // Ignore error if metadata doesn't exist
    }
    Ok(())
}

/// Updates the index of applications in the keyring.
///
/// This function retrieves the current index of applications from the keyring,
/// modifies it by adding or removing the specified application name, and then
/// saves the updated index back to the keyring.
///
/// # Arguments
///
/// * `app_name` - The name of the application to add or remove from the index.
/// * `add` - A boolean indicating whether to add (`true`) or remove (`false`) the application name.
///
/// # Returns
///
/// * `Result<(), KeyringError>` - Returns `Ok(())` if the index is updated successfully, otherwise returns a `KeyringError`.
fn update_index(app_name: &str, add: bool) -> Result<(), KeyringError> {
    // Skip if app_name is a metadata key (shouldn't be in index anyway)
    if app_name == crate::app::PASSWORD_LENGTH_KEY 
        || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
        || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
        || app_name == APP_INDEX {
        return Ok(());
    }

    let entry = Entry::new(APP_SERVICE, APP_INDEX)?;
    
    // Read current index with retry logic for race conditions
    let mut index: HashSet<String> = match entry.get_password() {
        Ok(data) => {
            // Parse and filter in one pass for efficiency
            data.split(',')
                .filter_map(|s| {
                    let trimmed = s.trim();
                    if trimmed.is_empty() 
                        || trimmed == crate::app::PASSWORD_LENGTH_KEY 
                        || trimmed.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                        || trimmed.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                        || trimmed == APP_INDEX {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                })
                .collect()
        }
        Err(KeyringError::NoEntry) => HashSet::new(),
        Err(e) => return Err(e),
    };

    // Update index
    if add {
        index.insert(app_name.to_string());
    } else {
        index.remove(app_name);
    }

    // If no real entries remain, delete the index entirely
    if index.is_empty() {
        // Try to delete the index entry (ignore error if already deleted)
        let _ = entry.delete_credential();
        return Ok(());
    }

    // Convert to sorted vector for consistent output (helps with testing and debugging)
    let mut real_entries: Vec<String> = index.into_iter().collect();
    real_entries.sort(); // Sort for deterministic output
    
    let updated_index = real_entries.join(",");
    entry.set_password(&updated_index)
}

/// Lists all applications stored in the keyring along with their passwords.
///
/// This function retrieves the index of applications from the keyring and
/// iterates through each application name to fetch and print the associated
/// password. It automatically cleans up orphaned entries (entries in the index
/// that no longer have passwords in the keyring).
///
/// # Panics
///
/// This function will panic if it fails to access the keyring or retrieve
/// the index of applications.
#[allow(dead_code)]
pub fn show_list_applications() {
    let entry = Entry::new(APP_SERVICE, APP_INDEX);
    match entry {
        Ok(index_entry) => match index_entry.get_password() {
            Ok(data) => {
                let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
                let mut has_any_passwords = false;
                
                for app_name in app_names {
                    // Skip metadata entries and internal index
                    if app_name == crate::app::PASSWORD_LENGTH_KEY 
                        || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                        || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                        || app_name == APP_INDEX {
                        continue;
                    }
                    
                    // Verify password exists before displaying
                    match get_from_keyring(app_name) {
                        Ok(password) => {
                            println!("Application Name: {}", app_name);
                            println!("Password: {}", password);
                            println!();
                            has_any_passwords = true;
                        }
                        Err(_) => {
                            // Orphaned entry - remove from index silently
                            // Don't show error to user, just clean it up
                            let _ = update_index(app_name, false);
                        }
                    }
                }
                
                if !has_any_passwords {
                    println!("No applications stored.");
                }
            }
            Err(KeyringError::NoEntry) => {
                println!("No applications stored.");
            }
            Err(e) => eprintln!("Failed to retrieve index: {}", e),
        },
        Err(e) => eprintln!("Failed to access index: {}", e),
    }
}

/// Sets the type of password (auto-generated or custom) for tracking purposes.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
/// * `password_type` - The type of password: "auto" or "custom".
///
/// # Returns
///
/// * `Result<(), KeyringError>` - Returns `Ok(())` if successful.
pub fn set_password_type(app_name: &str, password_type: &str) -> Result<(), KeyringError> {
    let type_key = format!("{}{}", app_name, crate::app::PASSWORD_TYPE_SUFFIX);
    let entry = Entry::new(APP_SERVICE, &type_key)?;
    entry.set_password(password_type)?;
    Ok(())
}

/// Gets the type of password for a given application.
///
/// # Arguments
///
/// * `app_name` - The name of the application.
///
/// # Returns
///
/// * `Option<String>` - Returns Some("auto") or Some("custom"), or None if not set.
#[allow(dead_code)]
pub fn get_password_type(app_name: &str) -> Option<String> {
    let type_key = format!("{}{}", app_name, crate::app::PASSWORD_TYPE_SUFFIX);
    if let Ok(entry) = Entry::new(APP_SERVICE, &type_key) {
        entry.get_password().ok()
    } else {
        None
    }
}

/// Checks if there are any passwords stored in the keyring (either auto or custom).
///
/// # Returns
///
/// * `bool` - Returns true if there are any passwords stored.
#[allow(dead_code)]
pub fn has_any_passwords() -> bool {
    if let Ok(entry) = Entry::new(APP_SERVICE, APP_INDEX) {
        if let Ok(data) = entry.get_password() {
            let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
            
            for app_name in app_names {
                // Skip metadata entries and internal index
                if app_name == crate::app::PASSWORD_LENGTH_KEY 
                    || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                    || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                    || app_name == APP_INDEX {
                    continue;
                }
                
                // Found at least one real password
                return true;
            }
        }
    }
    false
}

/// Cleans up orphaned apppass_index if it exists but has no real passwords.
/// Should be called at application startup.
///
/// # Returns
///
/// * `()` - This function does not return a value.
pub fn cleanup_orphaned_index() {
    if let Ok(entry) = Entry::new(APP_SERVICE, APP_INDEX) {
        if let Ok(data) = entry.get_password() {
            let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
            let mut has_real_passwords = false;
            
            for app_name in &app_names {
                // Skip metadata entries and internal index
                if *app_name == crate::app::PASSWORD_LENGTH_KEY 
                    || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                    || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                    || *app_name == APP_INDEX {
                    continue;
                }
                
                // Verify the password actually exists in keyring
                if get_from_keyring(app_name).is_ok() {
                    has_real_passwords = true;
                    break;
                }
            }
            
            // If no real passwords exist, delete the index
            if !has_real_passwords {
                let _ = entry.delete_credential();
            }
        }
    }
}

/// Checks if there are any auto-generated passwords in the keyring.
///
/// # Returns
///
/// * `bool` - Returns true if there are auto-generated passwords.
#[allow(dead_code)]
pub fn has_auto_passwords() -> bool {
    if let Ok(entry) = Entry::new(APP_SERVICE, APP_INDEX) {
        if let Ok(data) = entry.get_password() {
            let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
            let mut has_auto = false;
            let mut real_password_count = 0;
            
            for app_name in app_names {
                // Skip metadata entries and internal index
                if app_name == crate::app::PASSWORD_LENGTH_KEY 
                    || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                    || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                    || app_name == APP_INDEX {
                    continue;
                }
                
                // Count real passwords
                real_password_count += 1;
                
                // Check password type (default to "auto" for legacy passwords without type)
                let pw_type = get_password_type(app_name).unwrap_or_else(|| "auto".to_string());
                if pw_type == "auto" {
                    has_auto = true;
                }
            }
            
            // Only return true if there are real passwords AND at least one is auto-generated
            return real_password_count > 0 && has_auto;
        }
    }
    false
}

/// Checks if there are any custom passwords in the keyring.
///
/// # Returns
///
/// * `bool` - Returns true if there are custom passwords.
#[allow(dead_code)]
pub fn has_custom_passwords() -> bool {
    if let Ok(entry) = Entry::new(APP_SERVICE, APP_INDEX) {
        if let Ok(data) = entry.get_password() {
            let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
            let mut has_custom = false;
            let mut real_password_count = 0;
            
            for app_name in app_names {
                // Skip metadata entries and internal index
                if app_name == crate::app::PASSWORD_LENGTH_KEY 
                    || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
                    || app_name.ends_with(crate::app::OTP_EXPIRY_SUFFIX)
                    || app_name == APP_INDEX {
                    continue;
                }
                
                // Count real passwords
                real_password_count += 1;
                
                // Check password type (default to "auto" for legacy passwords without type)
                let pw_type = get_password_type(app_name).unwrap_or_else(|| "auto".to_string());
                if pw_type == "custom" {
                    has_custom = true;
                }
            }
            
            // Only return true if there are real passwords AND at least one is custom
            return real_password_count > 0 && has_custom;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use keyring::Entry;

    const TEST_APP_NAME: &str = "test_app_keyring";
    const TEST_PASSWORD: &str = "test_password_123";

    /// Check if keyring service is available (returns false on headless CI systems)
    fn is_keyring_available() -> bool {
        let test_entry = Entry::new("apppass_test_probe", "keyring_availability_check");
        match test_entry {
            Ok(entry) => {
                // Try to set and delete a test password
                match entry.set_password("test") {
                    Ok(_) => {
                        let _ = entry.delete_credential();
                        true
                    }
                    Err(_) => false
                }
            }
            Err(_) => false
        }
    }

    /// Skip test if keyring is not available
    macro_rules! skip_if_no_keyring {
        () => {
            if !is_keyring_available() {
                eprintln!("Skipping test: keyring service not available (CI environment)");
                return;
            }
        };
    }

    fn cleanup_test_entry(app_name: &str) {
        let _ = delete_from_keyring(app_name);
    }

    #[test]
    fn test_save_to_keyring() {
        skip_if_no_keyring!();
        cleanup_test_entry(TEST_APP_NAME);
        let result = save_to_keyring(TEST_APP_NAME, TEST_PASSWORD);
        assert!(result.is_ok());
        cleanup_test_entry(TEST_APP_NAME);
    }

    #[test]
    fn test_get_from_keyring() {
        skip_if_no_keyring!();
        let test_app = format!("{}_get_{}", TEST_APP_NAME, std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        cleanup_test_entry(&test_app);
        
        // Save password
        let save_result = save_to_keyring(&test_app, TEST_PASSWORD);
        assert!(save_result.is_ok(), "Failed to save password: {:?}", save_result);
        
        // Small delay to ensure keyring is updated (helps with race conditions)
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Get password
        let result = get_from_keyring(&test_app);
        assert!(result.is_ok(), "Failed to get password: {:?}", result);
        assert_eq!(result.unwrap(), TEST_PASSWORD);
        
        cleanup_test_entry(&test_app);
    }

    #[test]
    fn test_get_from_keyring_not_found() {
        let result = get_from_keyring("non_existent_app_xyz_123");
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_from_keyring() {
        skip_if_no_keyring!();
        let test_app = "test_delete_keyring_entry";
        cleanup_test_entry(test_app);
        save_to_keyring(test_app, TEST_PASSWORD).unwrap();
        let result = delete_from_keyring(test_app);
        assert!(result.is_ok());
        let result = get_from_keyring(test_app);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_index_add_and_remove() {
        skip_if_no_keyring!();
        // Use unique name with random component to avoid conflicts with other tests
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let test_app = format!("test_index_app_{}", nanos);
        let test_password = "test_password_for_index";
        
        // Ensure clean state - remove if exists
        cleanup_test_entry(&test_app);
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Add to index by saving a password (this internally calls update_index)
        // This is the real-world way to add entries, so it's more reliable
        let save_result = save_to_keyring(&test_app, test_password);
        assert!(save_result.is_ok(), "Failed to save password: {:?}", save_result);
        
        // Verify password was saved correctly
        let get_result = get_from_keyring(&test_app);
        assert!(get_result.is_ok(), "Failed to get password after save");
        assert_eq!(get_result.unwrap(), test_password);
        
        // Retry logic to handle race conditions with other parallel tests
        // Check multiple times with increasing delays
        let mut found = false;
        for attempt in 0..10 {
            std::thread::sleep(std::time::Duration::from_millis(100 + (attempt * 10)));
            let entry = Entry::new(APP_SERVICE, APP_INDEX).unwrap();
            if let Ok(index) = entry.get_password() {
                // Check if our app is in the index (may be among other apps from parallel tests)
                let entries: Vec<&str> = index.split(',').map(|s| s.trim()).collect();
                if entries.contains(&test_app.as_str()) {
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "App '{}' should be in index after saving password (verified password exists)", test_app);

        // Remove from index by deleting the password (this internally calls update_index)
        let delete_result = delete_from_keyring(&test_app);
        assert!(delete_result.is_ok(), "Failed to delete password: {:?}", delete_result);
        
        // Verify password was deleted
        let get_after_delete = get_from_keyring(&test_app);
        assert!(get_after_delete.is_err(), "Password should be deleted");
        
        // Retry logic to verify removal from index
        let mut removed = false;
        for attempt in 0..10 {
            std::thread::sleep(std::time::Duration::from_millis(100 + (attempt * 10)));
            let entry = Entry::new(APP_SERVICE, APP_INDEX).unwrap();
            match entry.get_password() {
                Ok(index) => {
                    let entries: Vec<&str> = index.split(',').map(|s| s.trim()).collect();
                    if !entries.contains(&test_app.as_str()) {
                        removed = true;
                        break;
                    }
                }
                Err(KeyringError::NoEntry) => {
                    // Index was deleted because it became empty - this is expected
                    removed = true;
                    break;
                }
                Err(_) => {
                    // Continue retrying
                }
            }
        }
        assert!(removed, "App '{}' should be removed from index after deleting password (verified password deleted)", test_app);
        
        // Final cleanup (should be no-op but ensure clean state)
        cleanup_test_entry(&test_app);
    }

    #[test]
    fn test_set_and_get_password_type() {
        skip_if_no_keyring!();
        let test_app = "test_type_app";
        cleanup_test_entry(test_app);
        save_to_keyring(test_app, "password").unwrap();
        
        // Set type to custom
        let result = set_password_type(test_app, "custom");
        assert!(result.is_ok());
        
        // Get type
        let pw_type = get_password_type(test_app);
        assert!(pw_type.is_some());
        assert_eq!(pw_type.unwrap(), "custom");
        
        cleanup_test_entry(test_app);
        // Cleanup type metadata
        let type_key = format!("{}{}", test_app, crate::app::PASSWORD_TYPE_SUFFIX);
        let _ = Entry::new(APP_SERVICE, &type_key).and_then(|e| e.delete_credential());
    }

    #[test]
    fn test_get_password_type_default() {
        // Non-existent app should return None
        let result = get_password_type("non_existent_type_app");
        assert!(result.is_none());
    }

    #[test]
    fn test_has_any_passwords_empty() {
        // This test assumes the keyring might have entries
        // The function should work without panicking
        let _result = has_any_passwords();
        // Just verify it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_has_any_passwords_with_entry() {
        skip_if_no_keyring!();
        let test_app = "test_has_passwords_app";
        cleanup_test_entry(test_app);
        
        save_to_keyring(test_app, "password").unwrap();
        
        let result = has_any_passwords();
        assert!(result);
        
        cleanup_test_entry(test_app);
    }

    #[test]
    fn test_cleanup_orphaned_index() {
        // Should not panic when called
        cleanup_orphaned_index();
        assert!(true);
    }

    #[test]
    fn test_show_list_applications() {
        skip_if_no_keyring!();
        let test_app = "test_list_app";
        cleanup_test_entry(test_app);
        
        save_to_keyring(test_app, "password").unwrap();
        
        // Should not panic
        show_list_applications();
        
        cleanup_test_entry(test_app);
    }

    #[test]
    fn test_has_auto_passwords() {
        skip_if_no_keyring!();
        let test_app = "test_auto_pw_app";
        cleanup_test_entry(test_app);
        
        save_to_keyring(test_app, "password").unwrap();
        let _ = set_password_type(test_app, "auto");
        
        let result = has_auto_passwords();
        assert!(result);
        
        cleanup_test_entry(test_app);
        let type_key = format!("{}{}", test_app, crate::app::PASSWORD_TYPE_SUFFIX);
        let _ = Entry::new(APP_SERVICE, &type_key).and_then(|e| e.delete_credential());
    }

    #[test]
    fn test_has_custom_passwords() {
        skip_if_no_keyring!();
        // Use unique name to avoid conflicts with other tests
        let test_app = format!("test_custom_pw_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());
        cleanup_test_entry(&test_app);
        
        // Save password
        let save_result = save_to_keyring(&test_app, "password");
        assert!(save_result.is_ok(), "Failed to save password: {:?}", save_result);
        
        // Set type to custom
        let type_result = set_password_type(&test_app, "custom");
        assert!(type_result.is_ok(), "Failed to set password type: {:?}", type_result);
        
        // Small delay to ensure keyring is updated
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Verify password type was set correctly
        let pw_type = get_password_type(&test_app);
        assert_eq!(pw_type, Some("custom".to_string()), "Password type should be 'custom'");
        
        // Check has_custom_passwords
        let result = has_custom_passwords();
        assert!(result, "has_custom_passwords should return true when custom password exists");
        
        cleanup_test_entry(&test_app);
        let type_key = format!("{}{}", test_app, crate::app::PASSWORD_TYPE_SUFFIX);
        let _ = Entry::new(APP_SERVICE, &type_key).and_then(|e| e.delete_credential());
    }

    #[test]
    fn test_save_overwrite_password() {
        skip_if_no_keyring!();
        let test_app = "test_overwrite_app";
        cleanup_test_entry(test_app);
        
        save_to_keyring(test_app, "password1").unwrap();
        save_to_keyring(test_app, "password2").unwrap();
        
        let result = get_from_keyring(test_app).unwrap();
        assert_eq!(result, "password2");
        
        cleanup_test_entry(test_app);
    }

    #[test]
    fn test_delete_also_removes_type_metadata() {
        skip_if_no_keyring!();
        let test_app = "test_delete_meta_app";
        cleanup_test_entry(test_app);
        
        save_to_keyring(test_app, "password").unwrap();
        let _ = set_password_type(test_app, "custom");
        
        // Delete should also remove metadata
        delete_from_keyring(test_app).unwrap();
        
        // Type should be None after deletion
        let pw_type = get_password_type(test_app);
        assert!(pw_type.is_none());
    }
}