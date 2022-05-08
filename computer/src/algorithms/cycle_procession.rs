use std::f64::{consts::E, EPSILON};

use crate::{
    algorithms::{get_path_edge_ids, get_path_node_ids, propagate_cycle},
    types::{DirectedGraph, Edge},
};

use super::bellman_ford;

fn has_cycles(graph: &DirectedGraph, step_counter: &mut usize) -> bool {
    if *step_counter % graph.nodes.len() == 0 {
        let (_, _, cycle) = bellman_ford(
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
        if cycle == None {
            return false;
        }
        return true;
    }
    *step_counter += 1;
    return true;
}
// return vector of bool. result[i] == true <=> vertex is reachable form source
// by edges of non-0 capacity.
fn get_reachable_from_source(graph: &DirectedGraph) -> Vec<bool> {
    let mut result = vec![false; graph.nodes.len()];
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
    dfs(graph.source, graph, &mut result);
    return result;
}

fn edge_value(edge: &Edge, potentials: &Vec<f64>) -> Option<f64> {
    if edge.capacity - edge.flow < EPSILON {
        return None;
    } else {
        let value =
            -(edge.amplification).log(E) - potentials[edge.from_id] + potentials[edge.to_id];
        if value < -EPSILON {
            return Some(value);
        } else {
            return None;
        }
    }
}

fn remove_negative_cycles(
    graph: &mut DirectedGraph,
    potentials: &Vec<f64>,
    reachable_from_source: &Vec<bool>,
) -> Option<f64> {
    let mut status = vec![0; graph.nodes.len()];
    let mut cycle: Vec<usize> = Vec::<usize>::new();
    cycle.reserve(graph.nodes.len());
    fn dfs(
        node_id: usize,
        graph: &DirectedGraph,
        potentials: &Vec<f64>,
        status: &mut Vec<i32>,
        cycle: &mut Vec<usize>,
    ) {
        status[node_id] = 1;

        for &edge_id in &graph.adj_lists[node_id] {
            let edge = &graph.edges_list[edge_id];
            if edge_value(edge, potentials) == None || status[edge.to_id] == 2 {
                continue;
            }
            if status[edge.to_id] == 0 {
                dfs(edge.to_id, graph, potentials, status, cycle);
            }
            if status[edge.to_id] == 1 || cycle.len() > 0 {
                if cycle.len() == 0 || cycle[0] != cycle[cycle.len() - 1] {
                    cycle.push(edge.to_id);
                    cycle.push(edge_id);
                    if cycle[0] == edge.from_id {
                        cycle.push(edge.from_id);
                        cycle.reverse();
                    }
                }
                status[node_id] = 0;
                return;
            }
        }
        status[node_id] = 2;
    }

    for node_id in 0..(graph.nodes.len()) {
        if !reachable_from_source[node_id] {
            continue;
        }
        if status[node_id] == 2 {
            continue;
        }
        if status[node_id] == 1 {
            panic!("Unexpected behavior in dfs");
        }
        dfs(node_id, &graph, potentials, &mut status, &mut cycle);
        if cycle.len() == 0 {
            continue;
        }
        let cycle_nodes = get_path_node_ids(Some(cycle.clone())).unwrap_or(vec![]);
        println!("Found cycle from {}:\n{:?}", node_id, cycle_nodes);

        let cycle_edges = get_path_edge_ids(Some(cycle.clone()));
        let excess = propagate_cycle(graph, cycle_edges).unwrap_or(0.0);

        println!("Which created excess {} in {}.", excess, node_id);

        cycle.clear();
    }

    Some(0.0)
}

fn recalculate_potentials(graph: &DirectedGraph, potentials: &mut Vec<f64>, barrier: f64) {
    let mut stack: Vec<usize> = vec![];
    let mut status = vec![0; graph.nodes.len()];

    fn dfs(
        node_id: usize,
        graph: &DirectedGraph,
        status: &mut Vec<i32>,
        potentials: &Vec<f64>,
        stack: &mut Vec<usize>,
    ) {
        status[node_id] = 1;
        for &edge_id in &graph.adj_lists[node_id] {
            let edge = &graph.edges_list[edge_id];
            if edge_value(edge, &potentials) == None || status[edge.to_id] != 0 {
                continue;
            }
            dfs(edge.to_id, graph, status, potentials, stack);
        }
        stack.push(node_id);
    }

    for node_id in (0..graph.nodes.len()).rev() {
        if status[node_id] != 0 {
            continue;
        }
        dfs(node_id, graph, &mut status, &potentials, &mut stack)
    }

    for (topological_order, node_id) in stack.iter().rev().enumerate() {
        potentials[*node_id] += (topological_order as f64) * barrier / (graph.nodes.len() as f64);
    }
}

fn validate_potentials(graph: &DirectedGraph, potentials: &Vec<f64>, barrier: f64) {
    for edge in &graph.edges_list {
        if let Some(value) = edge_value(edge, potentials) {
            if value < -barrier {
                panic!(
                    "Potentials calculated wrong way:\n{:?}\nwith it's potential value < -barrier: {} < {}",
                    edge, value, -barrier
                );
            }
        }
    }
}

pub fn process_cycles(graph: &mut DirectedGraph) {
    let mut counter = 0;
    let mut potentials = vec![0.0; graph.nodes.len()];
    let reachable_from_source = get_reachable_from_source(&graph);
    let edge_value = |edge: &Edge| -> Option<f64> {
        if edge.capacity - edge.flow < EPSILON {
            None
        } else {
            Some(-(edge.amplification).log(E))
        }
    };

    let mut barrier: Option<f64> = None;
    for edge in &graph.edges_list {
        let cost = match edge_value(&edge) {
            None => continue,
            Some(value) => value,
        };
        barrier = match barrier {
            None => Some(-cost),
            Some(value) => Some(value.max(-cost)),
        };
    }
    let mut barrier = match barrier {
        None => return,
        Some(value) => value,
    };

    while has_cycles(graph, &mut counter) {
        remove_negative_cycles(graph, &potentials, &reachable_from_source);
        println!("process_cycles stage {} finished", counter);
        recalculate_potentials(graph, &mut potentials, barrier);
        validate_potentials(graph, &potentials, barrier);
        barrier *= ((graph.nodes.len() as f64) - 1.0) / (graph.nodes.len() as f64);
        if counter == 2000 {
            panic!("too many stages of cycle procession");
        }
    }
}
