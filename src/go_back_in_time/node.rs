use std::iter;
use crate::general_data_structure::{Value, ValueDataT, ValueNr};
use crate::general_data_structure::node::{NodeT, ValueIndex};

pub type HistoryIndex = u16;

#[derive(Clone, Default)]
pub struct GoBackNode<D: ValueDataT> {
    pub values: Vec<Value<D>>,
    pub last_removed: Vec<HistoryIndex>,
    pub selected: bool,
}

impl<VD: ValueDataT> NodeT<VD> for GoBackNode<VD> {
    fn new(num_values: usize) -> Self {
        GoBackNode {
            last_removed: iter::repeat(0).take(num_values).collect(),
            values: vec![],
            selected: false,
        }
    }
    
    fn get_values(&self) -> &[Value<VD>] {
        &self.values
    }

    fn get_values_mut(&mut self) -> &mut [Value<VD>] {
        &mut self.values
    }

    fn set_values(&mut self, values: Vec<Value<VD>>) { self.values = values }

    fn add_value_with_index(&mut self, value_index: ValueIndex, value_data: VD) {
        self.values.insert(value_index as usize, Value::new(value_data))
    }


    fn get_value_index_from_value_nr(&self, value_nr: ValueNr) -> Result<usize, usize> {
        self.values.binary_search_by(|v| {
            v.value_data.get_value_nr().cmp(&value_nr)
        })
    }
}
