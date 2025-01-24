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
    let entry = Entry::new(APP_SERVICE, APP_INDEX)?;
    let mut index: HashSet<String> = match entry.get_password() {
        Ok(data) => data.split(',').map(String::from).collect(),
        Err(KeyringError::NoEntry) => HashSet::new(),
        Err(e) => return Err(e),
    };

    if add {
        index.insert(app_name.to_string());
    } else {
        index.remove(app_name);
    }

    let updated_index = index.into_iter().collect::<Vec<_>>().join(",");
    entry.set_password(&updated_index)
}

/// Lists all applications stored in the keyring along with their passwords.
///
/// This function retrieves the index of applications from the keyring and
/// iterates through each application name to fetch and print the associated
/// password.
///
/// # Panics
///
/// This function will panic if it fails to access the keyring or retrieve
/// the index of applications.
pub fn show_list_applications() {
    let entry = Entry::new(APP_SERVICE, APP_INDEX);
    match entry {
        Ok(index_entry) => match index_entry.get_password() {
            Ok(data) => {
                let app_names: Vec<&str> = data.split(',').filter(|s| !s.is_empty()).collect();
                for app_name in app_names {
                    match get_from_keyring(app_name) {
                        Ok(password) => {
                            println!("Application Name: {}", app_name);
                            println!("Password: {}", password);
                            println!();
                        }
                        Err(e) => eprintln!("Failed to retrieve password for '{}': {}", app_name, e),
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use keyring::Entry;

    const TEST_APP_NAME: &str = "test_app";
    const TEST_PASSWORD: &str = "test_password";

    #[test]
    fn test_save_to_keyring() {
        let result = save_to_keyring(TEST_APP_NAME, TEST_PASSWORD);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_from_keyring() {
        save_to_keyring(TEST_APP_NAME, TEST_PASSWORD).unwrap();
        let result = get_from_keyring(TEST_APP_NAME);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_PASSWORD);
        delete_from_keyring(TEST_APP_NAME).unwrap(); // Clean up
    }

    #[test]
    fn test_delete_from_keyring() {
        save_to_keyring(TEST_APP_NAME, TEST_PASSWORD).unwrap();
        let result = delete_from_keyring(TEST_APP_NAME);
        assert!(result.is_ok());
        let result = get_from_keyring(TEST_APP_NAME);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_index() {
        let result = update_index(TEST_APP_NAME, true);
        assert!(result.is_ok());
        let entry = Entry::new(APP_SERVICE, APP_INDEX).unwrap();
        let index = entry.get_password().unwrap();
        assert!(index.contains(TEST_APP_NAME));

        let result = update_index(TEST_APP_NAME, false);
        assert!(result.is_ok());
        let index = entry.get_password().unwrap();
        assert!(!index.contains(TEST_APP_NAME));
    }
}