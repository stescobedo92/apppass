use crate::app::keyring::{delete_from_keyring, get_from_keyring, save_to_keyring, set_password_type};
use keyring::Error as KeyringError;
use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

/// Retrieves the password for the specified application from the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is retrieved.
#[allow(dead_code)]
pub fn get_password_for_specify_app(app_name: &str) -> Result<String, KeyringError> {
    get_from_keyring(app_name)
}

/// Updates the password for the specified application in the keyring with a custom password.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is updated.
/// * `new_password` - A string slice that holds the new password to be saved.
#[allow(dead_code)]
pub fn update_password(app_name: &str, new_password: &str) -> Result<(), KeyringError> {
    // Check if password exists before updating
    match get_from_keyring(app_name) {
        Ok(_) => {
            // Password exists, proceed with update
            save_to_keyring(app_name, new_password)?;
            set_password_type(app_name, "custom")?;
            Ok(())
        }
        Err(KeyringError::NoEntry) => {
            Err(KeyringError::NoEntry)
        }
        Err(e) => Err(e),
    }
}

/// Updates the password for the specified application by regenerating a new secure password.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is updated.
/// * `length` - An optional length for the generated password (defaults to 30).
///
/// # Returns
///
/// * `Result<String, KeyringError>` - Returns the new password on success.
#[allow(dead_code)]
pub fn update_password_regenerate(app_name: &str, length: Option<usize>) -> Result<String, KeyringError> {
    // Check if password exists before updating
    match get_from_keyring(app_name) {
        Ok(_) => {
            let length = length.unwrap_or(30);
            
            // Generate new secure password
            let new_password: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect();
            
            // Save the new password
            save_to_keyring(app_name, &new_password)?;
            set_password_type(app_name, "auto")?;
            Ok(new_password)
        }
        Err(KeyringError::NoEntry) => {
            Err(KeyringError::NoEntry)
        }
        Err(e) => Err(e),
    }
}

/// Generates a random password for the specified application and saves it to the keyring.
///
/// The password is composed of alphanumeric characters and has a default length of 30 characters if not specified.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is generated.
/// * `length` - An optional length for the generated password.
pub fn generate_save_safety_password(app_name: &str, length: Option<usize>) -> Result<(), KeyringError> {
    // Check if password already exists
    match get_from_keyring(app_name) {
        Ok(_) => {
            return Err(KeyringError::NoEntry);
        }
        Err(KeyringError::NoEntry) => {
            // Continue - this is what we want
        }
        Err(e) => return Err(e),
    }

    let length = length.unwrap_or(30);

    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    save_to_keyring(app_name, &rand_password)?;
    set_password_type(app_name, "auto")?;
    Ok(())
}

/// Deletes the password for the specified application from the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is deleted.
#[allow(dead_code)]
pub fn delete_password(app_name: &str) -> Result<(), KeyringError> {
    delete_from_keyring(app_name)
}

/// Exports all stored passwords to a specified file.
///
/// The passwords are retrieved from the keyring and written to the file in the format `app_name,password`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file where passwords will be exported.
pub fn export_passwords(file_path: &str) -> Result<(), KeyringError> {
    use crate::app::{APP_SERVICE, APP_INDEX};
    use keyring::Entry;
    
    // Get the index of all applications from keyring
    let entry = Entry::new(APP_SERVICE, APP_INDEX)?;
    let app_names_str = match entry.get_password() {
        Ok(data) => data,
        Err(KeyringError::NoEntry) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    
    let app_names: Vec<&str> = app_names_str.split(',').filter(|s| !s.is_empty()).collect();
    let mut content = String::new();

    for app_name in app_names {
        // Skip metadata entries (password_length, _type suffixes, and internal index)
        if app_name == crate::app::PASSWORD_LENGTH_KEY 
            || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX)
            || app_name == crate::app::APP_INDEX {
            continue;
        }
        if let Ok(password) = get_from_keyring(app_name) {
            content.push_str(&format!("{},{}\n", app_name, password));
        }
    }

    if content.is_empty() {
        Ok(())
    } else if std::fs::write(file_path, content).is_ok() {
        Ok(())
    } else {
        Err(KeyringError::NoEntry)
    }
}

/// Imports passwords from a specified file and saves them to the keyring.
///
/// The file should contain lines in the format `app_name,password`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file from which passwords are imported.
pub fn import_passwords(file_path: &str) -> Result<(), KeyringError> {
    if let Ok(content) = std::fs::read_to_string(file_path) {
        let lines = content.lines();
        for line in lines {
            let key_value: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if key_value.len() == 2 {
                let app_name = key_value[0];
                let password = key_value[1];
                save_to_keyring(app_name, password)?;
                set_password_type(app_name, "custom")?; // Mark imported passwords as custom
            }
        }
        Ok(())
    } else {
        Err(KeyringError::NoEntry)
    }
}

/// Generates a memorizable password for the specified application and saves it to the keyring.
///
/// The password is composed of two random words from a predefined list and a random number between 10 and 99.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is generated.
pub fn generate_memorizable_password(app_name: &str) -> Result<(), KeyringError> {
    // Check if password already exists
    match get_from_keyring(app_name) {
        Ok(_) => {
            return Err(KeyringError::NoEntry);
        }
        Err(KeyringError::NoEntry) => {
            // Continue - this is what we want
        }
        Err(e) => return Err(e),
    }

    let words = vec!["Tiger", "Orange", "Mountain", "River", "Cloud", "Sky", "Sun", "Moon"];
    let mut rng = thread_rng();

    let password = format!(
        "{}-{}-{}",
        words.choose(&mut rng).unwrap(),
        thread_rng().gen_range(10..99),
        words.choose(&mut rng).unwrap()
    );

    save_to_keyring(app_name, &password)?;
    set_password_type(app_name, "auto")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup_test_password(app_name: &str) {
        let _ = delete_password(app_name);
    }

    #[test]
    fn test_get_password_for_specify_app() {
        let app_name = "test_get_pw_app";
        cleanup_test_password(app_name);
        
        save_to_keyring(app_name, "test_password").unwrap();
        
        let result = get_password_for_specify_app(app_name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_password");
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_get_password_not_found() {
        let result = get_password_for_specify_app("non_existent_app_xyz_456");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_password() {
        let app_name = "test_update_pw_app";
        cleanup_test_password(app_name);
        
        save_to_keyring(app_name, "old_password").unwrap();
        
        let result = update_password(app_name, "new_password");
        assert!(result.is_ok());
        
        let retrieved = get_from_keyring(app_name).unwrap();
        assert_eq!(retrieved, "new_password");
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_update_password_not_found() {
        let result = update_password("non_existent_update_app", "password");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_save_safety_password_default_length() {
        let app_name = "test_gen_pw_default";
        cleanup_test_password(app_name);
        
        let result = generate_save_safety_password(app_name, None);
        assert!(result.is_ok());
        
        let password = get_from_keyring(app_name).unwrap();
        assert_eq!(password.len(), 30); // Default length
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_generate_save_safety_password_custom_length() {
        let app_name = "test_gen_pw_custom";
        cleanup_test_password(app_name);
        
        let result = generate_save_safety_password(app_name, Some(15));
        assert!(result.is_ok());
        
        let password = get_from_keyring(app_name).unwrap();
        assert_eq!(password.len(), 15);
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_generate_save_safety_password_already_exists() {
        let app_name = "test_gen_pw_exists";
        cleanup_test_password(app_name);
        
        // First save should succeed
        generate_save_safety_password(app_name, None).unwrap();
        
        // Second save should fail (already exists)
        let result = generate_save_safety_password(app_name, None);
        assert!(result.is_err());
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_delete_password() {
        let app_name = "test_delete_pw_app";
        cleanup_test_password(app_name);
        
        save_to_keyring(app_name, "password").unwrap();
        
        let result = delete_password(app_name);
        assert!(result.is_ok());
        
        let retrieved = get_from_keyring(app_name);
        assert!(retrieved.is_err());
    }

    #[test]
    fn test_delete_password_not_found() {
        let result = delete_password("non_existent_delete_app");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_password_regenerate() {
        let app_name = "test_regen_pw_app";
        cleanup_test_password(app_name);
        
        save_to_keyring(app_name, "old_password").unwrap();
        
        let result = update_password_regenerate(app_name, Some(20));
        assert!(result.is_ok());
        
        let new_password = result.unwrap();
        assert_eq!(new_password.len(), 20);
        assert_ne!(new_password, "old_password");
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_update_password_regenerate_not_found() {
        let result = update_password_regenerate("non_existent_regen_app", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_memorizable_password() {
        let app_name = "test_memo_pw_app";
        cleanup_test_password(app_name);
        
        let result = generate_memorizable_password(app_name);
        assert!(result.is_ok());
        
        let password = get_from_keyring(app_name).unwrap();
        
        // Should have 3 words separated by dashes
        let parts: Vec<&str> = password.split('-').collect();
        assert_eq!(parts.len(), 3);
        
        // Each part should not be empty
        for part in parts {
            assert!(!part.is_empty());
        }
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_generate_memorizable_password_already_exists() {
        let app_name = "test_memo_pw_exists";
        cleanup_test_password(app_name);
        
        generate_memorizable_password(app_name).unwrap();
        
        let result = generate_memorizable_password(app_name);
        assert!(result.is_err());
        
        cleanup_test_password(app_name);
    }

    // Note: update_app_name function doesn't exist in this module
    // If needed in the future, implement it in password.rs

    #[test]
    fn test_generated_password_is_alphanumeric() {
        let app_name = "test_alphanum_pw";
        cleanup_test_password(app_name);
        
        generate_save_safety_password(app_name, Some(50)).unwrap();
        
        let password = get_from_keyring(app_name).unwrap();
        assert!(password.chars().all(|c| c.is_alphanumeric()));
        
        cleanup_test_password(app_name);
    }

    #[test]
    fn test_export_import_passwords_roundtrip() {
        let app_name = "test_export_import_app";
        let test_file = "test_export_temp.csv";
        cleanup_test_password(app_name);
        
        // Create a password
        save_to_keyring(app_name, "export_test_pwd").unwrap();
        
        // Export
        let export_result = export_passwords(test_file);
        assert!(export_result.is_ok());
        
        // Delete original
        delete_password(app_name).unwrap();
        
        // Import
        let import_result = import_passwords(test_file);
        assert!(import_result.is_ok());
        
        // Verify imported
        let retrieved = get_from_keyring(app_name);
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), "export_test_pwd");
        
        // Cleanup
        cleanup_test_password(app_name);
        let _ = std::fs::remove_file(test_file);
    }
}
