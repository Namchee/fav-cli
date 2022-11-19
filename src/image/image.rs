use crate::args::Args;

pub fn generate_image(args: Args) {
    let source = img::open(args.source).unwrap();
}