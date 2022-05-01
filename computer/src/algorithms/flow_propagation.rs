use std::f64::EPSILON;

use crate::types::DirectedGraph;

pub fn propagate_path(graph: &mut DirectedGraph, path: Option<Vec<usize>>) -> Option<f64> {
    if path == None {
        return None;
    }
    let path = path.unwrap();
    let mut flow: Option<f64> = None;

    for &edge_id in path.iter() {
        let edge = &graph.edges_list[edge_id];
        let available = edge.capacity - edge.flow + graph.nodes[edge.to_id].excess;
        if available < EPSILON {
            return None;
        }
        flow = match flow {
            None => Some(available * edge.amplification + graph.nodes[edge.to_id].excess),
            Some(value) => {
                Some(value.min(available) * edge.amplification + graph.nodes[edge.to_id].excess)
            }
        };
    }
    let mut flow = match flow {
        None => return None,
        Some(value) => value,
    };
    for &edge_id in path.iter().rev() {
        let reverse_edge = graph.reverse_edge_ids[edge_id];
        let reverse_edge = &mut graph.edges_list[reverse_edge];
        if graph.nodes[reverse_edge.from_id].excess > EPSILON {
            let leave_in_node = flow.min(graph.nodes[reverse_edge.from_id].excess);
            graph.nodes[reverse_edge.from_id].excess -= leave_in_node;
            flow -= leave_in_node;
        }
        reverse_edge.flow -= flow;
        flow *= reverse_edge.amplification;

        let edge = &mut graph.edges_list[edge_id];
        edge.flow += flow;
    }
    return Some(flow);
}

pub fn propagate_cycle(graph: &mut DirectedGraph, cycle: Option<Vec<usize>>) -> Option<f64> {
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
    if let Some(&cycle_start) = cycle.first() {
        let cycle_start = graph.edges_list[cycle_start].from_id;
        graph.nodes[cycle_start].excess += flow;
    }
    for &edge_id in cycle.iter().rev() {
        let reverse_edge = graph.reverse_edge_ids[edge_id];
        let reverse_edge = &mut graph.edges_list[reverse_edge];

        reverse_edge.flow -= flow;
        flow *= reverse_edge.amplification;

        let edge = &mut graph.edges_list[edge_id];
        edge.flow += flow;
    }

    return Some(flow);
}
