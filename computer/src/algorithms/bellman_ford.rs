use crate::types::{DirectedGraph, Edge};

pub enum Mode {
    Min,
    Max,
}

pub fn reverse_bellman_ford(
    graph: &DirectedGraph,
    edge_value: fn(edge: &Edge) -> Option<f64>,
    mode: Mode,
    finish_id: usize,
) -> (Vec<Option<f64>>, Vec<Option<usize>>) {
    let compare = match &mode {
        Mode::Min => (|x: f64, y: f64| x < y),
        Mode::Max => (|x: f64, y: f64| x > y),
    };
    let mut parents: Vec<Option<usize>> = vec![None; graph.nodes.len()];
    let mut dist: Vec<Option<f64>> = vec![None; graph.nodes.len()];
    dist[finish_id] = Some(0.0);
    parents[finish_id] = Some(finish_id);

    for i in 0..graph.nodes.len() {
        for (edge_id, edge) in graph.edges_list.iter().enumerate() {
            if dist[edge.to_id] == None {
                continue;
            }
            let value = edge_value(edge);
            let value = match &value {
                None => continue,
                Some(value) => value,
            };
            if dist[edge.from_id] == None
                || compare(
                    dist[edge.to_id].unwrap() + value,
                    dist[edge.from_id].unwrap(),
                )
            {
                if i == graph.nodes.len() - 1 {
                    panic!("Bellman Ford has found a negative cycle. Critical error!");
                }
                dist[edge.from_id] = Some(dist[edge.to_id].unwrap() + value);
                parents[edge.from_id] = Some(edge_id);
            }
        }
    }

    return (dist, parents);
}

pub fn bellman_ford(
    graph: &DirectedGraph,
    edge_value: fn(edge: &Edge) -> Option<f64>,
    mode: Mode,
    start_id: usize,
) -> (Vec<Option<f64>>, Vec<Option<usize>>) {
    let compare = match &mode {
        Mode::Min => (|x: f64, y: f64| x < y),
        Mode::Max => (|x: f64, y: f64| y > x),
    };
    let mut parents: Vec<Option<usize>> = vec![None; graph.nodes.len()];
    let mut dist: Vec<Option<f64>> = vec![None; graph.nodes.len()];
    dist[start_id] = Some(0.0);

    for i in 0..graph.nodes.len() {
        for (edge_id, edge) in graph.edges_list.iter().enumerate() {
            if dist[edge.from_id] == None {
                continue;
            }
            let value = edge_value(edge);
            let value = match &value {
                None => continue,
                Some(value) => value,
            };
            if dist[edge.to_id] == None
                || compare(
                    dist[edge.from_id].unwrap() + value,
                    dist[edge.to_id].unwrap(),
                )
            {
                if i == graph.nodes.len() - 1 {
                    panic!("Bellman Ford has found a negative cycle. Critical error!");
                }
                dist[edge.to_id] = Some(dist[edge.from_id].unwrap() + value);
                parents[edge.to_id] = Some(edge_id);
            }
        }
    }

    return (dist, parents);
}
