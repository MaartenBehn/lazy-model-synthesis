use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::value::{ValueDataT};

/// The Index of a node in the DepthTreeController.nodes
pub type DepthTreeIndex = usize;

/// The Index of ReqAtIdentifier in a DepthTreeNode
pub type ReqAtIndex = usize;

/// The Index of ReqAtIdentifier.tree_nodes entry in a DepthTreeNode
pub type InReqAtIndex = usize;

/// The Level of a DepthTreeNode in the DepthTree
pub type DepthLevel = usize;

#[derive(Default, Clone)]
pub struct DepthTreeController<FI: FastIdentifierT, VD: ValueDataT> {
    pub nodes: Vec<DepthTreeNode<FI, VD>>,
}

#[derive(Clone)]
pub struct DepthTreeNode<FI: FastIdentifierT, VD: ValueDataT> {
    pub fast_identifier: FI,
    pub value_data: VD,
    pub level: DepthLevel,
    pub reqs: Vec<ReqAtIdentifier<FI, VD>>,
    pub req_by: Vec<(DepthTreeIndex, ReqAtIndex, InReqAtIndex)>,
    pub satisfied: bool,
    pub possible: bool,
    pub build: bool,
    pub applied: bool,
}

#[derive(Clone)]
pub struct ReqAtIdentifier<FI: FastIdentifierT, VD> {
    pub fast_identifier: FI,
    pub tree_nodes: Vec<(VD, DepthTreeIndex)>,
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn new() -> DepthTreeController<FI, VD> {
        DepthTreeController {
            nodes: vec![],
        }
    }
}

impl<FI: FastIdentifierT, VD> ReqAtIdentifier<FI, VD> {
    pub fn new(fast: FI) -> ReqAtIdentifier<FI, VD> {
        ReqAtIdentifier {
            fast_identifier: fast,
            tree_nodes: vec![],
        }
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> DepthTreeController<FI, VD> {
    pub fn reset(&mut self) {
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
            req_by: vec![],
            satisfied: false,
            possible: true,
            build: false,
            applied: false,
        }
    }
}