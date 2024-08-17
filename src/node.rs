use std::iter;
use crate::value::{Value, ValueDataT, ValueNr};

pub type ValueIndex = usize;
const VALUE_INDEX_NONE: ValueIndex = ValueIndex::MAX;
const VALUE_INDEX_MAX: ValueIndex = ValueIndex::MAX - 1;
pub type HistoryIndex = u16;

#[derive(Clone)]
pub struct Node<D: ValueDataT> {
    pub values: Vec<Value<D>>,

    pub last_removed: Vec<HistoryIndex>,
}

impl<D: ValueDataT> Node<D> {
    
    pub fn new(num_values: usize) -> Self {
        Node {
            last_removed: iter::repeat(0).take(num_values).collect(),
            values: vec![],
        }
    }
    
    pub fn add_value_with_index(&mut self, value_index: ValueIndex, value_data: D) {
        self.values.insert(value_index as usize, Value::new(value_data))
    }
    
    /// Returns Ok with index if the value is in node and Error with the index where the node should be added
    pub fn get_value_index_from_value_nr(&self, value_nr: ValueNr) -> Result<usize, usize> {
        self.values.binary_search_by(|v| {
            v.value_data.get_value_nr().cmp(&value_nr)
        })
    }
}

