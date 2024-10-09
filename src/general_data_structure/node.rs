use std::fmt::Debug;
use crate::general_data_structure::{Value, ValueDataT, ValueNr};

pub type ValueIndex = usize;
const VALUE_INDEX_NONE: ValueIndex = ValueIndex::MAX;
const VALUE_INDEX_MAX: ValueIndex = ValueIndex::MAX - 1;



pub trait NodeT<VD: ValueDataT>: Clone + Default {
    
    fn new(num_values: usize) -> Self;
    fn get_values(&self) -> &[Value<VD>];
    fn get_values_mut(&mut self) -> &mut [Value<VD>];
    fn add_value_with_index(&mut self, value_index: ValueIndex, value_data: VD);

    /// Returns Ok with index if the general_data_structure is in node and Error with the index where the node should be added
    fn get_value_index_from_value_nr(&self, value_nr: ValueNr) -> Result<usize, usize>;
}