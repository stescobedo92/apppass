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