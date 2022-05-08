use std::io::stdin;

use computer::{algorithms::find_flow, converters::read_from_file};

fn main() {
    let mut result = read_from_file(&String::from("../static/mock_graph.txt"));
    println!(
        "RESULT: propagated {} units of flow.",
        find_flow(&mut result)
    );
    println!("Output graph? y/n:");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.pop();

    if s == "y" || s == "Y" {
        println!("{:#?}", result);
    }
}
