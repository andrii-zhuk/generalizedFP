use std::f64::{consts::E, EPSILON};

use crate::types::{DirectedGraph, Edge};

use super::bellman_ford;

pub fn get_fat_path(graph: &DirectedGraph) -> Option<Vec<usize>> {
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
    for node_id in 0..graph.nodes.len() {
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

    println!("Propagating fat path from {}", node);
    while node != graph.sink {
        path.push(node);
        let edge_id = match parent[node] {
            None => panic!("Bellman-Ford returned invalid path."),
            Some(value) => value,
        };
        path.push(edge_id);
        node = graph.edges_list[edge_id].to_id;
        if path.len() > 2 * graph.nodes.len() {
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

pub fn get_fat_path_node_ids(graph: &DirectedGraph) -> Option<Vec<usize>> {
    let path = get_fat_path(graph);
    if path == None {
        return None;
    }
    let path = path.unwrap();
    let nodes: Vec<usize> = path.iter().step_by(2).map(|&x| x).collect();

    return Some(nodes);
}

pub fn get_fat_path_edge_ids(graph: &DirectedGraph) -> Option<Vec<usize>> {
    let path = get_fat_path(graph);
    if path == None {
        return None;
    }
    let path = path.unwrap();
    let edges = path.iter().skip(1).step_by(2).map(|&x| x).collect();
    return Some(edges);
}
