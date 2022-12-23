use std::path::PathBuf;

use visioncortex::{Color, ColorImage, CompoundPath, PointF64, PathSimplifyMode};
use visioncortex::color_clusters::{Runner, RunnerConfig, KeyingAction, HIERARCHICAL_MAX};

type Dimension = (usize, usize);

struct SvgPath {
    path: CompoundPath,
    color: Color,
}

pub fn get_vectorized_image(p: PathBuf) -> String {
    let img = image::open(p).unwrap();

    let data = img.to_rgba8().as_raw().to_vec();
    let width = img.width() as usize;
    let height = img.height() as usize;

    let runner = Runner::new(RunnerConfig {
        diagonal: false,
        hierarchical: HIERARCHICAL_MAX,
        batch_size: 25600,
        good_min_area: 100,
        good_max_area: (width * height),
        is_same_color_a: 0,
        is_same_color_b: 1,
        deepen_diff: 48,
        hollow_neighbours: 1,
        key_color: Color::default(),
        keying_action: KeyingAction::Discard,
    }, ColorImage{pixels: data, width: width, height: height});

    let clusters = runner.run();
    let view = clusters.view();

    let mut paths: Vec<SvgPath> = vec![];

    for &cluster_index in view.clusters_output.iter().rev() {
        let cluster = view.get_cluster(cluster_index);
        
        let p = cluster.to_compound_path(
            &view,
            false,
            PathSimplifyMode::Polygon,
            180.0,
            4.0,
            16,
            30.0,
        );

        paths.push(SvgPath{path: p, color: cluster.residue_color()});
    }

    get_svg_string(paths, (width, height))
}

fn get_svg_string(
    paths: Vec<SvgPath>,
    dimension: Dimension,
) -> String {
    let mut lines: Vec<String> = vec![
        r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string(),
        format!(
            r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
            dimension.0,
            dimension.1,
        )
    ];

    for p in paths.iter() {
        let (d, offset) = p.path.to_svg_string(
            true,
            PointF64::default(),
            Some(8),
        );

        let path_str = format!(
            "<path d=\"{}\" fill=\"{}\" transform=\"translate({},{})\"/>",
            d,
            p.color.to_hex_string(),
            offset.x,
            offset.y,
        );

        lines.push(path_str);
    }

    lines.push("</svg>".to_string());

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use resvg::usvg::{Tree, Options};

    #[test]
    fn test_get_svg_string() {
        let paths: Vec<SvgPath> = vec![
            SvgPath{path: CompoundPath::new(), color: Color { r: 1, g: 2, b: 3, a: 30 } },
            SvgPath{path: CompoundPath::new(), color: Color { r: 4, g: 5, b: 6, a: 25 } },
            SvgPath{path: CompoundPath::new(), color: Color { r: 1, g: 2, b: 3, a: 50 } },
        ];
        let dimension = (64, 64);

        let svg_string = get_svg_string(paths, dimension);

        let svg = Tree::from_str(
            &svg_string.as_str(),
            &Options::default().to_ref(),
        );

        assert!(svg.is_ok());
    }
}
