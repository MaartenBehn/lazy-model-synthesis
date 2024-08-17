
pub type ReqCounterValue = i32;
pub type ReqIndex = usize;

// Remove value if there are:
#[derive(Copy, Clone)]
enum ReqOperation {
    NodeValueCounter,
    GlobalCounterLessThan,
    GlobalCounterMoreThan,
}

#[derive(Copy, Clone)]
pub struct ValueReq {
    counter: ReqCounterValue,
    operation: ReqOperation,
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

    pub fn on_add_req_by(&mut self) {
        match self.operation {
            ReqOperation::NodeValueCounter => {
                self.counter += 1;
            }
            ReqOperation::GlobalCounterLessThan => {
                self.counter += 1;
            }
            ReqOperation::GlobalCounterMoreThan => {}
        }
    }

    /// Returns true if the value of this req needs to be removed
    pub fn on_remove_req_by(&mut self) -> bool {
        match self.operation {
            ReqOperation::NodeValueCounter => {
                self.counter -= 1;

                assert!(self.counter >= 0, "The counter should not be negative, because if a value removes it should fist add one");
                if self.counter == 0 {
                    // remove
                    return true
                }
            }
            ReqOperation::GlobalCounterLessThan => {
                self.counter -= 1;

                if self.counter <= 0 {
                    // remove
                    return true
                }
            }
            ReqOperation::GlobalCounterMoreThan => {}
        }
        
        false
    }
}
