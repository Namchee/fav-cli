use crate::args::Args;

pub fn generate_image(args: Args) {
    let img = image::open(args.source).unwrap();
}