use std::{fs, str::FromStr};

use crate::types::{DirectedGraph, UncompressedEdge, UncompressedGraph};
use regex::{self};

fn parse_next<T: FromStr>(input: &mut Vec<&str>) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let result = input.pop().expect("Invalid file format");
    result.parse::<T>().expect("Invalid file format")
}

pub fn read_from_file(path: &String) -> DirectedGraph {
    let file = fs::read_to_string(path).expect("Error has been occured during reading from file.");
    read_from_string(file)
}

pub fn read_from_string(text: String) -> DirectedGraph {
    let re = regex::Regex::new(r"\n| ").unwrap();
    let mut entries: Vec<&str> = re
        .split(text.trim())
        .filter(|elem| elem.len() > 0)
        .collect();

    entries.reverse();
    let m: u64 = parse_next(&mut entries);
    let source: String = parse_next(&mut entries);
    let sink: String = parse_next(&mut entries);

    let mut edges_list: Vec<UncompressedEdge> = vec![];

    for _i in 0..m {
        let from: String = parse_next(&mut entries);
        let to: String = parse_next(&mut entries);

        let capacity: f64 = parse_next(&mut entries);
        let amplification: f64 = parse_next(&mut entries);

        edges_list.push(UncompressedEdge {
            from,
            to,
            capacity,
            amplification,
        });
    }

    let graph = UncompressedGraph {
        source,
        sink,
        edges_list,
    };

    let graph = DirectedGraph::from(graph);

    graph
}
