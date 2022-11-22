use std::path::PathBuf;
use std::process::exit;
use std::fs;
use colored::Colorize;

mod args;
mod image;

fn main() {
    let args: args::Args;

    match args::parse_args() {
        Ok(arguments) => args = arguments,
        Err(err) => {
            println!("❌ {}", err.red());
            exit(0);
        },
    }

    let output_folder = args.output.as_ref().unwrap();
    if !output_folder.is_dir() {
        fs::create_dir(output_folder);
    }

    image::generate_image(args);
}
