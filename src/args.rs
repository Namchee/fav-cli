use clap::{Parser, ValueEnum};
use std::{path::PathBuf, env};

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

    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath)]
    // Output folder
    pub output: Option<PathBuf>,

    #[arg(short = 't', default_value_t = false)]
    // Generate HTML template
    pub template: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Hash)]
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

// Validate shell arguments and assign defaults if `None`
pub fn validate_args(mut args: Args) -> Result<Args, String> {
    if !args.source.exists() {
        return Err("Source file does not exists".to_string());
    }

    if args.platforms.is_none() {
        args.platforms = Option::from(Vec::from([Platform::Web, Platform::Modern]));
    }

    if args.output.is_none() {
        let cwd = env::current_dir().unwrap();
        let mut output_path = PathBuf::new();

        output_path.push(cwd);
        output_path.push("output");

        args.output = Option::from(output_path);
    }

    return Ok(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_default_platforms() {
        let args = Args {
            source: PathBuf::from("samples/sample.svg"),
            platforms: Option::None,
            fill: false,
            output: Option::from(PathBuf::from("here")),
            template: false,
        };
        let result = validate_args(args);

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap().platforms.unwrap(), Vec::from([Platform::Web, Platform::Modern]));
    }

    #[test]
    fn test_assign_default_output() {
        let args = Args {
            source: PathBuf::from("samples/sample.svg"),
            platforms: Option::from(Vec::from([Platform::Web, Platform::Modern])),
            fill: false,
            output: Option::None,
            template: false,
        };
        let result = validate_args(args);

        let cwd = env::current_dir().unwrap();
        let mut path = PathBuf::new();

        path.push(cwd);
        path.push("output");

        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap().output.unwrap().to_str(), path.to_str());
    }
}