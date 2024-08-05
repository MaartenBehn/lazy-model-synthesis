use crate::node::Node;
use crate::value::Value;

pub struct Graph {
    pub nodes: Vec<Node>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
        }
    }
}