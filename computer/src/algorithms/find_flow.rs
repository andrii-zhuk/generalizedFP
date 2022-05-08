use crate::{
    algorithms::{get_fat_path, get_path_edge_ids, get_path_node_ids},
    types::DirectedGraph,
};

use super::{has_augmenting_path, process_cycles, propagate_path};

pub fn find_flow(graph: &mut DirectedGraph) -> f64 {
    let mut result = 0.0;
    let mut step = 0;
    while has_augmenting_path(&graph) != None {
        step += 1;
        println!("Step #{}", step);
        process_cycles(graph);
        let fat_path = get_fat_path(graph);
        let fat_path_nodes = get_path_node_ids(fat_path.clone());
        let fat_path_edges = get_path_edge_ids(fat_path);
        println!("Fat path found: {:?}", fat_path_nodes.unwrap_or(vec![]));
        let flow = propagate_path(graph, fat_path_edges);
        result += flow.unwrap_or(0.0);
        if flow == None {
            println!("No flow propagated");
        } else {
            println!("Propagated {}", flow.unwrap());
        }
    }
    result
}
