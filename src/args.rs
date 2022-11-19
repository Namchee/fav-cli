use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
/// Generate a complete and ready-to-use favicons for your websites
pub struct Args {
    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath)]
    // Image source
    pub source: PathBuf,

    #[arg(short = 'p', value_name = "platforms", value_enum)]
    // Platforms that should be supported
    pub platforms: Option<Vec<Platform> >,

    #[arg(short = 'f', value_name = "fill", default_value_t = false)]
    // Ignore original aspect ratio from the image source
    pub fill: bool,
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

    if !args.source.exists() {
        return Err("Source file does not exists".to_string());
    }

    if args.platforms.is_none() {
        args.platforms = Option::from(Vec::from([Platform::Web, Platform::Modern]));
    }

    return Ok(args);
}