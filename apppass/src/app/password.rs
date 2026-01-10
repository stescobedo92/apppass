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
pub fn get_password_for_specify_app(app_name: &str) -> Result<String, KeyringError> {
    get_from_keyring(app_name)
}

/// Updates the password for the specified application in the keyring.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application for which the password is updated.
/// * `new_password` - A string slice that holds the new password to be saved.
pub fn update_password(app_name: &str, new_password: &str) -> Result<(), KeyringError> {
    // Check if password exists before updating
    match get_from_keyring(app_name) {
        Ok(_) => {
            // Password exists, proceed with update
            save_to_keyring(app_name, new_password)
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
        // Skip metadata entries (password_length and _type suffixes)
        if app_name == crate::app::PASSWORD_LENGTH_KEY || app_name.ends_with(crate::app::PASSWORD_TYPE_SUFFIX) {
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
