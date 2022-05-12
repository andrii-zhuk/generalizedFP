use super::*;

impl AlgorithmResult {
    #[must_use]
    pub const fn new() -> AlgorithmResult {
        AlgorithmResult { steps: vec![] }
    }
    pub fn start(&mut self) {
        self.steps.push(Action {
            step_type: AlgorithmStep::AlgorithmStart,
            result: true,
            pushed_flow: 0.0,
            nodes_affected: vec![],
            edges_affected: vec![],
        })
    }
    pub fn finish(&mut self, pushed_flow: f64) {
        self.steps.push(Action {
            step_type: AlgorithmStep::AlgorithmEnd,
            result: true,
            pushed_flow,
            nodes_affected: vec![],
            edges_affected: vec![],
        })
    }
    pub fn push_has_augmenting_path(&mut self, result: Option<f64>) {
        self.steps.push(Action {
            step_type: AlgorithmStep::HasAugmentingPath,
            result: result != None,
            pushed_flow: result.unwrap_or(0.0),
            nodes_affected: vec![],
            edges_affected: vec![],
        });
    }
    pub fn push_if_cycles_not_found(&mut self) {
        if self.steps.len() == 0
            || (self.steps.last().unwrap().step_type != AlgorithmStep::FindCycles
                && self.steps.last().unwrap().step_type != AlgorithmStep::CancelCycles)
        {
            self.steps.push(Action {
                step_type: AlgorithmStep::FindCycles,
                result: false,
                pushed_flow: 0.0,
                nodes_affected: vec![],
                edges_affected: vec![],
            })
        }
    }

    pub fn push_find_cycles(&mut self, edges: Vec<(usize, f64)>) {
        self.steps.push(Action {
            step_type: AlgorithmStep::FindCycles,
            result: true,
            pushed_flow: 0.0,
            nodes_affected: vec![],
            edges_affected: edges
                .into_iter()
                .map(|(edge_id, flow_amount)| -> AffectedEdge {
                    AffectedEdge {
                        edge_id,
                        flow_amount,
                    }
                })
                .collect(),
        })
    }
    pub fn push_cancel_cycles(&mut self, node_id: usize, excess_amount: f64) {
        self.steps.push(Action {
            step_type: AlgorithmStep::CancelCycles,
            result: true,
            pushed_flow: excess_amount,
            nodes_affected: vec![AffectedNode {
                node_id,
                excess_amount,
            }],
            edges_affected: vec![],
        })
    }
    pub fn push_find_path(
        &mut self,
        start_node: Option<usize>,
        excess_amount: f64,
        flow: f64,
        edges: Vec<(usize, f64)>,
    ) {
        self.steps.push(Action {
            step_type: AlgorithmStep::FindPath,
            result: true,
            pushed_flow: flow,
            nodes_affected: if start_node != None {
                vec![AffectedNode {
                    node_id: start_node.unwrap(),
                    excess_amount,
                }]
            } else {
                vec![]
            },
            edges_affected: edges
                .into_iter()
                .map(|(edge_id, flow_amount)| -> AffectedEdge {
                    AffectedEdge {
                        edge_id,
                        flow_amount,
                    }
                })
                .collect(),
        })
    }
}
