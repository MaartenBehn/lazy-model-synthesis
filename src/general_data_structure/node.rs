use std::fmt::Debug;
use crate::general_data_structure::value::{ValueDataT, ValueT};

pub type ValueIndex = usize;
const VALUE_INDEX_NONE: ValueIndex = ValueIndex::MAX;
const VALUE_INDEX_MAX: ValueIndex = ValueIndex::MAX - 1;

pub trait NodeT<V: ValueT<VD>, VD: ValueDataT>: Clone + Default {
    
    fn new(num_values: usize) -> Self;
    fn get_values(&self) -> &[V];
    fn get_values_mut(&mut self) -> &mut [V];
}