use std::io::Write;
use std::process::exit;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

use colored::Colorize;
use clap::Parser;
use converter::get_vectorized_image;
use template::MANIFEST;

mod args;
mod image;
mod template;
mod converter;

fn main() {
    let mut args = args::Args::parse();

    match args::validate_args(args) {
        Ok(arguments) => args = arguments,
        Err(err) => {
            println!("❌ {}", err.red());
            exit(0);
        },
    }

    let platforms = args.platforms.unwrap();

    let output_folder = args.output.as_ref().unwrap();
    if !output_folder.is_dir() {
        let create_folder = fs::create_dir(output_folder);

        if create_folder.is_err() {
            println!("❌ {}", "Failed to create output folder".red());
            exit(0);
        }
    }
    
    let source_path = args.source.as_path();

    let ext = source_path.extension()
        .and_then(OsStr::to_str)
        .unwrap();
    let input: String;

    if ext != "svg" {
        input = get_vectorized_image(args.source);
    } else {
        input = fs::read_to_string(source_path).unwrap()
    }

    let image_data = image::generate_image_data(input, platforms.clone());

    for output in image_data.iter() {
        let mut path = PathBuf::from(args.output.as_ref().unwrap());
        path.push(output.name.clone());

        let mut file = fs::File::create(path).unwrap();
        let res = file.write_all(&output.data);

        if res.is_err() {
            println!("❌ Failed to write {}", output.name.red());
        }
    }

    if platforms.contains(&args::Platform::Android) {
        let mut path = PathBuf::from(args.output.as_ref().unwrap());
        path.push("manifest.webmanifest");

        let mut file = fs::File::create(path).unwrap();
        let res = file.write_all(MANIFEST.as_bytes());

        if res.is_err() {
            println!("❌ Failed to write {}", "webmanifest".red());
        }
    }

    if args.template {
        let mut path = PathBuf::from(args.output.as_ref().unwrap());
        path.push("index.html");

        let mut file = fs::File::create(path).unwrap();
        let res = file.write_all(template::generate_template(platforms).as_bytes());

        if res.is_err() {
            println!("❌ Failed to write {}", "HTML template".red());
        }
    }

    println!("✔️ {}", "Favicons generated successfully".green())
}
