use std::f64::{consts::E, EPSILON};

use crate::types::{DirectedGraph, Edge};

use super::bellman_ford;

fn get_fat_path(graph: &DirectedGraph) -> Option<Vec<usize>> {
    let (_, parents, cycle) = bellman_ford(
        graph,
        |edge: &Edge| {
            if (edge.flow - edge.capacity).abs() < EPSILON {
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
        return None;
    }
    if parents[graph.source] == None {
        return None;
    }
    let mut path: Vec<usize> = vec![];

    let mut node: usize = graph.source;
    while node != graph.sink {
        path.push(node);
        let edge_id = match parents[node] {
            None => panic!("Bellman-Ford returned invalid path."),
            Some(value) => value,
        };
        path.push(edge_id);
        node = graph.edges_list[edge_id].to_id;
        if path.len() > 2 * graph.nodes.len() {
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
