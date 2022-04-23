use computer::{algorithms::has_augmenting_path, converters::read_from_file};

fn main() {
    println!("Hello, world123!");
    let result = read_from_file(&String::from("../static/mock_graph.txt"));
    println!("{:#?}", result);
    println!("{}", has_augmenting_path(&result));
}
