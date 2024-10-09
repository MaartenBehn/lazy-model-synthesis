use crate::general_data_structure::{Value, ValueDataT, ValueNr};
use crate::general_data_structure::node::{NodeT, ValueIndex};

#[derive(Clone, Default)]
pub struct DepthNode<VD: ValueDataT> {
    pub values: Vec<Value<VD>>,
}

impl<VD: ValueDataT> NodeT<VD> for DepthNode<VD> {
    fn new(num_values: usize) -> Self {
        DepthNode {
            values: vec![],
        }
    }

    fn get_values(&self) -> &[Value<VD>] {
        &self.values
    }

    fn get_values_mut(&mut self) -> &mut [Value<VD>] {
        &mut self.values
    }

    fn add_value_with_index(&mut self, value_index: ValueIndex, value_data: VD) {
        self.values.insert(value_index as usize, Value::new(value_data))
    }

    fn get_value_index_from_value_nr(&self, value_nr: ValueNr) -> Result<usize, usize> {
        self.values.binary_search_by(|v| {
            v.value_data.get_value_nr().cmp(&value_nr)
        })
    }
}

