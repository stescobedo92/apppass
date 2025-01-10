use std::fs::*;
use std::io::*;
use rand::{thread_rng, Rng};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use rand::distributions::Alphanumeric;

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