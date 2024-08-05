use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::graph::Graph;
use crate::node::Node;
use crate::value::{Value, ValueReq};

mod node;
mod value;
mod graph;

fn main() {
    let mut graph = Graph::new();
    graph.nodes.push(Node::new());
    graph.nodes.push(Node::new());

    let mut value_0 = Value::new();
    let mut value_1 = Value::new();

    let value_req = ValueReq::new_node_value_counter();

    value_0.add_value_req(value_req, &mut [&mut value_1]);

    graph.nodes[0].add_value(value_0);
    graph.nodes[1].add_value(value_1);

    println!("Done")
}
