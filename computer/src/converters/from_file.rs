use std::fs;

use crate::types::DirectedGraph;



pub fn read_from_file(path: &String) -> DirectedGraph {
    println!("In file {}", path);

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    DirectedGraph { adj_lists: vec![], nodes: vec![] }

}