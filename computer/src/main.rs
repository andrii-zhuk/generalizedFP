use computer::{
    algorithms::{get_fat_path_node_ids, propagate_fat_path},
    converters::read_from_file,
};

fn main() {
    println!("Hello, world123!");
    let mut result = read_from_file(&String::from("../static/mock_graph.txt"));
    println!("{:#?}", result);
    println!("{:#?}", get_fat_path_node_ids(&result).unwrap_or(vec![]));
    println!("{:?}", propagate_fat_path(&mut result).unwrap_or(47.0));
    println!("{:?}", result);
}
