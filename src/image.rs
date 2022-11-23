use std::path::{PathBuf};
use std::fs;
use std::{collections::HashMap, io::Write};
use std::ffi::OsStr;

use crate::args::{Args, Platform};

struct OutputFile {
    size: u32,
    name: String,
}

pub fn generate_image(args: Args) {
    let size_map: HashMap<Platform, Vec<OutputFile> > = HashMap::from([
        (Platform::Web, vec![OutputFile{ size: 32, name: "favicon.ico".to_string() }]),
        (Platform::Android, vec![OutputFile{ size: 192, name: "192.png".to_string() }, OutputFile{ size: 512, name: "512.png".to_string() }]),
        (Platform::Apple, vec![OutputFile{ size: 180, name: "apple_touch_icon.png".to_string() }]),
    ]);
    
    // TODO: use this to check if should rasterize or not
    let ext = args.source.extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let input = fs::read_to_string(args.source).unwrap();

    let opts = resvg::usvg::Options::default();
    let svg = resvg::usvg::Tree::from_str(
        input.as_str(),
        &opts.to_ref(),
    ).unwrap();

    let output_folder = args.output.unwrap();

    for platform in args.platforms.unwrap().iter() {
        let sizes = size_map.get(platform);

        if !sizes.is_none() {
            let target = sizes.unwrap();

            for output in target.iter() {
                let mut pixmap = resvg::tiny_skia::Pixmap::new(
                    output.size, 
                    output.size,
                ).unwrap();

                resvg::render(
                    &svg,
                    resvg::usvg::FitTo::Size(output.size, output.size),
                    resvg::tiny_skia::Transform::default(),
                 pixmap.as_mut(),
                ).unwrap();

                let mut parent = PathBuf::from(output_folder.as_os_str());
                parent.push(&output.name);

                let result = pixmap.save_png(parent);

                if result.is_err() {    
                    println!("error")
                }
            }
        } else {
            let mut parent = PathBuf::from(output_folder.as_os_str());
            parent.push("icon.svg");

            let file = fs::File::create(parent);
            let result = file.unwrap().write_all(input.as_bytes());

            if result.is_err() {
                println!("error");
            }
        }
    }
}