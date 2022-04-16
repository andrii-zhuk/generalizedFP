use crate::types::DirectedGraph;

impl DirectedGraph {
    pub fn temp(& self) {
        println!("durkaaaa {}", self.adj_lists.len());
        if self.nodes.len() != 0 {
            for node in &self.nodes {
                println!("node {}", node.label);
            }
        }
    }
}
