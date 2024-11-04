use crate::rules::ValueType;

const SELECTOR_BIT: usize = 31;
const NEXT_BIT: usize = 30;
const HAS_TREE_IDENTIFIER_BIT: usize = 29;

const BITS_PER_VALUE: usize = 4;
const ADDED_OFFSET: usize = 0;

const MAX_SELECTED_BITS: usize = 4;
const MAX_VALUE_TYPE_INDEX: usize = 2_usize.pow(MAX_SELECTED_BITS as u32);

pub const NUM_VALUE_TYPES: u32 = 3;

#[derive(Copy, Clone)]
pub struct NodeRenderData {
    data: u32,
}

impl NodeRenderData {
    
    pub fn new(base_value: Option<ValueType>) -> NodeRenderData {
        let mut data = NodeRenderData {
            data: 0,
        };
        
        if base_value.is_some() {
            data.set_selected_value_type(base_value.unwrap());
        }
        
        data
    }

    fn get_bit(&self, idx: usize) -> bool {
        (self.data >> idx & 1) == 1
    }
    fn set_bit(&mut self, idx: usize, b: bool) {
        self.data = (self.data & !(1 << idx)) | ((b as u32) << idx);
    }

    pub fn get_selector(&self) -> bool { self.get_bit(SELECTOR_BIT) }
    pub fn set_selector(&mut self, v: bool) { self.set_bit(SELECTOR_BIT, v) }

    pub fn get_next(&self) -> bool { self.get_bit(NEXT_BIT) }

    pub fn set_next(&mut self, v: bool) { self.set_bit(NEXT_BIT, v) }

    pub fn get_depth_tree_identifier(&self) -> bool { self.get_bit(HAS_TREE_IDENTIFIER_BIT) }

    pub fn set_depth_tree_identifier(&mut self, v: bool) { self.set_bit(HAS_TREE_IDENTIFIER_BIT, v) }

    fn get_value_bit(&self, value_type: ValueType, offset: usize) -> bool {
        let index = value_type.get_value_nr() as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.get_bit(index)
    }

    fn set_value_bit(&mut self, value_type: ValueType, offset: usize, v: bool) {
        let index = value_type.get_value_nr() as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.set_bit(index, v);
    }

    pub fn get_value_type(&self, value_type: ValueType) -> bool { self.get_value_bit(value_type, ADDED_OFFSET) }
    pub fn set_value_type(&mut self, value_type: ValueType, v: bool) { self.set_value_bit(value_type, ADDED_OFFSET, v) }

    pub fn get_queue(&self, value_type: ValueType, i: usize) -> bool { self.get_value_bit(value_type, i) }
    pub fn set_queue(&mut self, value_type: ValueType, v: bool, i: usize) { self.set_value_bit(value_type, i, v) }
    
    pub fn get_selected_value_type(&self) -> ValueType {
        ValueType::from_value_nr(self.data & (MAX_VALUE_TYPE_INDEX - 1) as u32 - 1)
    }

    pub fn set_selected_value_type(&mut self, value_type: ValueType)  {
        self.data = value_type.get_value_nr() + 1 + (self.data & !(MAX_VALUE_TYPE_INDEX - 1) as u32);
    }

    pub fn unselected_value_type(&mut self)  {
        self.data = self.data & !(MAX_VALUE_TYPE_INDEX - 1) as u32;
    }

}
