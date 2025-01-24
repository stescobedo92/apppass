use crate::app::keyring::{delete_from_keyring, get_from_keyring, save_to_keyring};
use crate::app::APPLICATION_DATA;
use keyring::Error as KeyringError;
use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

/// Retrieves the password for the specified application from the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is retrieved.
pub fn get_password_for_specify_app(app_name: &str) -> Result<(), KeyringError> {
    match get_from_keyring(app_name) {
        Ok(password) => {
            println!("Application Name: {}", app_name);
            println!("Password: {}", password);
            Ok(())
        }
        Err(KeyringError::NoEntry) => {
            println!("No password found for '{}'.", app_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to retrieve password for '{}': {}", app_name, e);
            Err(e)
        }
    }
}

/// Updates the password for the specified application in the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is updated.
/// * `new_password` - A string slice that holds the new password to be saved.
pub fn update_password(app_name: &str, new_password: &str) -> Result<(), KeyringError> {
    match save_to_keyring(app_name, new_password) {
        Ok(_) => {
            println!("Password updated successfully for '{}'.", app_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to update password for '{}': {}", app_name, e);
            Err(e)
        }
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
    let length = length.unwrap_or(30);

    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    match save_to_keyring(app_name, &rand_password) {
        Ok(_) => {
            println!("Password saved securely for '{}'.", app_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save password for '{}': {}", app_name, e);
            Err(e)
        }
    }
}

/// Deletes the password for the specified application from the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is deleted.
pub fn delete_password(app_name: &str) -> Result<(), KeyringError> {
    match delete_from_keyring(app_name) {
        Ok(_) => {
            println!("Password for '{}' deleted successfully.", app_name);
            Ok(())
        }
        Err(KeyringError::NoEntry) => {
            println!("No password found for '{}'.", app_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to delete password for '{}': {}", app_name, e);
            Err(e)
        }
    }
}

/// Exports all stored passwords to a specified file.
///
/// The passwords are retrieved from the keyring and written to the file in the format `app_name,password`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file where passwords will be exported.
pub fn export_passwords(file_path: &str) -> Result<(), KeyringError> {
    let application_data = APPLICATION_DATA.lock().unwrap();
    let mut content = String::new();

    for app_name in application_data.iter() {
        if let Ok(password) = get_from_keyring(app_name) {
            content.push_str(&format!("{},{}\n", app_name, password));
        }
    }

    if std::fs::write(file_path, content).is_ok() {
        println!("Passwords exported to '{}'.", file_path);
        Ok(())
    } else {
        eprintln!("Failed to export passwords to '{}'.", file_path);
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
            }
        }
        println!("Passwords imported from '{}'.", file_path);
        Ok(())
    } else {
        eprintln!("Failed to import passwords from '{}'.", file_path);
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
    let words = vec!["Tiger", "Orange", "Mountain", "River", "Cloud", "Sky", "Sun", "Moon"];
    let mut rng = thread_rng();

    let password = format!(
        "{}-{}-{}",
        words.choose(&mut rng).unwrap(),
        thread_rng().gen_range(10..99),
        words.choose(&mut rng).unwrap()
    );

    match save_to_keyring(app_name, &password) {
        Ok(_) => {
            println!("Memorizable Password saved for '{}'.", app_name);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save memorizable password for '{}': {}", app_name, e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::{eq, function};
    use std::sync::Mutex;
    use mockall::mock;

    mock! {
        pub Keyring {
            fn get_password(&self, app_name: &str) -> Result<String, KeyringError>;
            fn set_password(&self, app_name: &str, password: &str) -> Result<(), KeyringError>;
            fn delete_password(&self, app_name: &str) -> Result<(), KeyringError>;
        }
    }

    #[test]
    fn test_get_password_for_specify_app() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_get_password()
            .with(eq("test_app"))
            .returning(|_| Ok("test_password".to_string()));

        let result = get_password_for_specify_app("test_app");
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_password() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_set_password()
            .with(eq("test_app"), eq("new_password"))
            .returning(|_, _| Ok(()));

        let result = update_password("test_app", "new_password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_save_safety_password() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_set_password()
            .with(eq("test_app"), function(|password: &str| password.len() == 30))
            .returning(|_, _| Ok(()));

        let result = generate_save_safety_password("test_app", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_password() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_delete_password()
            .with(eq("test_app"))
            .returning(|_| Ok(()));

        let result = delete_password("test_app");
        assert!(result.is_ok());
    }

    #[test]
    fn test_export_passwords() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_get_password()
            .with(eq("test_app"))
            .returning(|_| Ok("test_password".to_string()));

        let application_data = Mutex::new(vec!["test_app".to_string()]);
        let result = export_passwords("test_file.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_memorizable_password() {
        let mut mock_keyring = MockKeyring::new();
        mock_keyring.expect_set_password()
            .with(eq("test_app"), function(|password: &str| password.split('-').count() == 3))
            .returning(|_, _| Ok(()));

        let result = generate_memorizable_password("test_app");
        assert!(result.is_ok());
    }
}
