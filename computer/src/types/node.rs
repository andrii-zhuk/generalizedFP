use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Node {
    pub id: usize,
    pub label: String,
    pub excess: f64,
    pub reachable_from_source: bool,
}
