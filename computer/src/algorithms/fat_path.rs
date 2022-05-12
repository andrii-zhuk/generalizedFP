use crate::{
    algorithms::{get_augmenting_path, get_path_edge_ids, get_path_node_ids},
    types::{AlgorithmResult, DirectedGraph},
};

use super::{cancel_cycles, has_augmenting_path, propagate_path};

pub fn find_flow(graph: &mut DirectedGraph, algorithm_result: &mut AlgorithmResult) -> f64 {
    let mut result = 0.0;
    let mut step = 0;
    while has_augmenting_path(&graph, algorithm_result) != None {
        step += 1;
        println!("Step #{}", step);
        cancel_cycles(graph);
        let augmenting_path = get_augmenting_path(graph);
        let augmenting_path_nodes = get_path_node_ids(augmenting_path.clone());
        let augmenting_path_edges = get_path_edge_ids(augmenting_path);
        println!(
            "Fat path found: {:?}",
            augmenting_path_nodes.unwrap_or(vec![])
        );
        let flow = propagate_path(graph, augmenting_path_edges);
        result += flow.unwrap_or(0.0);
        if flow == None {
            println!("No flow propagated");
        } else {
            println!("Propagated {}", flow.unwrap());
        }
    }
    result
}
