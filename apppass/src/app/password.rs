
use keyring::{Error as KeyringError};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
use crate::app::keyring::{get_from_keyring, save_to_keyring, delete_from_keyring};
use crate::app::APPLICATION_DATA;

pub fn get_password_for_specify_app(app_name: &str) {
    match get_from_keyring(app_name) {
        Ok(password) => {
            println!("Application Name: {}", app_name);
            println!("Password: {}", password);
        }
        Err(KeyringError::NoEntry) => {
            println!("No password found for '{}'.", app_name);
        }
        Err(e) => {
            eprintln!("Failed to retrieve password for '{}': {}", app_name, e);
        }
    }
}

pub fn update_password(app_name: &str, new_password: &str) {
    match save_to_keyring(app_name, new_password) {
        Ok(_) => println!("Password updated successfully for '{}'.", app_name),
        Err(e) => eprintln!("Failed to update password for '{}': {}", app_name, e),
    }
}

pub fn generate_save_safety_password(app_name: &str, length: Option<usize>) {
    let length = length.unwrap_or(30);

    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    match save_to_keyring(app_name, &rand_password) {
        Ok(_) => println!("Password saved securely for '{}'.", app_name),
        Err(e) => eprintln!("Failed to save password for '{}': {}", app_name, e),
    }
}

pub fn delete_password(app_name: &str) {
    match delete_from_keyring(app_name) {
        Ok(_) => println!("Password for '{}' deleted successfully.", app_name),
        Err(KeyringError::NoEntry) => println!("No password found for '{}'.", app_name),
        Err(e) => eprintln!("Failed to delete password for '{}': {}", app_name, e),
    }
}

pub fn export_passwords(file_path: &str) {
    let application_data = APPLICATION_DATA.lock().unwrap();
    let mut content = String::new();

    for app_name in application_data.iter() {
        if let Ok(password) = get_from_keyring(app_name) {
            content.push_str(&format!("{},{}\n", app_name, password));
        }
    }

    if std::fs::write(file_path, content).is_ok() {
        println!("Passwords exported to '{}'.", file_path);
    } else {
        eprintln!("Failed to export passwords to '{}'.", file_path);
    }
}

pub fn import_passwords(file_path: &str) {
    if let Ok(content) = std::fs::read_to_string(file_path) {
        let lines = content.lines();
        for line in lines {
            let key_value: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if key_value.len() == 2 {
                let app_name = key_value[0];
                let password = key_value[1];
                let _ = save_to_keyring(app_name, password);
            }
        }
        println!("Passwords imported from '{}'.", file_path);
    } else {
        eprintln!("Failed to import passwords from '{}'.", file_path);
    }
}

pub fn generate_memorizable_password(app_name: &str) {
    let words = vec!["Tiger", "Orange", "Mountain", "River", "Cloud", "Sky", "Sun", "Moon"];
    let mut rng = thread_rng();

    let password = format!(
        "{}-{}-{}",
        words.choose(&mut rng).unwrap(),
        thread_rng().gen_range(10..99),
        words.choose(&mut rng).unwrap()
    );

    match save_to_keyring(app_name, &password) {
        Ok(_) => println!("Memorizable Password saved for '{}'.", app_name),
        Err(e) => eprintln!("Failed to save memorizable password for '{}': {}", app_name, e),
    }
}
