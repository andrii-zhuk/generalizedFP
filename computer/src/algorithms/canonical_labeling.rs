use std::f64::{consts::E, EPSILON};

use crate::{
    algorithms::bellman_ford,
    types::{DirectedGraph, Edge},
};

pub fn find_canonical_labeling(graph: &DirectedGraph) -> Vec<Option<f64>> {
    let (mut dist, _, cycle) = bellman_ford(
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
        panic!("Bellman-Ford has found flow generating cycle.");
    }

    for i in 0..dist.len() {
        dist[i] = match dist[i] {
            Some(value) => Some((1.0 / value).exp()),
            None => None,
        };
    }
    return dist;
}
