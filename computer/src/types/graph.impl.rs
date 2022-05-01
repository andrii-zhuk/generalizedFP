use std::collections::{HashMap, HashSet};

use crate::types::{DirectedGraph, Edge, Node, UncompressedGraph};

impl DirectedGraph {
    pub fn temp(&self) {
        println!("durkaaaa {}", self.adj_lists.len());
        if self.nodes.len() != 0 {
            for node in &self.nodes {
                println!("node {}", node.label);
            }
        }
    }
}
impl From<UncompressedGraph> for DirectedGraph {
    fn from(input: UncompressedGraph) -> Self {
        let mut translator = HashSet::<String>::new();
        let sink = input.sink;
        let source = input.source;
        translator.insert(sink.clone());
        translator.insert(source.clone());
        for edge in &input.edges_list {
            translator.insert(edge.from.clone());
            translator.insert(edge.to.clone());
        }
        let mut nodes: Vec<String> = translator.into_iter().collect();
        nodes.sort();

        let nodes: Vec<Node> = nodes
            .into_iter()
            .map(|label| Node { label, excess: 0.0 })
            .collect();

        let mut translator = HashMap::<String, usize>::new();
        nodes.iter().enumerate().for_each(|(id, node)| {
            translator.insert(node.label.clone(), id);
        });

        let sink = *translator.get(&sink).unwrap();
        let source = *translator.get(&source).unwrap();

        let mut edges_list: Vec<Edge> = vec![];
        let mut reverse_edge_ids: Vec<usize> = vec![];
        let mut adj_lists: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
        for edge in input.edges_list {
            let from_id = *translator.get(&edge.from).unwrap();
            let to_id = *translator.get(&edge.to).unwrap();
            if to_id == source || from_id == sink {
                continue;
            }
            let capacity = edge.capacity;
            let amplification = edge.amplification;
            adj_lists[from_id].push(edges_list.len());
            edges_list.push(Edge {
                from_id,
                to_id,
                capacity,
                amplification,
                flow: 0.0,
            });
            reverse_edge_ids.push(edges_list.len());

            adj_lists[to_id].push(edges_list.len());
            edges_list.push(Edge {
                from_id: to_id,
                to_id: from_id,
                capacity: 0.0,
                amplification: 1.0 / amplification,
                flow: 0.0,
            });
            reverse_edge_ids.push(edges_list.len() - 2);
        }

        DirectedGraph {
            adj_lists,
            edges_list,
            nodes,
            sink,
            source,
            reverse_edge_ids,
        }
    }
}
