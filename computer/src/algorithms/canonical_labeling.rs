use std::f64::{consts::E, EPSILON};

use crate::types::{DirectedGraph, Edge};

use super::reverse_bellman_ford;

pub fn find_canonical_labeling(graph: &DirectedGraph) -> Vec<Option<f64>> {
    let (mut dist, _) = reverse_bellman_ford(
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
    );

    for i in 0..dist.len() {
        dist[i] = match dist[i] {
            Some(value) => Some((1.0 / value).exp()),
            None => None,
        };
    }

    return dist;
}
