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

    graph.nodes[0].values.push(Value::new());
    graph.nodes[1].values.push(Value::new());

    graph.nodes[0].values[0].reqs.push(rclite::Rc::new(ValueReq::new()));
    let rc = graph.nodes[0].values[0].reqs[0].clone();
    graph.nodes[1].values[0].required_by.push(rc);


    println!("Done")
}
