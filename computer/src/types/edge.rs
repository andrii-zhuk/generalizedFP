#[derive(Debug, Clone)]
pub struct Edge {
    pub from_id: usize,
    pub to_id: usize,
    pub flow: f64,
    pub capacity: f64,
    pub amplification: f64,
}

#[derive(Debug)]
pub struct UncompressedEdge {
    pub to: String,
    pub from: String,
    pub capacity: f64,
    pub amplification: f64,
}
