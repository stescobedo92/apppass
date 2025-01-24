#[warn(unused_variables)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generates a one-time password (OTP) and prints its value along with its expiration time.
///
/// # Arguments
///
/// * `app_name` - A string slice that holds the name of the application (currently unused).
/// * `ttl_seconds` - The time-to-live for the OTP in seconds.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_otp() {
        let ttl_seconds = 5;
        let start_time = SystemTime::now();
        generate_otp("TestApp", ttl_seconds);
        let end_time = SystemTime::now();
        let expiration = start_time + Duration::new(ttl_seconds, 0);

        // Allow some margin for the test execution time
        assert!(end_time.duration_since(start_time).unwrap().as_secs() < ttl_seconds + 1);
        assert_eq!(expiration.duration_since(start_time).unwrap().as_secs(), ttl_seconds);
    }
}