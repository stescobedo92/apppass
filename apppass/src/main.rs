mod functionalities;

use clap::{Arg, ArgAction, Command};

fn main() {
    let apppass = Command::new("apppass")
        .version("1.2")
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
        .get_matches();

    if let Some(name) = apppass.get_one::<String>("app") {
        let length = apppass
            .get_one::<String>("length")
            .and_then(|l| l.parse::<usize>().ok());
        functionalities::generate_save_safety_password(name, length);
    }

    if *apppass.get_one::<bool>("list").unwrap_or(&false) {
        functionalities::show_list_applications();
    }

    if let Some(name) = apppass.get_one::<String>("get") {
        functionalities::get_password_for_specify_app(name);
    }

    if let Some(name) = apppass.get_one::<String>("delete") {
        functionalities::delete_password(name);
    }

    if let Some(name) = apppass.get_one::<String>("update") {
        let new_pass = "new_secure_password";
        functionalities::update_password(name, new_pass);
    }

    if let Some(path) = apppass.get_one::<String>("export") {
        functionalities::export_passwords(path);
    }

    if let Some(path) = apppass.get_one::<String>("import") {
        functionalities::import_passwords(path);
    }
}
