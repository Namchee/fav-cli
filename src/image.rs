use std::collections::HashMap;
use std::ffi::OsStr;

use std::fs;

use crate::args::{Args, Platform};

pub fn generate_image(args: Args) {
    let size_map: HashMap<Platform, Vec<u16> > = HashMap::from([
        (Platform::Web, vec![32]),
        (Platform::Android, vec![192, 512]),
        (Platform::Apple, vec![180]),
    ]);
    
    let ext = args.source.extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let input = fs::read_to_string(args.source).unwrap();

    let opts = usvg::Options::default();
    let svg = usvg::Tree::from_str(
        input.as_str(),
        &opts.to_ref(),
    ).unwrap();

    for platform in args.platforms.unwrap().iter() {
        let sizes = size_map.get(platform);

        if !sizes.is_none() {
            let sz = sizes.unwrap()[0] as f64;
            let size = resvg::Size::new(sz, sz);
        }
    }
}