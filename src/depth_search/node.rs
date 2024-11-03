use crate::depth_search::depth_tree::{DepthTreeIndex, ReqAtIndex};
use crate::depth_search::value::DepthValue;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::node::{NodeT};
use crate::general_data_structure::value::{ValueDataT, ValueT};


#[derive(Clone, Default)]
pub struct DepthNode<VD: ValueDataT, FI: FastIdentifierT> {
    pub value: Option<DepthValue<VD>>,
    pub tree_nodes: Vec<(VD, DepthTreeIndex)>,
    pub chosen_node_index: Option<DepthTreeIndex>,
    pub fixed_value: Option<VD>,
    pub reqs_at: Vec<(FI, Vec<(DepthTreeIndex, ReqAtIndex)>)>,
}

impl<VD: ValueDataT, FI: FastIdentifierT> NodeT<DepthValue<VD>, VD> for DepthNode<VD, FI> {
    fn new(num_values: usize, base_value: Option<VD>) -> Self {
        DepthNode {
            value: base_value.map(|v| DepthValue::<VD>::new(v)),
            tree_nodes: vec![],
            chosen_node_index: None,
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


