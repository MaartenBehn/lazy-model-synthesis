use std::collections::HashMap;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::{ValueDataT, ValueNr};

pub type DepthIndex = usize;
pub type DepthReqATIndex = usize;
pub type DepthReqIndex = usize;
pub type DepthLevel = usize;

#[derive(Default, Clone)]
pub struct DepthTreeController<FI: FastIdentifierT, VD: ValueDataT> {
    pub identifier_nodes: HashMap<FI, IdentifierNode>,
    pub nodes: Vec<DepthTreeNode<FI, VD>>,
}

#[derive(Clone)]
pub struct DepthTreeNode<FI: FastIdentifierT, VD: ValueDataT> {
    pub fast_identifier: FI,
    pub value_data: VD,
    pub level: DepthLevel,
    pub reqs: Vec<ReqAtIdentifier<FI>>,
    pub other_req: Vec<(DepthIndex, DepthReqATIndex, DepthReqIndex)>,
    pub satisfied: bool,
    pub processed: bool,
    pub applied: bool,
}

#[derive(Clone)]
pub struct ReqAtIdentifier<FI: FastIdentifierT> {
    pub fast_identifier: FI,
    pub tree_nodes: Vec<(ValueNr, DepthIndex)>,
    pub chosen_index: Option<usize>, 
}

#[derive(Clone)]
pub struct IdentifierNode {
    pub tree_nodes: Vec<(ValueNr, DepthIndex)>,
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn new() -> DepthTreeController<FI, VD> {
        DepthTreeController {
            identifier_nodes: HashMap::default(),
            nodes: vec![],
        }
    }
}

impl<FI: FastIdentifierT> ReqAtIdentifier<FI> {
    pub fn new(fast: FI) -> ReqAtIdentifier<FI> {
        ReqAtIdentifier {
            fast_identifier: fast,
            tree_nodes: vec![],
            chosen_index: None,
        }
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn reset(&mut self) {
        self.identifier_nodes = HashMap::default();
        self.nodes = vec![];
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeNode<FI, VD>{
    pub fn new(fast_identifier: FI, value_data: VD, level: DepthLevel) -> Self {
        Self { 
            fast_identifier, 
            value_data,
            level,
            reqs: vec![],
            other_req: vec![],
            satisfied: false,
            processed: false,
            applied: false,
        }
    }
}

impl IdentifierNode {
    pub fn new(nodes: Vec<(ValueNr, DepthIndex)>) -> Self {
        Self {
            tree_nodes: nodes,
        }
    }
}