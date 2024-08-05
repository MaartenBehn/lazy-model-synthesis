use crate::value::Value;

type ValueIndex = usize;
const VALUE_INDEX_NONE: ValueIndex = ValueIndex::MAX;
const VALUE_INDEX_MAX: ValueIndex = ValueIndex::MAX - 1;

pub struct Node {
    values: Vec<Value>,
    selected_index: ValueIndex
}

impl Node {
    pub fn new() -> Self {
        Node {
            values: vec![],
            selected_index: VALUE_INDEX_NONE,
        }
    }

    pub fn add_value(&mut self, mut value: Value) {
        value.add_callback();
        self.values.push(value);
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

