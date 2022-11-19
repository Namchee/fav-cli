use std::process::exit;

use colored::Colorize;

mod args;

fn main() {
    let args: args::Args;

    match args::parse_args() {
        Ok(arguments) => args = arguments,
        Err(err) => {
            println!("❌ {}", err.red());
            exit(0);
        },
    }
}
