mod app;
mod ui;

use clap::{Arg, ArgAction, Command};
use crate::app::keyring::show_list_applications;
use crate::app::lock::start_auto_lock;
use crate::app::otp::generate_otp;
use crate::app::password::{delete_password, export_passwords, generate_memorizable_password,
                           generate_save_safety_password, get_password_for_specify_app,
                           import_passwords, update_password};

fn main() {
    let apppass = Command::new("apppass")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sergio Triana Escobedo")
        .about("Generate secure passwords for your applications.")
        .arg(
            Arg::new("app")
                .short('a')
                .long("app")
                .action(ArgAction::Set)
                .help("Generate a password for an application"),
        )
        .arg(
            Arg::new("length")
                .short('n')
                .long("length")
                .action(ArgAction::Set)
                .help("Password length"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("List all applications"),
        )
        .arg(
            Arg::new("get")
                .short('g')
                .long("get")
                .action(ArgAction::Set)
                .help("Get password for an application"),
        )
        .arg(
            Arg::new("delete")
                .short('d')
                .long("delete")
                .action(ArgAction::Set)
                .help("Delete an application"),
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .action(ArgAction::Set)
                .help("Update password for an application"),
        )
        .arg(
            Arg::new("export")
                .long("export")
                .action(ArgAction::Set)
                .help("Export passwords to CSV"),
        )
        .arg(
            Arg::new("import")
                .long("import")
                .action(ArgAction::Set)
                .help("Import passwords from CSV"),
        )
        .arg(
            Arg::new("otp")
                .long("otp")
                .action(ArgAction::Set)
                .help("Generate a one-time password (OTP)"),
        )
        .arg(
            Arg::new("ttl")
                .long("ttl")
                .action(ArgAction::Set)
                .help("Time-to-live for OTP in seconds (default: 300)"),
        )
        .arg(
            Arg::new("memorizable")
                .long("memorizable")
                .action(ArgAction::Set)
                .help("Generate a memorizable password for an application"),
        )
        .arg(
            Arg::new("lock")
                .long("lock")
                .action(ArgAction::Set)
                .help("Set auto-lock timeout in seconds"),
        )
        .arg(
            Arg::new("ui")
                .long("ui")
                .action(ArgAction::SetTrue)
                .help("Launch interactive UI mode"),
        )
        .get_matches();

    // If UI flag is set, launch the interactive TUI
    if *apppass.get_one::<bool>("ui").unwrap_or(&false) {
        if let Err(e) = ui::run_tui() {
            eprintln!("Error running UI: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if let Some(name) = apppass.get_one::<String>("app") {
        let length = apppass
            .get_one::<String>("length")
            .and_then(|l| l.parse::<usize>().ok());
        match generate_save_safety_password(name, length) {
            Ok(_) => println!("Password saved securely for '{}'.", name),
            Err(_) => eprintln!("Password already exists for '{}'. Use update to change it.", name),
        }
    }

    if *apppass.get_one::<bool>("list").unwrap_or(&false) {
        show_list_applications();
    }

    if let Some(name) = apppass.get_one::<String>("get") {
        match get_password_for_specify_app(name) {
            Ok(password) => {
                println!("Application Name: {}", name);
                println!("Password: {}", password);
            }
            Err(_) => println!("No password found for '{}'.", name),
        }
    }

    if let Some(name) = apppass.get_one::<String>("delete") {
        match delete_password(name) {
            Ok(_) => println!("Password for '{}' deleted successfully.", name),
            Err(_) => println!("No password found for '{}'.", name),
        }
    }

    if let Some(name) = apppass.get_one::<String>("update") {
        let new_pass = "new_secure_password";
        match update_password(name, new_pass) {
            Ok(_) => println!("Password updated successfully for '{}'.", name),
            Err(_) => eprintln!("No password found for '{}'. Use create to add a new password.", name),
        }
    }

    if let Some(path) = apppass.get_one::<String>("export") {
        match export_passwords(path) {
            Ok(_) => println!("Passwords exported to '{}'.", path),
            Err(_) => eprintln!("Failed to export passwords to '{}'.", path),
        }
    }

    if let Some(path) = apppass.get_one::<String>("import") {
        match import_passwords(path) {
            Ok(_) => println!("Passwords imported from '{}'.", path),
            Err(_) => eprintln!("Failed to import passwords from '{}'.", path),
        }
    }

    if let Some(name) = apppass.get_one::<String>("otp") {
        let ttl = apppass
            .get_one::<String>("ttl")
            .and_then(|t| t.parse::<u64>().ok())
            .unwrap_or(300);
        
        // Load password length from keyring, default to 30
        let password_length = crate::app::keyring::get_from_keyring("password_length")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(30);
        
        match generate_otp(name, ttl, password_length) {
            Ok(otp) => {
                println!("OTP generated and saved for '{}'", name);
                println!("Temporary Password: {}", otp);
                println!("Expires in: {} seconds", ttl);
                println!("\nThis password will be automatically deleted from the keyring after {} seconds.", ttl);
            }
            Err(e) => eprintln!("Failed to generate OTP: {}", e),
        }
    }

    if let Some(name) = apppass.get_one::<String>("memorizable") {
        match generate_memorizable_password(name) {
            Ok(_) => println!("Memorizable password saved for '{}'.", name),
            Err(_) => eprintln!("Password already exists for '{}'. Use update to change it.", name),
        }
    }

    if let Some(lock_time) = apppass.get_one::<String>("lock") {
        let timeout = lock_time.parse::<u64>().unwrap_or(60);
        start_auto_lock(timeout);
    }
}

