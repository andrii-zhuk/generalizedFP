use crate::types;
#[derive(Debug)]
pub struct DirectedGraph {
    pub adj_lists: Vec<Vec<usize>>,
    pub nodes: Vec<types::node::Node>,
}


#[path = "graph.impl.rs"]
mod implementation;
