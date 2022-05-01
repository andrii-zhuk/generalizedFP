use crate::types::{DirectedGraph, Edge};

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
        Mode::Min => |x: f64, y: f64| x < y,
        Mode::Max => |x: f64, y: f64| x > y,
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

    let mut parents: Vec<Option<usize>> = vec![None; graph.nodes.len()];
    let mut dist: Vec<Option<f64>> = vec![None; graph.nodes.len()];
    dist[start_id] = Some(0.0);
    for i in 0..graph.nodes.len() {
        for (edge_id, edge) in graph.edges_list.iter().enumerate() {
            if dist[current_node(edge)] == None {
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
                if i == graph.nodes.len() - 1 {
                    let mut cycle: Vec<usize> = vec![];
                    let mut node_id = edge.to_id;
                    cycle.push(edge.to_id);
                    while node_id != edge.from_id {
                        if let Some(edge_id) = parents[node_id] {
                            cycle.push(edge_id);
                            cycle.push(graph.edges_list[edge_id].to_id);
                            node_id = graph.edges_list[edge_id].to_id;
                        } else {
                            panic!("Negative cycle retrieving error: Undefined parent.");
                        }
                        if cycle.len() > 2 * graph.nodes.len() {
                            panic!("Negative cycle retrieving error: Cycle length can not exceed number of nodes in graph.");
                        }
                    }
                    cycle.push(edge_id);
                    cycle.push(edge.to_id);
                    return (dist, parents, Some(cycle));
                }
                dist[next_node(edge)] = Some(dist[current_node(edge)].unwrap() + value);
                parents[next_node(edge)] = Some(edge_id);
            }
        }
    }

    return (dist, parents, None);
}
