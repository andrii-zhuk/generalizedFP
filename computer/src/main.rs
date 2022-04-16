use computer::converters::read_from_file;

fn main() {
    println!("Hello, world123!");
    let result = read_from_file(&String::from("../static/mock_graph.txt"));
    println!("{:#?}", result);
}
