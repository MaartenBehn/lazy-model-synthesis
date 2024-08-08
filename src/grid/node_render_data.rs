use crate::grid::rules::ValueType;


const SELECTOR_BIT: usize = 31;
const BITS_PER_VALUE: usize = 4;

const ADDED_OFFSET: usize = 0;
const ADD_QUEUE_OFFSET: usize = 1;
const PROPERGATE_QUEUE_OFFSET: usize = 2;
const RESET_QUEUE_OFFSET: usize = 2;

const MAX_SELECTED_BITS: usize = 4;
const MAX_VALUE_TYPE_INDEX: usize = 2_usize.pow(MAX_SELECTED_BITS as u32);

// Selected Value       0 -  3
// ValueTypes possible  4 - 19
// SE               29 - 31

#[derive(Copy, Clone, Default)]
pub struct NodeRenderData {
    data: u32,
}

impl NodeRenderData {

    fn get_bit(&self, idx: usize) -> bool {
        (self.data >> idx & 1) == 1
    }
    fn set_bit(&mut self, idx: usize, b: bool) {
        self.data = (self.data & !(1 << idx)) | ((b as u32) << idx);
    }

    pub fn get_selector(&self) -> bool { self.get_bit(SELECTOR_BIT) }
    pub fn set_selector(&mut self, v: bool) { self.set_bit(SELECTOR_BIT, v) }


    fn get_value_bit(&self, value_type: ValueType, offset: usize) -> bool {
        let index = value_type as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.get_bit(index)
    }

    fn set_value_bit(&mut self, value_type: ValueType, offset: usize, v: bool) {
        let index = value_type as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.set_bit(index, v);
    }

    pub fn get_value_type(&self, value_type: ValueType) -> bool { self.get_value_bit(value_type, ADDED_OFFSET) }
    pub fn set_value_type(&mut self, value_type: ValueType, v: bool) { self.set_value_bit(value_type, ADDED_OFFSET, v) }

    pub fn get_add_queue(&self, value_type: ValueType) -> bool { self.get_value_bit(value_type, ADD_QUEUE_OFFSET) }
    pub fn set_add_queue(&mut self, value_type: ValueType, v: bool) { self.set_value_bit(value_type, ADD_QUEUE_OFFSET, v) }

    pub fn get_propagate_queue(&self, value_type: ValueType) -> bool { self.get_value_bit(value_type, PROPERGATE_QUEUE_OFFSET) }
    pub fn set_propagate_queue(&mut self, value_type: ValueType, v: bool) { self.set_value_bit(value_type, PROPERGATE_QUEUE_OFFSET, v) }

    pub fn get_reset_queue(&self, value_type: ValueType) -> bool { self.get_value_bit(value_type, RESET_QUEUE_OFFSET) }
    pub fn set_reset_queue(&mut self, value_type: ValueType, v: bool) { self.set_value_bit(value_type, RESET_QUEUE_OFFSET, v) }

    pub fn get_selected_value_type(&self) -> ValueType {
        ValueType::try_from(self.data & (MAX_VALUE_TYPE_INDEX - 1) as u32 - 1).unwrap()
    }

    pub fn set_selected_value_type(&mut self, value_type: ValueType)  {
        self.data = value_type as u32 + 1 + (self.data & !(MAX_VALUE_TYPE_INDEX - 1) as u32);
    }

}
