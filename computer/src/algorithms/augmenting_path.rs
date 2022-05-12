use super::EPSILON;
use std::{collections::VecDeque, f64::consts::E};

use crate::types::{AlgorithmResult, DirectedGraph, Edge};

use super::bellman_ford;

pub fn has_augmenting_path(
    graph: &DirectedGraph,
    algorithm_result: &mut AlgorithmResult,
) -> Option<f64> {
    fn bfs(graph: &DirectedGraph) -> f64 {
        let mut q = VecDeque::<(usize, Option<f64>)>::new();
        let mut used = vec![0; graph.n()];
        used[graph.source] = 1;
        q.push_back((graph.source, None));
        for i in 0..graph.n() {
            if i != graph.source && graph.nodes[i].excess > EPSILON {
                used[i] = 1;
                q.push_back((i, Some(graph.nodes[i].excess)));
            }
        }
        while let Some((cur, flow)) = q.pop_back() {
            if cur == graph.sink {
                return flow.unwrap();
            }
            for edge_id in &graph.adj_lists[cur] {
                let edge = &graph.edges_list[*edge_id];
                let to = edge.to_id;
                let available = edge.capacity - edge.flow;
                if used[to] != 0 || available < EPSILON || !graph.reachable_from_source(to) {
                    continue;
                }
                used[to] = 1;
                q.push_back((to, Some(flow.unwrap_or(available).min(available))));
            }
        }
        return 0.0;
    }
    let flow = bfs(graph);
    if flow < EPSILON {
        algorithm_result.push_has_augmenting_path(None);
        return None;
    }
    algorithm_result.push_has_augmenting_path(Some(flow));
    return Some(flow);
}

pub fn get_augmenting_path(graph: &DirectedGraph) -> Option<Vec<usize>> {
    let (dist, parent, cycle) = bellman_ford(
        graph,
        |edge: &Edge| {
            if edge.capacity - edge.flow < EPSILON {
                None
            } else {
                Some((edge.amplification).log(E))
            }
        },
        super::Mode::Max,
        graph.sink,
        true,
    );
    if cycle != None {
        panic!("Impossible to get fat path if graph has flow-generating cycles.");
    }
    let mut start_node: usize = graph.source;
    let mut best_dist = dist[start_node];
    for node_id in 0..graph.n() {
        if let Some(cur_dist) = dist[node_id] {
            if best_dist.unwrap_or(cur_dist - 1.0) < cur_dist
                && graph.nodes[node_id].excess > EPSILON
            {
                start_node = node_id;
                best_dist = Some(cur_dist);
            }
        }
    }
    let mut node = start_node;
    if parent[node] == None {
        return None;
    }
    let mut path: Vec<usize> = vec![];

    while node != graph.sink {
        path.push(node);
        let edge_id = parent[node].expect("Bellman-Ford returned invalid path.");
        path.push(edge_id);
        node = graph.edges_list[edge_id].to_id;
        if path.len() > 2 * graph.n() {
            for i in 0..parent.len() {
                if parent[i] == None {
                    println!("None");
                } else {
                    println!("{:#?}", graph.edges_list[parent[i].unwrap()]);
                }
            }
            println!("{:#?}", dist);
            panic!("Bellman-Ford returned invalid path. (cycled path)")
        }
    }
    path.push(node);

    return Some(path);
}
