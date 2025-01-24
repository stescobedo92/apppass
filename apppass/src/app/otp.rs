#[warn(unused_variables)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_otp(_app_name: &str, ttl_seconds: u64) {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        + Duration::new(ttl_seconds, 0);

    let otp: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    println!("Temporary Password: {}", otp);
    println!("Expires at: {:?}", expiration);
}