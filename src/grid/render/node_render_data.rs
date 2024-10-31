
use crate::general_data_structure::ValueNr;


const SELECTOR_BIT: usize = 31;
const NEXT_BIT: usize = 30;
const HAS_TREE_IDENTIFIER_BIT: usize = 29;

const BITS_PER_VALUE: usize = 6;

const ADDED_OFFSET: usize = 0;
const SELECT_OFFSET: usize = 1;
const ADD_OFFSET: usize = 2;
const REMOVE_OFFSET: usize = 3;
const TREE_BUILD_OFFSET: usize = 4;
const TREE_APPLY_OFFSET: usize = 5;

const MAX_SELECTED_BITS: usize = 4;
const MAX_VALUE_TYPE_INDEX: usize = 2_usize.pow(MAX_SELECTED_BITS as u32);

pub const NUM_VALUE_TYPES: u32 = 3;

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

    pub fn get_next(&self) -> bool { self.get_bit(NEXT_BIT) }

    pub fn set_next(&mut self, v: bool) { self.set_bit(NEXT_BIT, v) }

    pub fn get_depth_tree_identifier(&self) -> bool { self.get_bit(HAS_TREE_IDENTIFIER_BIT) }

    pub fn set_depth_tree_identifier(&mut self, v: bool) { self.set_bit(HAS_TREE_IDENTIFIER_BIT, v) }

    fn get_value_bit(&self, value_nr: ValueNr, offset: usize) -> bool {
        let index = value_nr as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.get_bit(index)
    }

    fn set_value_bit(&mut self, value_nr: ValueNr, offset: usize, v: bool) {
        let index = value_nr as usize * BITS_PER_VALUE + offset + MAX_SELECTED_BITS;
        self.set_bit(index, v);
    }

    pub fn get_value_type(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, ADDED_OFFSET) }
    pub fn set_value_type(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, ADDED_OFFSET, v) }

    pub fn get_select_queue(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, SELECT_OFFSET) }
    pub fn set_select_queue(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, ADD_OFFSET, v) }

    pub fn get_add_queue(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, ADD_OFFSET) }
    pub fn set_add_queue(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, ADD_OFFSET, v) }

    pub fn get_remove_queue(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, REMOVE_OFFSET) }
    pub fn set_remove_queue(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, REMOVE_OFFSET, v) }

    pub fn get_tree_build_queue(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, TREE_BUILD_OFFSET) }
    pub fn set_tree_build_queue(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, TREE_BUILD_OFFSET, v) }

    pub fn get_tree_apply_queue(&self, value_nr: ValueNr) -> bool { self.get_value_bit(value_nr, TREE_APPLY_OFFSET) }
    pub fn set_tree_apply_queue(&mut self, value_nr: ValueNr, v: bool) { self.set_value_bit(value_nr, TREE_APPLY_OFFSET, v) }
    
    pub fn get_selected_value_type(&self) -> ValueNr {
        (self.data & (MAX_VALUE_TYPE_INDEX - 1) as u32 - 1) as ValueNr
    }

    pub fn set_selected_value_type(&mut self, value_nr: ValueNr)  {
        self.data = value_nr as u32 + 1 + (self.data & !(MAX_VALUE_TYPE_INDEX - 1) as u32);
    }

    pub fn unselected_value_type(&mut self)  {
        self.data = self.data & !(MAX_VALUE_TYPE_INDEX - 1) as u32;
    }

}
