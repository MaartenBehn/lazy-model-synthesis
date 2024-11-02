use crate::depth_search::depth_tree::{DepthIndex, DepthReqATIndex, DepthReqIndex, ReqAtIdentifier};
use crate::depth_search::value::DepthValue;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::node::{NodeT};
use crate::general_data_structure::value::{ValueDataT};

#[derive(Clone, Default)]
pub struct DepthNode<VD: ValueDataT, FI: FastIdentifierT> {
    pub value: Option<DepthValue<VD>>,
    pub tree_nodes: Vec<(VD, DepthIndex)>,
    pub chosen_node_index: Option<usize>,
    pub fixed_value: Option<VD>,
    pub reqs_at: Vec<(FI, Vec<(DepthIndex, DepthReqATIndex)>)>,
}

impl<VD: ValueDataT, FI: FastIdentifierT> NodeT<DepthValue<VD>, VD> for DepthNode<VD, FI> {
    fn new(num_values: usize) -> Self {
        DepthNode {
            value: None,
            tree_nodes: vec![],
            fixed_value: None,
            reqs_at: vec![],
        }
    }

    fn get_values(&self) -> &[DepthValue<VD>] {
        self.value.as_slice()
    }

    fn get_values_mut(&mut self) -> &mut [DepthValue<VD>] {
        self.value.as_mut_slice()
    }
}


