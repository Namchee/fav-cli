use std::{collections::HashMap};

use crate::args::Platform;

struct Target {
    size: u32,
    name: String,
}

pub struct OutputFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn generate_image(
    input: String,
    platforms: Vec<Platform>,
) -> Vec<OutputFile> {
    let size_map: HashMap<Platform, Vec<Target> > = HashMap::from([
        (Platform::Web, vec![Target{ size: 32, name: "favicon.ico".to_string() }]),
        (Platform::Android, vec![Target{ size: 192, name: "192.png".to_string() }, Target{ size: 512, name: "512.png".to_string() }]),
        (Platform::Apple, vec![Target{ size: 180, name: "apple_touch_icon.png".to_string() }]),
    ]);

    let opts = resvg::usvg::Options::default();
    let svg = resvg::usvg::Tree::from_str(
        input.as_str(),
        &opts.to_ref(),
    ).unwrap();

    let mut results: Vec<OutputFile> = vec![];

    for platform in platforms.iter() {
        let sizes = size_map.get(platform).unwrap();

        for output in sizes.iter() {
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

            results.push(OutputFile { name: output.name.clone(), data: pixmap.data().to_vec() });
        }
    }

    if platforms.contains(&Platform::Modern) {
        results.push(
            OutputFile{ name: "icon.svg".to_string(), data: input.as_bytes().to_vec() },
        );
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

}