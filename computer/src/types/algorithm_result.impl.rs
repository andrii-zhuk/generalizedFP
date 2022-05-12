use super::*;

impl AlgorithmResult {
    #[must_use]
    pub const fn new() -> AlgorithmResult {
        AlgorithmResult { steps: vec![] }
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

    pub fn push_find_cycles(&mut self) {}
}
