use super::EPSILON;
use std::f64::consts::E;

use crate::{
    algorithms::{get_path_edge_ids, get_path_node_ids, propagate_cycle},
    types::{AlgorithmResult, DirectedGraph, Edge},
};

use super::bellman_ford;

/// Returns true if graph contains flow-generating cycles.
/// For speed-up purpose, we run it once in `n` steps.
/// ### Complexity
/// O(n*m)
fn has_cycles(graph: &DirectedGraph, step_counter: &mut usize) -> bool {
    if *step_counter % graph.n() == 0 {
        *step_counter += 1;
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

/// Calculates potential cost of an edge: `-ln(amplification) - potential[from_id] + potential[to_id]`.
///
/// Returns `None` if an edge is empty or potential cost is negative.
///
/// Returns `Option` otherwise.
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

/// Indicates and removes from `graph` all cycles that have negative
/// potential cost.
///
/// ### Complexity
/// O(n + m)
fn remove_negative_cycles(
    graph: &mut DirectedGraph,
    potentials: &Vec<f64>,
    algorithm_result: &mut AlgorithmResult,
) -> Option<f64> {
    let mut status = vec![0; graph.n()];
    let mut cycle: Vec<usize> = Vec::<usize>::new();
    cycle.reserve(graph.n());
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
    let mut cumulative_excess = 0.0;

    for node_id in 0..(graph.n()) {
        if !graph.nodes[node_id].reachable_from_source {
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
        let excess = propagate_cycle(graph, cycle_edges, algorithm_result).unwrap_or(0.0);
        cumulative_excess += excess;
        cycle.clear();

        println!("Which created excess {} in {}.", excess, node_id);
    }
    if cumulative_excess < EPSILON {
        return None;
    }
    Some(cumulative_excess)
}

/// Recalculates values of potentials in order to maintain following logic:
///
/// For all non empty edges that are connected with source and have
/// non-negative potential cost should be not less than `-barrier`:
///
/// ### Complexity
/// O(n + m)
fn recalculate_potentials(graph: &DirectedGraph, potentials: &mut Vec<f64>, barrier: f64) {
    let mut stack: Vec<usize> = vec![];
    let mut status = vec![0; graph.n()];

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

    for node_id in (0..graph.n()).rev() {
        if status[node_id] != 0 {
            continue;
        }
        dfs(node_id, graph, &mut status, &potentials, &mut stack)
    }

    for (topological_order, node_id) in stack.iter().rev().enumerate() {
        potentials[*node_id] += (topological_order as f64) * barrier / (graph.n() as f64);
    }
}

/// Checks if potentials keep following logic:
///
/// For all non empty edges that are connected with source and have
/// non-negative potential cost should be not less than `-barrier`:
///
/// ### Panics
/// If at least one edge don't follow this logic.
///
/// ### Complexity
/// O(m)
fn validate_potentials(graph: &DirectedGraph, potentials: &Vec<f64>, barrier: f64) {
    for edge in &graph.edges_list {
        // If an edge is not connected to the source, then we will not cancel negative cycles
        // since it do not affect a solution. Therefore we do not need to maintain this rule
        if !graph.reachable_from_source(edge.from_id) || !graph.reachable_from_source(edge.to_id) {
            continue;
        }
        if let Some(value) = edge_value(edge, potentials) {
            if value + barrier < -EPSILON {
                panic!(
                    "Potentials calculated wrong way:\n{:?}\nWith it's potential value < -barrier: {} < {}",
                    edge, value, -barrier
                );
            }
        }
    }
}

pub fn cancel_cycles(graph: &mut DirectedGraph, algorithm_result: &mut AlgorithmResult) {
    let mut counter = 0;
    let mut potentials = vec![0.0; graph.n()];
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
        barrier = Some(barrier.unwrap_or(-cost).max(-cost));
    }
    if let Some(mut barrier) = barrier {
        while has_cycles(graph, &mut counter) {
            remove_negative_cycles(graph, &potentials, algorithm_result);
            println!("cancel_cycles stage {} finished", counter);
            recalculate_potentials(graph, &mut potentials, barrier);
            barrier *= 1.0 - 1.0 / (graph.n() as f64);
            validate_potentials(&graph, &potentials, barrier);
            if counter > 2000 {
                panic!("too many stages of cycle procession");
            }
        }
    }
    algorithm_result.push_if_cycles_not_found();
}
