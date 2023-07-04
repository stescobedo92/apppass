mod functionalities;

use clap::{App, Arg };

fn main() {
    let passgen = App::new("passgen")
        .version("1.0")
        .author("Sergio Triana Escobedo")
        .about("Generate secure passwords for your applications.")
        .arg(
            Arg::with_name("app")
                .short("a")
                .long("app")
                .value_name("APP_NAME")
                .help("Application name")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Show list of applications")
                .required(false),
        )
        .arg(
            Arg::with_name("get")
                .short("g")
                .long("get")
                .value_name("APP_NAME")
                .takes_value(true)
                .help("Get data passing an application name")
                .required(false),
        )
        .get_matches();

    if passgen.is_present("app") {
        let name = matches.value_of("app").unwrap();
        functionalities::generate_save_safety_password(name);
    }

    if passgen.is_present("list") {
        functionalities::show_list_applications();
    }

    if passgen.is_present("get") {
        let name = matches.value_of("get").unwrap();
        functionalities::get_password_for_specify_app(name);
    }
}
