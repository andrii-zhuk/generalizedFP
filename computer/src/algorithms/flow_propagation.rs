use super::EPSILON;

use crate::types::{AlgorithmResult, DirectedGraph};

fn get_start_of_path(graph: &DirectedGraph, path: &Vec<usize>) -> usize {
    let edge_id = *path.first().unwrap();
    let edge = &graph.edges_list[edge_id];
    return edge.from_id;
}

pub fn propagate_path(
    graph: &mut DirectedGraph,
    path: Option<Vec<usize>>,
    algorithm_result: &mut AlgorithmResult,
) -> Option<f64> {
    if path == None {
        return None;
    }
    let mut flow: Option<f64> = None;
    let path = path.unwrap();
    let start_node_id = get_start_of_path(&graph, &path);
    let start_node = &graph.nodes[start_node_id];
    if start_node_id != graph.source && start_node.excess > EPSILON {
        flow = Some(start_node.excess);
    }
    for &edge_id in path.iter() {
        let edge = &graph.edges_list[edge_id];
        let available = edge.capacity - edge.flow;
        if available < EPSILON {
            return None;
        }
        flow = match flow {
            None => Some(available * edge.amplification),
            Some(value) => Some(value.min(available) * edge.amplification),
        };
    }
    let mut flow = match flow {
        None => return None,
        Some(value) => value,
    };
    let flow_in_destination = flow;
    let mut changed_edges: Vec<(usize, f64)> = vec![];
    for &edge_id in path.iter().rev() {
        let reverse_edge = graph.reverse_edge_ids[edge_id];
        let reverse_edge = &mut graph.edges_list[reverse_edge];

        reverse_edge.flow -= flow;
        flow *= reverse_edge.amplification;

        let edge = &mut graph.edges_list[edge_id];
        edge.flow += flow;
        changed_edges.push((edge_id, flow));
    }
    if start_node_id != graph.source && start_node.excess > EPSILON {
        graph.nodes[start_node_id].excess -= flow;
    }
    algorithm_result.push_find_path(
        start_node_id,
        flow,
        graph.sink,
        flow_in_destination,
        changed_edges,
    );
    return Some(flow_in_destination);
}

pub fn propagate_cycle(
    graph: &mut DirectedGraph,
    cycle: Option<Vec<usize>>,
    algorithm_result: &mut AlgorithmResult,
) -> Option<f64> {
    if cycle == None {
        return None;
    }
    let cycle = cycle.unwrap();
    let mut flow: Option<f64> = None;
    for &edge_id in cycle.iter() {
        let edge = &graph.edges_list[edge_id];
        let available = edge.capacity - edge.flow;
        if available < EPSILON {
            return None;
        }
        flow = match flow {
            None => Some(available * edge.amplification),
            Some(value) => Some(value.min(available) * edge.amplification),
        };
    }
    let mut flow = match flow {
        None => return None,
        Some(value) => value,
    };
    let flow_in_destination = flow;

    let &cycle_start = cycle.first().unwrap();
    let cycle_start = graph.edges_list[cycle_start].from_id;

    graph.nodes[cycle_start].excess += flow_in_destination;

    let mut changed_edges: Vec<(usize, f64)> = vec![];
    for &edge_id in cycle.iter().rev() {
        let reverse_edge = graph.reverse_edge_ids[edge_id];
        let reverse_edge = &mut graph.edges_list[reverse_edge];

        reverse_edge.flow -= flow;
        flow *= reverse_edge.amplification;

        let edge = &mut graph.edges_list[edge_id];
        edge.flow += flow;
        changed_edges.push((edge_id, flow));
    }

    algorithm_result.push_find_cycles(changed_edges);
    algorithm_result.push_cancel_cycles(cycle_start, flow_in_destination);
    return Some(flow_in_destination);
}
