const APP_NAME: &str = env!("CARGO_BIN_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn display_help() {
    println!("Usage: {APP_NAME} [OPTION]");
    println!();
    println!("Options:");
    println!("\t-v, --version       Print app version");
    println!("\t-h, --help          Display help text");
    println!("\t-c, --copy          Copy the generated password to clipboard");
}

pub fn display_version() {
    println!("{APP_NAME} {APP_VERSION}")
}

pub fn print_error(err: ApplicationError) {
    match err {
        ApplicationError::UnknownFlag(flag) => eprintln!("{APP_NAME}: unrecognized flag {flag}"),
    }
}

pub enum ApplicationError {
    UnknownFlag(String),
}
