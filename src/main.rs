use std::env::args;

use crate::{
    cli::{ApplicationError, display_help, display_version, print_error},
    clipboard::copy_password,
    password::generate_password,
};

mod cli;
mod clipboard;
mod password;

fn main() {
    let args = args().collect::<Vec<String>>();

    match args.iter().nth(1) {
        Some(arg) => match arg.as_str() {
            "-c" | "--copy" => match copy_password(&generate_password()) {
                Ok(_) => println!("The generated password was copied to the clipboard"),
                Err(err) => eprintln!("{err}"),
            },
            "-h" | "--help" => display_help(),
            "-v" | "--version" => display_version(),
            _ => print_error(ApplicationError::UnknownFlag(arg.to_owned())),
        },
        None => println!("{}", generate_password()),
    }
}
