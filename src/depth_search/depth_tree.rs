use std::collections::HashMap;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::{ValueDataT, ValueNr};

#[derive(Default, Clone)]
pub struct DepthTreeController<FI: FastIdentifierT, VD: ValueDataT> {
    pub identifier_nodes: HashMap<FI, IdentifierNodes>,
    pub nodes: Vec<DepthTreeNode<FI, VD>>,
}

#[derive(Clone)]
pub struct DepthTreeNode<FI: FastIdentifierT, VD: ValueDataT> {
    pub fast_identifier: FI,
    pub value_data: VD,
    pub level: usize,
    pub reqs: Vec<ReqAtIdentifier<FI>>
}

#[derive(Clone)]
pub struct ReqAtIdentifier<FI: FastIdentifierT> {
    pub fast_identifier: FI,
    pub nodes: Vec<(ValueNr, usize)>,
}

#[derive(Clone)]
pub struct IdentifierNodes {
    pub nodes: Vec<(ValueNr, usize)>,
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn new() -> DepthTreeController<FI, VD> {
        DepthTreeController {
            identifier_nodes: HashMap::default(),
            nodes: vec![],
        }
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeNode<FI, VD>{
    pub fn new(fast_identifier: FI, value_data: VD, level: usize) -> Self {
        Self { 
            fast_identifier, 
            value_data,
            level,
            reqs: vec![],
        }
    }
}

impl IdentifierNodes{
    pub fn new(nodes: Vec<(ValueNr, usize)>) -> Self {
        Self {
            nodes,
        }
    }
}