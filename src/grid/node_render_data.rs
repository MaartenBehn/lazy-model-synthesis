

const SELECTOR_BIT: usize = 31;
const ADD_QUEUE_BIT: usize = 32;
const PROPAGATE_QUEUE_BIT: usize = 31;
const RESET_QUEUE_BIT: usize = 30;
const MAX_NODE_TYPE_INDEX: usize = 2_usize.pow(30) - 1;

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

    pub fn get_selector(&mut self) -> bool { self.get_bit(SELECTOR_BIT) }
    pub fn set_selector(&mut self, v: bool) { self.set_bit(SELECTOR_BIT, v) }

    pub fn get_add_queue(&mut self) -> bool { self.get_bit(ADD_QUEUE_BIT) }
    pub fn set_add_queue(&mut self, v: bool) { self.set_bit(ADD_QUEUE_BIT, v) }

    pub fn get_propagate_queue(&mut self) -> bool { self.get_bit(PROPAGATE_QUEUE_BIT) }
    pub fn set_propagate_queue(&mut self, v: bool) { self.set_bit(PROPAGATE_QUEUE_BIT, v) }

    pub fn get_reset_queue(&mut self) -> bool { self.get_bit(RESET_QUEUE_BIT) }
    pub fn set_reset_queue(&mut self, v: bool) { self.set_bit(RESET_QUEUE_BIT, v) }

}
