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

pub fn generate_image_data(
    input: String,
    platforms: Vec<Platform>,
) -> Vec<OutputFile> {
    let size_map: HashMap<Platform, Vec<Target> > = HashMap::from([
        (Platform::Web, vec![Target{ size: 32, name: "favicon.ico".to_string() }]),
        (Platform::Modern, vec![]),
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

            let png_data = pixmap.encode_png().unwrap();
            results.push(OutputFile { name: output.name.clone(), data: png_data });
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

    const SVG: &str = "<svg width=\"72\" height=\"72\" viewBox=\"0 0 72 72\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\"><rect width=\"72\" height=\"72\" rx=\"10\" fill=\"#2C3333\"/></svg>";

    #[test]
    fn test_svg_only() {
        let platforms = vec![Platform::Modern];
        let result = generate_image_data(SVG.to_string(), platforms);

        assert_eq!(result.len(), 1);
        
        let has_svg = result.iter().find(|x| x.name == "icon.svg");
        let has_else = result.iter().find(|x| x.name != "icon.svg");

        assert_eq!(has_svg.is_some(), true);
        assert_eq!(has_else.is_some(), false);
    }

    #[test]
    fn test_with_rasterize() {
        let platforms = vec![Platform::Modern, Platform::Web, Platform::Apple];
        let result = generate_image_data(SVG.to_string(), platforms);

        assert_eq!(result.len(), 3);

        let has_svg = result.iter().find(|x| x.name == "icon.svg");
        let has_ico = result.iter().find(|x| x.name == "favicon.ico");
        let has_apple = result.iter().find(|x| x.name == "apple_touch_icon.png");

        assert_eq!(has_svg.is_some(), true);
        assert_eq!(has_ico.is_some(), true);
        assert_eq!(has_apple.is_some(), true);
    }

    #[test]
    fn test_with_android() {
        let platforms = vec![Platform::Android];
        let result = generate_image_data(SVG.to_string(), platforms);

        assert_eq!(result.len(), 2);

        let has_mq = result.iter().find(|x| x.name == "192.png");
        let has_hq = result.iter().find(|x| x.name == "512.png");

        assert_eq!(has_mq.is_some(), true);
        assert_eq!(has_hq.is_some(), true);
    }
}