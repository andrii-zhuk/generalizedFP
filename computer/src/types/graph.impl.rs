use std::collections::{HashMap, HashSet};

use crate::algorithms::EPSILON;

use crate::types::{DirectedGraph, Edge, Node, UncompressedGraph};

/// Sets value of Node.reachable_from_source = true <=> vertex was ever reachable form
/// source by edges of non-0 capacity.
/// ### Complexity
/// O(n + m)
fn set_reachable_from_source(graph: &mut DirectedGraph) {
    let mut status: Vec<bool> = vec![false; graph.n()];
    fn dfs(node_id: usize, graph: &DirectedGraph, status: &mut Vec<bool>) {
        status[node_id] = true;
        for &edge_id in &graph.adj_lists[node_id] {
            let edge = &graph.edges_list[edge_id];
            if edge.capacity < EPSILON || status[edge.to_id] == true {
                continue;
            }
            dfs(edge.to_id, graph, status);
        }
    }
    dfs(graph.source, graph, &mut status);
    for (node_id, &was_reached) in status.iter().enumerate() {
        graph.nodes[node_id].reachable_from_source = was_reached;
    }
}

impl DirectedGraph {
    #[inline]
    pub fn reachable_from_source(&self, node_id: usize) -> bool {
        self.nodes[node_id].reachable_from_source
    }
    #[inline]
    pub fn n(&self) -> usize {
        self.nodes.len()
    }
    pub fn add_edge(&mut self, from_id: usize, to_id: usize, capacity: f64, amplification: f64) {
        if from_id.max(to_id) >= self.nodes.len() {
            panic!("Failed to add edge between non-existent nodes.");
        }
        self.adj_lists[from_id].push(self.edges_list.len());
        self.edges_list.push(Edge {
            from_id,
            to_id,
            capacity,
            flow: 0.0,
            amplification,
        });
        self.reverse_edge_ids.push(self.edges_list.len());

        self.adj_lists[to_id].push(self.edges_list.len());
        self.edges_list.push(Edge {
            from_id: to_id,
            to_id: from_id,
            capacity: 0.0,
            flow: 0.0,
            amplification: 1.0 / amplification,
        });
        self.reverse_edge_ids.push(self.edges_list.len() - 2);
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
            .enumerate()
            .map(|(id, label)| Node {
                id,
                label,
                excess: 0.0,
                reachable_from_source: false,
            })
            .collect();

        let mut translator = HashMap::<String, usize>::new();
        nodes.iter().enumerate().for_each(|(id, node)| {
            translator.insert(node.label.clone(), id);
        });

        let sink = *translator.get(&sink).unwrap();
        let source = *translator.get(&source).unwrap();

        let mut graph = DirectedGraph {
            adj_lists: vec![vec![]; nodes.len()],
            edges_list: vec![],
            reverse_edge_ids: vec![],
            nodes,
            sink,
            source,
        };
        for edge in input.edges_list {
            let from_id = *translator.get(&edge.from).unwrap();
            let to_id = *translator.get(&edge.to).unwrap();
            if to_id == source || from_id == sink {
                continue;
            }
            graph.add_edge(from_id, to_id, edge.capacity, edge.amplification);
        }
        set_reachable_from_source(&mut graph);
        graph
    }
}
