use crate::general_data_structure::req::{ReqIndex, ValueReq};
use crate::general_data_structure::req_by::{ReqByIndex, ValueReqBy};
use crate::general_data_structure::value::{ValueDataT, ValueT};

#[derive(Clone, Default)]
pub struct GoBackValue<VD> {
    pub value_data: VD,
    pub reqs: Vec<ValueReq>,
    pub req_by: Vec<ValueReqBy>
}

impl<VD: ValueDataT> ValueT<VD> for GoBackValue<VD> {
    fn new(user_data: VD) -> Self {
        GoBackValue {
            value_data: user_data,
            reqs: vec![],
            req_by: vec![] ,
        }
    }

    fn get_value_data(&self) -> VD {
        self.value_data
    }
}

impl<VD> GoBackValue<VD> {
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
}