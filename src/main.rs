mod app;

#[cfg(feature = "tui")]
mod ui;

#[cfg(feature = "console")]
use clap::{Arg, ArgAction, Command};

#[cfg(feature = "console")]
use crate::app::keyring::show_list_applications;
#[cfg(feature = "console")]
use crate::app::keyring::cleanup_orphaned_index;
#[cfg(feature = "console")]
use crate::app::lock::start_auto_lock;
#[cfg(feature = "console")]
use crate::app::otp::generate_otp;
#[cfg(feature = "console")]
use crate::app::otp::cleanup_expired_otps;
#[cfg(feature = "console")]
use crate::app::password::{delete_password, export_passwords, generate_memorizable_password,
                           generate_save_safety_password, get_password_for_specify_app,
                           import_passwords, update_password, update_password_regenerate};

fn main() {
    #[cfg(feature = "console")]
    run_cli();
    
    #[cfg(all(not(feature = "console"), feature = "tui"))]
    {
        if let Err(e) = ui::run_tui() {
            eprintln!("Error running UI: {}", e);
            std::process::exit(1);
        }
    }
    
    #[cfg(all(not(feature = "console"), not(feature = "tui")))]
    {
        eprintln!("No features enabled. Please enable 'console' or 'tui' feature.");
        std::process::exit(1);
    }
}

#[cfg(feature = "console")]
fn run_cli() {
    // Cleanup at startup
    cleanup_orphaned_index();
    cleanup_expired_otps();
    
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
                .help("Update password for an application (regenerates a new secure password)"),
        )
        .arg(
            Arg::new("update-custom")
                .long("update-custom")
                .action(ArgAction::Set)
                .help("Update password for an application with a custom password (requires --password)"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .action(ArgAction::Set)
                .help("Custom password to use with --update-custom"),
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
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .help("Launch interactive console menu mode"),
        );
    
    #[cfg(feature = "tui")]
    let apppass = apppass.arg(
        Arg::new("ui")
            .long("ui")
            .action(ArgAction::SetTrue)
            .help("Launch interactive UI mode (TUI)"),
    );
    
    let apppass = apppass.get_matches();

    // If interactive flag is set, launch the interactive console menu
    if *apppass.get_one::<bool>("interactive").unwrap_or(&false) {
        run_interactive_console();
        return;
    }

    // If UI flag is set, launch the interactive TUI
    #[cfg(feature = "tui")]
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

    // Update command - regenerates a new secure password
    if let Some(name) = apppass.get_one::<String>("update") {
        let length = apppass
            .get_one::<String>("length")
            .and_then(|l| l.parse::<usize>().ok());
        match update_password_regenerate(name, length) {
            Ok(new_password) => {
                println!("Password updated successfully for '{}'.", name);
                println!("New Password: {}", new_password);
            }
            Err(_) => eprintln!("No password found for '{}'. Use -a/--app to create a new password.", name),
        }
    }

    // Update custom command - uses user-provided password
    if let Some(name) = apppass.get_one::<String>("update-custom") {
        if let Some(new_pass) = apppass.get_one::<String>("password") {
            match update_password(name, new_pass) {
                Ok(_) => println!("Password updated successfully for '{}'.", name),
                Err(_) => eprintln!("No password found for '{}'. Use -a/--app to create a new password.", name),
            }
        } else {
            eprintln!("Error: --update-custom requires --password/-p to specify the new password.");
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
        let password_length = crate::app::keyring::get_from_keyring(crate::app::PASSWORD_LENGTH_KEY)
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

/// Helper function to read a line from stdin
#[cfg(feature = "console")]
fn read_line() -> String {
    use std::io::{self, BufRead, Write};
    io::stdout().flush().ok();
    
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.lock().read_line(&mut line) {
        Ok(_) => line.trim().to_string(),
        Err(_) => String::new(),
    }
}

/// Helper function to prompt and read input
#[cfg(feature = "console")]
fn prompt(message: &str) -> String {
    use std::io::{self, Write};
    print!("{}", message);
    io::stdout().flush().ok();
    read_line()
}

/// Interactive console menu mode
#[cfg(feature = "console")]
fn run_interactive_console() {
    println!("\n╔══════════════════════════════════════════╗");
    println!("║         APPPASS - Password Manager       ║");
    println!("║         Interactive Console Mode         ║");
    println!("╚══════════════════════════════════════════╝");
    
    loop {
        // Check password availability for menu display
        let has_passwords = crate::app::keyring::has_any_passwords();
        let has_auto = crate::app::keyring::has_auto_passwords();
        let has_custom = crate::app::keyring::has_custom_passwords();
        
        println!("\n┌──────────────────────────────────────────┐");
        println!("│              MAIN MENU                   │");
        println!("├──────────────────────────────────────────┤");
        println!("│  1. Create Password (Auto-generated)     │");
        println!("│  2. Create Password (Custom)             │");
        if has_passwords {
            println!("│  3. List All Passwords                   │");
        } else {
            println!("│  3. List All Passwords (No passwords)    │");
        }
        println!("│  4. Get Password                         │");
        if has_auto {
            println!("│  5. Update Password (Regenerate)         │");
        } else {
            println!("│  5. Update Password (Regenerate) (No auto)│");
        }
        if has_custom {
            println!("│  6. Update Password (Custom)             │");
        } else {
            println!("│  6. Update Password (Custom) (No custom) │");
        }
        if has_passwords {
            println!("│  7. Delete Password                      │");
        } else {
            println!("│  7. Delete Password (No passwords)       │");
        }
        println!("│  8. Generate OTP                         │");
        println!("│  9. Generate Memorizable Password        │");
        if has_passwords {
            println!("│ 10. Export to CSV                        │");
        } else {
            println!("│ 10. Export to CSV (No passwords)        │");
        }
        println!("│ 11. Import from CSV                      │");
        println!("│  0. Exit                                 │");
        println!("└──────────────────────────────────────────┘");
        
        let choice = prompt("\nSelect option: ");
        
        match choice.as_str() {
            "1" => {
                let app_name = prompt("Application name: ");
                if app_name.is_empty() {
                    println!("✗ Application name cannot be empty");
                    continue;
                }
                
                let length_str = prompt("Password length [30]: ");
                let length: Option<usize> = if length_str.is_empty() {
                    None
                } else {
                    length_str.parse().ok()
                };
                
                match generate_save_safety_password(&app_name, length) {
                    Ok(_) => println!("✓ Password saved for '{}'", app_name),
                    Err(_) => println!("✗ Password already exists for '{}'", app_name),
                }
            }
            "2" => {
                let app_name = prompt("Application name: ");
                if app_name.is_empty() {
                    println!("✗ Application name cannot be empty");
                    continue;
                }
                
                let password = prompt("Custom password: ");
                if password.is_empty() {
                    println!("✗ Password cannot be empty");
                    continue;
                }
                
                match crate::app::keyring::save_to_keyring(&app_name, &password) {
                    Ok(_) => {
                        let _ = crate::app::keyring::set_password_type(&app_name, "custom");
                        println!("✓ Custom password saved for '{}'", app_name);
                    }
                    Err(e) => println!("✗ Error: {}", e),
                }
            }
            "3" => {
                if !crate::app::keyring::has_any_passwords() {
                    println!("✗ No passwords to list");
                    continue;
                }
                println!("\n--- Stored Passwords ---");
                show_list_applications();
            }
            "4" => {
                let app_name = prompt("Application name: ");
                match get_password_for_specify_app(&app_name) {
                    Ok(password) => {
                        println!("Application: {}", app_name);
                        println!("Password: {}", password);
                    }
                    Err(_) => println!("✗ No password found for '{}'", app_name),
                }
            }
            "5" => {
                if !crate::app::keyring::has_auto_passwords() {
                    println!("✗ No auto-generated passwords to update");
                    continue;
                }
                let app_name = prompt("Application name: ");
                let length_str = prompt("New password length [30]: ");
                let length: Option<usize> = if length_str.is_empty() {
                    None
                } else {
                    length_str.parse().ok()
                };
                
                match update_password_regenerate(&app_name, length) {
                    Ok(new_password) => {
                        println!("✓ Password updated for '{}'", app_name);
                        println!("New Password: {}", new_password);
                    }
                    Err(_) => println!("✗ No password found for '{}'", app_name),
                }
            }
            "6" => {
                if !crate::app::keyring::has_custom_passwords() {
                    println!("✗ No custom passwords to update");
                    continue;
                }
                let app_name = prompt("Application name: ");
                let password = prompt("New password: ");
                if password.is_empty() {
                    println!("✗ Password cannot be empty");
                    continue;
                }
                
                match update_password(&app_name, &password) {
                    Ok(_) => println!("✓ Password updated for '{}'", app_name),
                    Err(_) => println!("✗ No password found for '{}'", app_name),
                }
            }
            "7" => {
                if !crate::app::keyring::has_any_passwords() {
                    println!("✗ No passwords to delete");
                    continue;
                }
                let app_name = prompt("Application name to delete: ");
                let confirm = prompt(&format!("Delete '{}'? (y/N): ", app_name));
                
                if confirm.to_lowercase() == "y" {
                    match delete_password(&app_name) {
                        Ok(_) => println!("✓ Password deleted for '{}'", app_name),
                        Err(_) => println!("✗ No password found for '{}'", app_name),
                    }
                } else {
                    println!("Cancelled.");
                }
            }
            "8" => {
                let app_name = prompt("OTP application name: ");
                let ttl_str = prompt("TTL in seconds [300]: ");
                let ttl: u64 = if ttl_str.is_empty() {
                    300
                } else {
                    ttl_str.parse().unwrap_or(300)
                };
                
                let password_length = crate::app::keyring::get_from_keyring(crate::app::PASSWORD_LENGTH_KEY)
                    .ok()
                    .and_then(|v| v.parse::<usize>().ok())
                    .unwrap_or(30);
                
                match generate_otp(&app_name, ttl, password_length) {
                    Ok(otp) => {
                        println!("✓ OTP generated for '{}'", app_name);
                        println!("Password: {}", otp);
                        println!("Expires in: {} seconds", ttl);
                    }
                    Err(e) => println!("✗ Error: {}", e),
                }
            }
            "9" => {
                let app_name = prompt("Application name: ");
                match generate_memorizable_password(&app_name) {
                    Ok(_) => println!("✓ Memorizable password saved for '{}'", app_name),
                    Err(_) => println!("✗ Password already exists for '{}'", app_name),
                }
            }
            "10" => {
                if !crate::app::keyring::has_any_passwords() {
                    println!("✗ No passwords to export");
                    continue;
                }
                let path = prompt("Export file path: ");
                match export_passwords(&path) {
                    Ok(_) => println!("✓ Exported to '{}'", path),
                    Err(_) => println!("✗ Export failed"),
                }
            }
            "11" => {
                let path = prompt("Import file path: ");
                match import_passwords(&path) {
                    Ok(_) => println!("✓ Imported from '{}'", path),
                    Err(_) => println!("✗ Import failed"),
                }
            }
            "0" | "q" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "" => {
                // Empty input, just show menu again
                continue;
            }
            _ => {
                println!("Invalid option");
            }
        }    }
}