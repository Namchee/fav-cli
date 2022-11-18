use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(value_name = "source_image", value_hint = clap::ValueHint::DirPath)]
    input: PathBuf,

    #[arg(short = 'p', value_name = "platforms")]
    platforms: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.input.as_path().to_str().unwrap());
    println!("{}", args.platforms.unwrap().join(", "))
}
