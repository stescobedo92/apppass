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
        generate_save_safety_password(name, length);
    }

    if *apppass.get_one::<bool>("list").unwrap_or(&false) {
        show_list_applications();
    }

    if let Some(name) = apppass.get_one::<String>("get") {
        get_password_for_specify_app(name);
    }

    if let Some(name) = apppass.get_one::<String>("delete") {
        delete_password(name);
    }

    if let Some(name) = apppass.get_one::<String>("update") {
        let new_pass = "new_secure_password";
        update_password(name, new_pass);
    }

    if let Some(path) = apppass.get_one::<String>("export") {
        export_passwords(path);
    }

    if let Some(path) = apppass.get_one::<String>("import") {
        import_passwords(path);
    }

    if let Some(name) = apppass.get_one::<String>("otp") {
        let ttl = apppass
            .get_one::<String>("ttl")
            .and_then(|t| t.parse::<u64>().ok())
            .unwrap_or(300);
        generate_otp(name, ttl);
    }

    if let Some(name) = apppass.get_one::<String>("memorizable") {
        generate_memorizable_password(name);
    }

    if let Some(lock_time) = apppass.get_one::<String>("lock") {
        let timeout = lock_time.parse::<u64>().unwrap_or(60);
        start_auto_lock(timeout);
    }
}

