#[warn(unused_imports)]

use keyring::{Entry, Error as KeyringError};
use std::collections::{HashSet};
use crate::app::{APP_INDEX, APP_SERVICE};

pub fn save_to_keyring(app_name: &str, password: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.set_password(password)?;
    update_index(app_name, true)?;
    Ok(())
}

pub fn get_from_keyring(app_name: &str) -> Result<String, KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.get_password()
}


pub fn delete_from_keyring(app_name: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.delete_credential()?;
    update_index(app_name, false)?;
    Ok(())
}

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