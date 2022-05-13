extern crate self as computer;

pub mod algorithms;
pub mod converters;
pub mod types;

use serde::Serialize;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn find_generalized_flow(graph_input: String) -> String {
    std::panic::set_hook(Box::new(|_info| {}));
    #[derive(Serialize)]
    struct Result {
        initial_graph: types::DirectedGraph,
        algorithm_steps: types::AlgorithmResult,
        error: String,
    }

    let result = std::panic::catch_unwind(|| {
        let mut algorithm_result = types::AlgorithmResult::new();
        let mut graph = converters::read_from_string(graph_input);
        let initial_graph = graph.clone();

        algorithm_result.start();
        let found_flow = algorithms::find_flow(&mut graph, &mut algorithm_result);
        algorithm_result.finish(found_flow);
        Result {
            initial_graph,
            algorithm_steps: algorithm_result,
            error: "".to_string(),
        }
    });

    let result = match result {
        Ok(res) => res,
        Err(_err) => Result {
            initial_graph: types::DirectedGraph::new(),
            algorithm_steps: types::AlgorithmResult::new(),
            error: "Error caught.".to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}
