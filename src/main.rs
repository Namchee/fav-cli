use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
/// Generate a complete and ready-to-use favicons for your websites
struct Args {
    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath)]
    input: PathBuf,

    #[arg(short = 'p', value_name = "platforms", value_enum)]
    platforms: Option<Vec<Platform> >,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Platform {
    // Favicons that are compatible with almost all web browsers
    Web,
    // Include favicons features that are supported by major modern browsers
    Modern,
    // Enable Android-based favicon support. Includes manifest.
    Android,
    // Enable Apple-based device favicon support.
    Apple,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.input.as_path().to_str().unwrap());

    let mut platforms = Vec::from([Platform::Web, Platform::Modern]);

    if !args.platforms.is_none() {
        platforms = args.platforms.unwrap();
    }

    println!("{:?}", platforms[1]);
}
