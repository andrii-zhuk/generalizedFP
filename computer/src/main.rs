use computer::{types::{DirectedGraph, Node}, converters::read_from_file};

fn main() {
    println!("Hello, world123!");
    let _cur = DirectedGraph {
        adj_lists: vec![vec![1, 2, 3], vec![4, 7]],
        nodes: vec![Node{label: String::from("asdasd")}],
    };
    _cur.temp();
    println!("{:?}", _cur);
    read_from_file(&String::from("src/lib.rs"));
}
