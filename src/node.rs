use crate::node_storage::NodeStorage;
use crate::value::{Value};

pub type ValueIndex = usize;
const VALUE_INDEX_NONE: ValueIndex = ValueIndex::MAX;
const VALUE_INDEX_MAX: ValueIndex = ValueIndex::MAX - 1;

#[derive(Clone)]
pub struct Node<D> {
    pub values: Vec<Value<D>>,
    selected_index: ValueIndex
}

impl<D> Node<D> {
    pub fn add_value<>(&mut self, value_data: D) -> ValueIndex {
        self.values.push(Value::new(value_data));
        self.values.len() - 1
    }

    pub fn get_value_index<P: FnMut(&D) -> bool>(&self, mut predicate: P) -> Option<ValueIndex> {
        self.values.iter().position(|v| {
            predicate(&v.value_data)
        })
    }

    pub fn remove_value(&mut self, index: ValueIndex) {
        if index == self.selected_index {
            self.unselect_value(index);
        }

        let mut removed_value = self.values.swap_remove(index);
        removed_value.remove_callback();
    }

    pub fn select_value(&mut self, index: ValueIndex) {
        self.selected_index = index;
        self.values[index].select_callback();

    }

    pub fn unselect_value(&mut self, index: ValueIndex) {
        self.selected_index = VALUE_INDEX_NONE;
        self.values[index].unselect_callback();

    }
}

impl<D> Default for Node<D> {
    fn default() -> Self {
        Node {
            values: vec![],
            selected_index: VALUE_INDEX_NONE,
        }
    }
}

