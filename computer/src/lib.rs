extern crate self as computer;

pub mod algorithms;
pub mod converters;
pub mod types;

use std::env;

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> String {
    // let mut graph = converters::read_from_file(&String::from("../../static/mock_graph.txt"));

    // let mut algorithm_result = types::AlgorithmResult::new();
    // algorithm_result.start();
    // let found_flow = algorithms::find_flow(&mut graph, &mut algorithm_result);
    // println!("RESULT: propagated {} units of flow.", found_flow);
    // algorithm_result.finish(found_flow);

    // format!("{:#?}", graph)
    format!(
        "{}{}{}",
        env::current_dir().unwrap_or_default().display().to_string(),
        a,
        b
    )
}
