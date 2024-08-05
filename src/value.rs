use std::cell::{Cell, RefCell, RefMut};
use std::ops::DerefMut;
use crate::node::Node;

type ReqCounterValue = i32;

// Remove if there are:
enum ReqOperation {
    NodeValueCounter,
    GlobalCounterLessThan,
    GlobalCounterMoreThan,
}

pub struct Value {
    pub reqs: Vec<rclite::Rc<RefCell<ValueReq>>>,
    pub required_by: Vec<rclite::Rc<RefCell<ValueReq>>>,
}

pub struct ValueReq {
    pub counter: ReqCounterValue,
    pub operation: ReqOperation,
}

impl Value {
    pub fn new() -> Self {
        Value {
            reqs: vec![],
            required_by: vec![] ,
        }
    }

    pub fn add_value_req(&mut self, value_req: ValueReq, requirements: &mut [&mut Value]) {
        let rc = rclite::Rc::new(RefCell::new(value_req));

        for value in requirements.iter_mut() {
            value.required_by.push(rc.clone());
        }

        self.reqs.push(rc);
    }

    fn required_by_iter(&mut self) -> impl Iterator<Item = RefMut<ValueReq>> {
        self.required_by.iter_mut().map(|req| { req.borrow_mut() })
    }

    pub fn add_callback(&mut self) {
        for mut req in self.required_by_iter() {

            match req.operation {
                ReqOperation::NodeValueCounter => {
                    req.counter += 1;
                }
                ReqOperation::GlobalCounterLessThan => {
                    req.counter += 1;
                }
                ReqOperation::GlobalCounterMoreThan => {}
            }
        }
    }

    pub fn remove_callback(&mut self) {
        for mut req in self.required_by_iter() {

            match req.operation {
                ReqOperation::NodeValueCounter => {
                    req.counter -= 1;

                    assert!(req.counter >= 0, "The counter should not be negative, because if a value removes it should fist add one");
                    if req.counter == 0 {
                        // remove other node
                    }
                }
                ReqOperation::GlobalCounterLessThan => {
                    req.counter -= 1;

                    if req.counter <= 0 {
                        // remove other node
                    }
                }
                ReqOperation::GlobalCounterMoreThan => {}
            }
        }
    }

    pub fn select_callback(&mut self) {
        for mut req in self.required_by_iter() {
            match req.operation {
                ReqOperation::GlobalCounterMoreThan => {
                    req.counter += 1;

                    if req.counter >= 0 {
                        // remove other node
                    }
                }
                ReqOperation::NodeValueCounter | ReqOperation::GlobalCounterLessThan => {}
            }
        }
    }

    pub fn unselect_callback(&mut self) {
        for mut req in self.required_by_iter() {
            match req.operation {
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
        }
    }

    pub fn new_global_counter_more_than(value: ReqCounterValue) -> Self {
        ValueReq {
            counter: -value,
            operation: ReqOperation::GlobalCounterMoreThan,
        }
    }

    pub fn new_global_counter_less_than(value: ReqCounterValue) -> Self {
        ValueReq {
            counter: value,
            operation: ReqOperation::GlobalCounterLessThan,
        }
    }
}
