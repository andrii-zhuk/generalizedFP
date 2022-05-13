use serde::Serialize;

use crate::types;
#[derive(Serialize, Debug, Clone)]
pub struct DirectedGraph {
    pub source: usize,
    pub sink: usize,
    pub adj_lists: Vec<Vec<usize>>,
    pub edges_list: Vec<types::Edge>,
    pub reverse_edge_ids: Vec<usize>,
    pub nodes: Vec<types::Node>,
}

#[derive(Serialize, Debug)]
pub struct UncompressedGraph {
    pub source: String,
    pub sink: String,
    pub edges_list: Vec<types::UncompressedEdge>,
}

#[path = "graph.impl.rs"]
mod implementation;
