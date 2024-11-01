use crate::depth_search::value::DepthValue;
use crate::general_data_structure::node::{NodeT, ValueIndex};
use crate::general_data_structure::value::{ValueDataT, ValueNr, ValueT};

#[derive(Copy, Clone, Default)]
pub struct DepthNode<VD: ValueDataT> {
    pub value: Option<DepthValue<VD>>,
}

impl<VD: ValueDataT> NodeT<DepthValue<VD>, VD> for DepthNode<VD> {
    fn new(num_values: usize) -> Self {
        DepthNode {
            value: None,
        }
    }

    fn get_values(&self) -> &[DepthValue<VD>] {
        self.value.as_slice()
    }

    fn get_values_mut(&mut self) -> &mut [DepthValue<VD>] {
        self.value.as_mut_slice()
    }
}


