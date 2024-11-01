use std::iter;
use std::marker::PhantomData;
use crate::general_data_structure::node::{NodeT, ValueIndex};
use crate::general_data_structure::value::{ValueDataT, ValueNr, ValueT};
use crate::go_back_in_time::value::GoBackValue;

pub type HistoryIndex = u16;

#[derive(Clone, Default)]
pub struct GoBackNode<VD: ValueDataT> {
    pub values: Vec<GoBackValue<VD>>,
    pub last_removed: Vec<HistoryIndex>,
    pub selected: bool,
}

impl<VD: ValueDataT> NodeT<GoBackValue<VD>, VD> for GoBackNode<VD> {
    fn new(num_values: usize) -> Self {
        GoBackNode {
            last_removed: iter::repeat(0).take(num_values).collect(),
            values: vec![],
            selected: false,
        }
    }
    
    fn get_values(&self) -> &[GoBackValue<VD>] {
        &self.values
    }

    fn get_values_mut(&mut self) -> &mut [GoBackValue<VD>] {
        &mut self.values
    }

    
}

impl<VD: ValueDataT> GoBackNode<VD> {
    pub fn add_value_with_index(&mut self, value_index: ValueIndex, value_data: VD) {
        self.values.insert(value_index as usize, GoBackValue::new(value_data))
    }


    pub fn get_value_index_from_value_nr(&self, value_nr: ValueNr) -> Result<usize, usize> {
        self.values.binary_search_by(|v| {
            v.get_value_data().get_value_nr().cmp(&value_nr)
        })
    }
}

