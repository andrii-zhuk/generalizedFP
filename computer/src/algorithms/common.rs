use std::{collections::VecDeque, f64::INFINITY};

use crate::types::DirectedGraph;

pub fn has_augmenting_path(graph: &DirectedGraph) -> f64 {
    fn bfs(graph: &DirectedGraph) -> f64 {
        let mut q = VecDeque::<(usize, f64)>::new();
        let mut used = vec![0; graph.nodes.len()];
        used[graph.source] = 1;
        q.push_back((graph.source, INFINITY));

        while q.len() > 0 {
            let (cur, flow) = q.pop_back().unwrap();
            if cur == graph.sink {
                return flow;
            }
            for edge_id in &graph.adj_lists[cur] {
                let edge = &graph.edges_list[*edge_id];
                let to = edge.to_id;
                if used[to] != 0 || edge.capacity < edge.flow {
                    continue;
                }
                used[to] = 1;
                q.push_back((to, flow.min(edge.capacity - edge.flow)));
            }
        }
        return 0.0;
    }
    return bfs(graph);
}
