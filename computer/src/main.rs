use std::io::{stdin, Error};

use computer::{algorithms::find_flow, converters::{read_from_file, to_file}};

fn main() {
    let mut graph = read_from_file(&String::from("../static/mock_graph.txt"));
    println!(
        "RESULT: propagated {} units of flow.",
        find_flow(&mut graph)
    );
    println!("Output graph? y/n:");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.pop();

    if s == "y" || s == "Y" {
        println!("{:#?}", graph);
    }

    let serialized = serde_json::to_string(&graph).unwrap();
    println!("serialized {}", serialized);

    let result = to_file(&String::from("../static/result_graph.json"), &graph);
        println!("{:?}", result);
}
