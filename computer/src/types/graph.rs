use serde::Serialize;

use crate::types;
#[derive(Serialize, Debug)]
pub struct DirectedGraph {
    pub adj_lists: Vec<Vec<usize>>,
    pub edges_list: Vec<types::Edge>,
    pub reverse_edge_ids: Vec<usize>,
    pub nodes: Vec<types::Node>,
    pub sink: usize,
    pub source: usize,
}

#[derive(Serialize, Debug)]
pub struct UncompressedGraph {
    pub source: String,
    pub sink: String,
    pub edges_list: Vec<types::UncompressedEdge>,
}

#[path = "graph.impl.rs"]
mod implementation;
