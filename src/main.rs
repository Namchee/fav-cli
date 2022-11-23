use std::process::exit;
use std::fs;
use colored::Colorize;

use clap::Parser;

mod args;
mod image;

fn main() {
    let mut args = args::Args::parse();

    match args::validate_args(args) {
        Ok(arguments) => args = arguments,
        Err(err) => {
            println!("❌ {}", err.red());
            exit(0);
        },
    }

    let output_folder = args.output.as_ref().unwrap();
    if !output_folder.is_dir() {
        let create_folder = fs::create_dir(output_folder);

        if create_folder.is_err() {
            println!("❌ {}", "Failed to create output folder");
            exit(0);
        }
    }

    image::generate_image(args);
}
