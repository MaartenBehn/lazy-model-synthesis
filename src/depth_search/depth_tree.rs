use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::{ValueDataT, ValueNr};

#[derive(Default, Clone)]
pub struct DepthTreeController<FI: FastIdentifierT, VD: ValueDataT> {
    pub nodes: Vec<DepthTreeNode<FI, VD>>,
}

#[derive(Clone)]
pub struct DepthTreeNode<FI: FastIdentifierT, VD: ValueDataT> {
    pub fast_identifier: FI,
    pub value_data: VD,
    pub parent: usize,
    pub reqs: Vec<DepthTreeNodeReq<FI>>
}

#[derive(Clone)]
pub struct DepthTreeNodeReq<FI: FastIdentifierT> {
    pub fast_identifier: FI,
    pub reqs: Vec<(ValueNr, usize)>,
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn new() -> DepthTreeController<FI, VD> {
        DepthTreeController {
            nodes: vec![],
        }
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeNode<FI, VD>{
    pub fn new(fast_identifier: FI, value_data: VD, parent: usize) -> Self {
        Self { 
            fast_identifier, 
            value_data,
            parent,
            reqs: vec![],
        }
    }
}