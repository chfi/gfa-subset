use gfa::gfa::{Link, Path, Segment, GFA};
use gfa::parser::parse_gfa;
use gfa::writer::{gfa_string, write_gfa};

use std::env;
use std::path::PathBuf;

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(args[0].clone());
    if let Some(gfa) = parse_gfa(&path) {
        // Set of the path names we're interested in
        let path_names: HashSet<_> = args.into_iter().skip(1).collect();

        // Filter out the paths in the GFA we don't want
        let paths: Vec<_> = gfa
            .paths
            .into_iter()
            .filter(|p| path_names.contains(&p.path_name))
            .collect();

        // Set of the segments in the paths we're keeping
        let mut segment_names: HashSet<&str> = HashSet::new();

        paths.iter().for_each(|path| {
            path.segment_names.iter().for_each(|(seg, _)| {
                segment_names.insert(seg);
            });
        });

        // Filter out the segments in the GFA we don't want
        let segments: Vec<_> = gfa
            .segments
            .into_iter()
            .filter(|s| segment_names.contains(s.name.as_str()))
            .collect();

        // Filter out the links in the GFA we don't want
        let links: Vec<_> = gfa
            .links
            .into_iter()
            .filter(|l| {
                segment_names.contains(l.from_segment.as_str())
                    && segment_names.contains(l.to_segment.as_str())
            })
            .collect();

        let new_gfa = GFA {
            segments,
            links,
            paths,
            containments: Vec::new(),
        };

        println!("{}", gfa_string(&new_gfa));

    // let find_path = |pn: &str| gfa.paths.iter().find(|
    // let paths = path_names.iter().map(|pn| {
    //     gfa.paths.iter().filter(
    // });
    } else {
        panic!("Could not read provided .gfa file");
    }
}
