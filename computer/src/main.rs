use std::io::{stdin, Error};

use computer::{
    algorithms::find_flow,
    converters::{read_from_file, write_to_file},
    types::AlgorithmResult,
};

fn main() {
    let mut graph = read_from_file(&String::from("../static/mock_graph.txt"));
    let mut algorithm_result = AlgorithmResult::new();
    algorithm_result.start();
    let found_flow = find_flow(&mut graph, &mut algorithm_result);
    println!("RESULT: propagated {} units of flow.", found_flow);
    algorithm_result.finish(found_flow);
    println!("Output graph? y/n:");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.pop();

    if s == "y" || s == "Y" {
        println!("{:#?}", graph);
    }

    let result = write_to_file(&String::from("../static/result_graph.json"), &graph);
    println!("{:?}", result);

    let result = write_to_file(
        &String::from("../static/result_algorithm.json"),
        &algorithm_result,
    );
    println!("{:?}", result);
}
