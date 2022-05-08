pub const EPSILON: f64 = std::f64::EPSILON;

pub fn get_path_node_ids(path: Option<Vec<usize>>) -> Option<Vec<usize>> {
    if let Some(path) = path {
        let nodes: Vec<usize> = path.iter().step_by(2).map(|&x| x).collect();
        return Some(nodes);
    }
    return None;
}

pub fn get_path_edge_ids(path: Option<Vec<usize>>) -> Option<Vec<usize>> {
    if let Some(path) = path {
        let edges = path.iter().skip(1).step_by(2).map(|&x| x).collect();
        return Some(edges);
    }
    return None;
}
