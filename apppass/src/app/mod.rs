use std::collections::HashSet;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub mod keyring;
pub mod password;
pub mod otp;
pub mod lock;

static APP_INDEX: &str = "apppass_index";
static APP_SERVICE: &str = "apppass";
static APPLICATION_DATA: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));
