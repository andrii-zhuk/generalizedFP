#[derive(Debug)]
pub struct Node {
    pub label: String,
    pub excess: f64,
    pub reachable_from_source: bool,
}
