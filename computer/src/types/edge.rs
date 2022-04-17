#[derive(Debug, Clone)]
pub struct Edge {
    pub to_id: usize,
    pub from_id: usize,
    pub capacity: f64,
    pub flow: f64,
    pub amplification: f64,
}

#[derive(Debug)]
pub struct UncompressedEdge {
    pub to: String,
    pub from: String,
    pub capacity: f64,
    pub amplification: f64,
}
