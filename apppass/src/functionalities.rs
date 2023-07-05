use std::fs::*;
use std::io::*;
use rand::{thread_rng, Rng};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use rand::distributions::Alphanumeric;

const FILE_NAME: &str = "passwords";

static APPLICATION_DATA: Lazy<std::sync::Mutex<HashMap<String, String>>> = Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

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

pub fn generate_save_safety_password(app_name: &str) {

    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
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

    fill_data();

    println!("Password generated and saved for the application: {}", app_name);
}

pub fn show_list_applications() {
    fill_data();
    let application_data = APPLICATION_DATA.lock().unwrap();

    for (key, value) in application_data.iter() {
        print!("Application_Name: {}\n", key);
        print!("Password: {}\n", value);
        println!();
    }
}

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