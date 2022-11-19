use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::error;

#[derive(Parser)]
#[command(version, about)]
/// Generate a complete and ready-to-use favicons for your websites
pub struct Args {
    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath)]
    pub input: PathBuf,

    #[arg(short = 'p', value_name = "platforms", value_enum)]
    pub platforms: Option<Vec<Platform> >,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Platform {
    // Favicons that are compatible with almost all web browsers
    Web,
    // Include favicons features that are supported by major modern browsers
    Modern,
    // Enable Android-based favicon support. Includes manifest.
    Android,
    // Enable Apple-based device favicon support.
    Apple,
}

// Parse shell arguments and assign defaults if `None`
pub fn parse_args() -> Result<Args, String> {
    let mut args = Args::parse();

    if !args.input.exists() {
        return Err("Input file does not exists".to_string());
    }

    if args.platforms.is_none() {
        args.platforms = Option::from(Vec::from([Platform::Web, Platform::Modern]));
    }

    return Ok(args);
}