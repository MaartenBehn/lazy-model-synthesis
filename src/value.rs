use crate::node::Node;

type ReqCounterValue = u32;

enum ReqOperation {
    NodeValueCounter,
    GlobalCounterLessThan,
    GlobalCounterMoreThan,
}

pub struct Value {
    pub reqs: Vec<rclite::Rc<ValueReq>>,
    pub required_by: Vec<rclite::Rc<ValueReq>>,
}

pub struct ValueReq {
    pub counter: ReqCounterValue,
    pub operation: ReqOperation,
    pub value: ReqCounterValue
}

impl Value {
    pub fn new() -> Self {
        Value {
            reqs: vec![],
            required_by: vec![] ,
        }
    }

    pub fn add_callback(&mut self, node: &mut Node) {

    }

    pub fn remove_callback(&mut self, node: &mut Node) {
        for req in self.required_by {
            match req.operation {
                ReqOperation::NodeValueCounter => {
                    req.counter -= 1;

                    if req.counter == 0 {
                        // remove other node
                    }
                }
                ReqOperation::GlobalCounterLessThan => {
                    req.counter -= 1;

                    if req.counter <= req.value {
                        // remove other node
                    }
                }
                ReqOperation::GlobalCounterMoreThan => {}
            }
        }
    }

    pub fn select_callback(&mut self, node: &mut Node) {
        for req in self.required_by {
            match req.operation {
                ReqOperation::GlobalCounterMoreThan => {
                    req.counter -= 1;
                }
                ReqOperation::NodeValueCounter | ReqOperation::GlobalCounterLessThan => {}
            }
        }
    }

    pub fn unselect_callback(&mut self, node: &mut Node) {
        for req in self.required_by {
            match req.operation {
                ReqOperation::GlobalCounterExact => {
                    if req.counter == req.value {
                        // remove other node
                    }

                    req.counter -= 1;
                }
                ReqOperation::GlobalCounterMoreThan => {
                    req.counter -= 1;
                }
                ReqOperation::NodeValueCounter | ReqOperation::GlobalCounterLessThan => {}
            }
        }
    }
}

impl ValueReq {
    pub fn new_node_value_counter() -> Self {
        ValueReq {
            counter: 0,
            operation: ReqOperation::NodeValueCounter,
            value: 0,
        }
    }

    pub fn new_global_counter_more_than(value: ReqCounterValue) -> Self {
        ValueReq {
            counter: 0,
            operation: ReqOperation::GlobalCounterMoreThan,
            value,
        }
    }

    pub fn new_global_counter_exact(value: ReqCounterValue) -> Self {
        ValueReq {
            counter: 0,
            operation: ReqOperation::GlobalCounterExact,
            value,
        }
    }

    pub fn new_global_counter_less_than(value: ReqCounterValue) -> Self {
        ValueReq {
            counter: 0,
            operation: ReqOperation::GlobalCounterLessThan,
            value,
        }
    }
}
