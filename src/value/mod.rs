pub mod req;
pub mod req_by;

use crate::identifier::FastIdentifierT;
use crate::value::req::{ReqIndex, ValueReq};
use crate::value::req_by::{ReqByIndex, ValueReqBy};


pub type ValueNr = u32;

#[derive(Clone)]
pub struct Value<VD> {
    pub value_data: VD,
    pub reqs: Vec<ValueReq>,
    pub req_by: Vec<ValueReqBy>
}


pub trait ValueDataT: Copy + Default {
    fn get_value_nr(&self) -> ValueNr;
}

impl<VD> Value<VD> {
    pub fn new(user_data: VD) -> Self {
        Value {
            value_data: user_data,
            reqs: vec![],
            req_by: vec![] ,
        }
    }

    pub fn add_value_req(&mut self, value_req: ValueReq) -> ReqIndex {
        let index = self.reqs.len();
        self.reqs.push(value_req);
        
        index as ReqIndex
    }

    pub fn add_req_by(&mut self, req_by: ValueReqBy) -> ReqByIndex{
        let index = self.reqs.len();
        self.req_by.push(req_by);

        index as ReqByIndex
    }
    
    pub fn on_add_req_by(&mut self, req_index: ReqIndex) {
        self.reqs[req_index].on_add_req_by();
    }
    
    /// Returns true if this value should be removed
    pub fn on_remove_req_by(&mut self, req_index: ReqIndex) -> bool {
        self.reqs[req_index].on_remove_req_by()
    }
    

    /*
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
    
     */
}

