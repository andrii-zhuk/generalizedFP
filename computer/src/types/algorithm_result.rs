use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub enum AlgorithmStep {
    #[serde(rename = "0")]
    AlgorithmStart,
    #[serde(rename = "1")]
    HasAugmentingPath,
    #[serde(rename = "2")]
    FindCycles,
    #[serde(rename = "3")]
    CancelCycles,
    #[serde(rename = "4")]
    FindPath,
    #[serde(rename = "5")]
    AlgorithmEnd,
}

#[derive(Serialize, Debug)]
pub struct AffectedNode {
    node_id: usize,
    excess_amount: f64,
}
#[derive(Serialize, Debug)]
pub struct AffectedEdge {
    edge_id: usize,
    flow_amount: f64,
}
#[derive(Serialize, Debug)]
pub struct Action {
    step_type: AlgorithmStep,
    result: bool,
    pushed_flow: f64,
    nodes_affected: Vec<AffectedNode>,
    edges_affected: Vec<AffectedEdge>,
}

#[derive(Serialize, Debug)]
pub struct AlgorithmResult {
    steps: Vec<Action>,
}

#[path = "algorithm_result.impl.rs"]
mod implementation;
