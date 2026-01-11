pub mod keyring;
pub mod password;
pub mod otp;
pub mod lock;

pub static APP_INDEX: &str = "apppass_index";
pub static APP_SERVICE: &str = "apppass";
pub static PASSWORD_LENGTH_KEY: &str = "password_length";
pub static PASSWORD_TYPE_SUFFIX: &str = "_type";
pub static OTP_EXPIRY_SUFFIX: &str = "_otp_expiry";
