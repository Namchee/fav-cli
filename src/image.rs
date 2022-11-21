use std::collections::HashMap;
use std::ffi::OsStr;

use std::fs;

use crate::args::{Args, Platform};

pub fn generate_image(args: Args) {
    let size_map: HashMap<Platform, Vec<u32> > = HashMap::from([
        (Platform::Web, vec![512]),
        (Platform::Android, vec![192, 512]),
        (Platform::Apple, vec![180]),
    ]);
    
    let ext = args.source.extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let input = fs::read_to_string(args.source).unwrap();

    let opts = resvg::usvg::Options::default();
    let svg = resvg::usvg::Tree::from_str(
        input.as_str(),
        &opts.to_ref(),
    ).unwrap();

    for platform in args.platforms.unwrap().iter() {
        let sizes = size_map.get(platform);

        if !sizes.is_none() {
            let sz = sizes.unwrap()[0];

            let mut pixmap = tiny_skia::Pixmap::new(sz, sz).unwrap();
            resvg::render(
                &svg,
                resvg::usvg::FitTo::Size(sz, sz),
                tiny_skia::Transform::default(),
                 pixmap.as_mut(),
            ).unwrap();

            pixmap.save_png(std::path::Path::new("test.png"));
        }
    }
}