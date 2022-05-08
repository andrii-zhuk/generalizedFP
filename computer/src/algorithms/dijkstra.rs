// TODO:
// Either rewrite with use of potentials or completely delete.
// Yield parent edge_ids as well.

use super::EPSILON;
use std::f64::INFINITY;
use std::{cmp::Ordering, collections::BinaryHeap};

use crate::types::{DirectedGraph, Edge};

pub enum DijkstraMode {
    Min,
    Max,
}
#[derive(Copy, Clone)]
struct State {
    cost: f64,
    position: usize,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.cost - other.cost).abs() < EPSILON && self.position == other.position
    }
}
impl Eq for State {
    fn assert_receiver_is_total_eq(&self) {}
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap()
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(
    graph: &DirectedGraph,
    edge_value: fn(edge: &Edge) -> Option<f64>,
    mode: DijkstraMode,
    start_id: usize,
) -> Vec<Option<f64>> {
    let inf = match &mode {
        DijkstraMode::Min => INFINITY,
        DijkstraMode::Max => -INFINITY,
    };
    let multiplier = match &mode {
        DijkstraMode::Min => 1.0,
        DijkstraMode::Max => -1.0,
    };

    let mut dist = vec![inf; graph.n()];
    let mut heap = BinaryHeap::new();

    dist[start_id] = 0.0;
    heap.push(State {
        cost: 0.0,
        position: start_id,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }
        for edge in &graph.adj_lists[position] {
            let edge = &graph.edges_list[*edge];
            let value = edge_value(edge);
            let value = match value {
                Some(value) => multiplier * value,
                None => continue,
            };
            let next = State {
                cost: cost + value,
                position: edge.to_id,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    let mut result = vec![Some(0.0); graph.n()];
    for (i, value) in dist.iter().enumerate() {
        result[i] = if *value == inf {
            None
        } else {
            Some(multiplier * value)
        };
    }
    return result;
}
