//! # Module Documentation
//!
//! This module provides functionalities for password management, including storage, generation,
//! listing, retrieval, deletion, updating, exporting, importing, and generating one-time passwords (OTPs).
//!
//! ## Functions
//!
//! - **fill_data()**
//!   - Fills internal application data (`APPLICATION_DATA`) from a file specified by `FILE_NAME`.
//!   - Reads each line, splits by comma, and populates the in-memory hash map for quick lookups.
//!
//! - **generate_save_safety_password(app_name: &str, length: Option<usize>)**
//!   - Generates a random password (default length 30) using alphanumeric characters.
//!   - Saves the application name and generated password to the file and updates in-memory data accordingly.
//!
//! - **show_list_applications()**
//!   - Displays all stored application-password pairs by reading from and printing the in-memory hash map.
//!
//! - **get_password_for_specify_app(app_name: &str)**
//!   - Retrieves the password for a specified application from the in-memory data.
//!   - Prints the password if found, otherwise informs that the application doesn't exist.
//!
//! - **delete_password(app_name: &str)**
//!   - Removes the specified application-password pair from both in-memory data and the file.
//!   - Rewrites the file with updated content after removal.
//!
//! - **update_password(app_name: &str, new_password: &str)**
//!   - Updates the password for a specified application in in-memory data and the file.
//!   - Rewrites the file with new content after updating.
//!
//! - **export_passwords(file_path: &str)**
//!   - Exports all application-password pairs from in-memory data to the specified file.
//!
//! - **import_passwords(file_path: &str)**
//!   - Imports application-password pairs from a file and merges them into in-memory data.
//!   - Saves the resulting data back to the main storage file.
//!
//! - **generate_otp(app_name: &str, ttl_seconds: u64)**
//!   - Generates and prints a random one-time password (OTP) associated with the specified application.
//!   - Displays the expiration time based on `ttl_seconds`.
use std::fs::*;
use std::io::*;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use rand::distributions::Alphanumeric;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use std::sync::{Arc, Mutex};

const FILE_NAME: &str = "passwords";

static APPLICATION_DATA: Lazy<std::sync::Mutex<HashMap<String, String>>> = Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

/// Fills the `APPLICATION_DATA` with data from the file specified by `FILE_NAME`.
fn fill_data() {
    let mut application_data = APPLICATION_DATA.lock().unwrap();
    let file = File::open(FILE_NAME).expect("Could not open the file");

    let reader: BufReader<File> = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let key_value: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if key_value.len() == 2 {
                let app_name = key_value[0].to_string();
                let password = key_value[1].to_string();
                application_data.insert(app_name, password);
            }
        }
    }
}

/// Generates a random password, saves it to the file, and updates the application data.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
pub fn generate_save_safety_password(app_name: &str, length: Option<usize>) {
    let length = length.unwrap_or(30); // Longitud predeterminada: 30 caracteres

    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(FILE_NAME)
        .unwrap();

    if let Err(e) = writeln!(file, "{},{}", app_name, rand_password) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("Password generated and saved for the application: {}", app_name);
}

/// Shows a list of all applications and their passwords.
pub fn show_list_applications() {
    fill_data();
    let application_data = APPLICATION_DATA.lock().unwrap();

    for (key, value) in application_data.iter() {
        print!("Application_Name: {}\n", key);
        print!("Password: {}\n", value);
        println!();
    }
}

/// Retrieves the password for a specified application.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
pub fn get_password_for_specify_app(app_name: &str) {
    fill_data();
    let application_data = APPLICATION_DATA.lock().unwrap();

    let to_find = [app_name];
    for &name_app in &to_find {
        match application_data.get(name_app) {
            Some(password) => {
                print!("Application_Name: {}\n", name_app);
                print!("Password: {}\n", password);
                println!();
            },
            None => println!("{name_app} don't exists.")
        }
    }
}

/// Deletes the password for a specified application.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
pub fn delete_password(app_name: &str) {
    fill_data();
    let mut application_data = APPLICATION_DATA.lock().unwrap();

    if application_data.remove(app_name).is_some() {
        let new_content: String = application_data
            .iter()
            .map(|(app, pass)| format!("{},{}", app, pass))
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(FILE_NAME, new_content).expect("Unable to write to file");
        println!("Application '{}' deleted successfully.", app_name);
    } else {
        println!("Application '{}' not found.", app_name);
    }
}

/// Updates the password for a specified application.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application.
/// * `new_password` - A string slice that holds the new password for the application.
pub fn update_password(app_name: &str, new_password: &str) {
    fill_data();
    let mut application_data = APPLICATION_DATA.lock().unwrap();

    if application_data.contains_key(app_name) {
        application_data.insert(app_name.to_string(), new_password.to_string());

        let new_content: String = application_data
            .iter()
            .map(|(app, pass)| format!("{},{}", app, pass))
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(FILE_NAME, new_content).expect("Unable to write to file");
        println!("Password updated for '{}'.", app_name);
    } else {
        println!("Application '{}' not found.", app_name);
    }
}

/// Exports all stored passwords to a specified file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file where passwords will be exported.
pub fn export_passwords(file_path: &str) {
    fill_data();
    let application_data = APPLICATION_DATA.lock().unwrap();

    let mut file = File::create(file_path).expect("Failed to create export file");

    for (app, pass) in application_data.iter() {
        writeln!(file, "{},{}", app, pass).expect("Failed to write to export file");
    }

    println!("Passwords exported to '{}'.", file_path);
}

/// Imports passwords from a specified file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file to import passwords from.
pub fn import_passwords(file_path: &str) {
    let file = File::open(file_path).expect("Failed to open import file");
    let reader = BufReader::new(file);

    let mut application_data = APPLICATION_DATA.lock().unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            let key_value: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if key_value.len() == 2 {
                let app_name = key_value[0].to_string();
                let password = key_value[1].to_string();
                application_data.insert(app_name, password);
            }
        }
    }

    let new_content: String = application_data
        .iter()
        .map(|(app, pass)| format!("{},{}", app, pass))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(FILE_NAME, new_content).expect("Unable to write to file");

    println!("Passwords imported from '{}'.", file_path);
}

/// Generates a random one-time password (OTP) and prints it along with the expiration time.
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

/// Generates a memorizable password for a specified application.
pub fn generate_memorizable_password(app_name: &str) {
    let words = vec!["Tiger", "Orange", "Mountain", "River", "Cloud"];
    let mut rng = thread_rng();

    let password = format!(
        "{}-{}-{}",
        words.choose(&mut rng).unwrap(),
        thread_rng().gen_range(10..99),
        words.choose(&mut rng).unwrap()
    );

    println!("Memorizable Password for '{}': {}", app_name, password);
}

/// Starts a thread that locks the application after a specified timeout.
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