use keyring::{Entry, Error as KeyringError};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use rand::{thread_rng, Rng, seq::SliceRandom};
use rand::distributions::Alphanumeric;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use std::sync::Arc;

static APP_INDEX: &str = "apppass_index";
static APP_SERVICE: &str = "apppass";
static APPLICATION_DATA: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn save_to_keyring(app_name: &str, password: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.set_password(password)?;
    update_index(app_name, true)?;
    Ok(())
}

fn get_from_keyring(app_name: &str) -> Result<String, KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.get_password()
}


fn delete_from_keyring(app_name: &str) -> Result<(), KeyringError> {
    let entry = Entry::new(APP_SERVICE, app_name)?;
    entry.delete_credential()?;
    update_index(app_name, false)?;
    Ok(())
}

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

/// Lista todas las aplicaciones almacenadas
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

pub fn generate_otp(app_name: &str, ttl_seconds: u64) {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        + Duration::new(ttl_seconds, 0);

    let otp: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10) // Longitud predeterminada para OTP
        .map(char::from)
        .collect();

    println!("Temporary Password: {}", otp);
    println!("Expires at: {:?}", expiration);
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