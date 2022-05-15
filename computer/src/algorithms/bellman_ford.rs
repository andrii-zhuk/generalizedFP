use crate::types::{DirectedGraph, Edge};

use super::EPSILON;

pub enum Mode {
    Min,
    Max,
}

pub fn bellman_ford(
    graph: &DirectedGraph,
    edge_value: fn(edge: &Edge) -> Option<f64>,
    mode: Mode,
    start_id: usize,
    reverse: bool,
) -> (Vec<Option<f64>>, Vec<Option<usize>>, Option<Vec<usize>>) {
    let compare = match &mode {
        Mode::Min => |x: f64, y: f64| -> bool { x - y < -EPSILON },
        Mode::Max => |x: f64, y: f64| -> bool { x - y > EPSILON },
    };
    let current_node = if reverse == true {
        |edge: &Edge| edge.to_id
    } else {
        |edge: &Edge| edge.from_id
    };
    let next_node = if reverse == true {
        |edge: &Edge| edge.from_id
    } else {
        |edge: &Edge| edge.to_id
    };

    let mut parents: Vec<Option<usize>> = vec![None; graph.n()];
    let mut dist: Vec<Option<f64>> = vec![None; graph.n()];
    dist[start_id] = Some(0.0);
    for i in 0..graph.n() {
        for (edge_id, edge) in graph.edges_list.iter().enumerate() {
            if dist[current_node(edge)] == None
                || !graph.reachable_from_source(current_node(edge))
                || !graph.reachable_from_source(next_node(edge))
            {
                continue;
            }
            let value = edge_value(edge);
            let value = match &value {
                None => continue,
                Some(value) => value,
            };
            if dist[next_node(edge)] == None
                || compare(
                    dist[current_node(edge)].unwrap() + value,
                    dist[next_node(edge)].unwrap(),
                )
            {
                if i == graph.n() - 1 {
                    let mut cycle: Vec<usize> = vec![];
                    let mut node_id = current_node(edge);
                    cycle.push(node_id);
                    while (cycle.len() == 1 || node_id != cycle[0]) && cycle.len() <= 2 * graph.n()
                    {
                        let edge_id = parents[node_id]
                            .expect("Negative cycle retrieving error: Undefined parent.");
                        cycle.push(edge_id);
                        node_id = current_node(&graph.edges_list[edge_id]);
                        cycle.push(node_id);
                    }

                    cycle.reverse();
                    while cycle.len() > 1 && cycle.first().unwrap() != cycle.last().unwrap() {
                        cycle.pop();
                        cycle.pop();
                    }
                    if cycle.len() == 1 {
                        panic!(
                            "Negative cycle retrieving error: possible bug in bellman-algorithm."
                        );
                    }
                    if reverse {
                        cycle.reverse();
                    }
                    return (dist, parents, Some(cycle));
                }
                dist[next_node(edge)] = Some(dist[current_node(edge)].unwrap() + value);
                parents[next_node(edge)] = Some(edge_id);
            }
        }
    }

    return (dist, parents, None);
}
