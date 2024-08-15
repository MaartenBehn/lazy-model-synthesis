use crate::node_identifier::PackedIdentifier;
use crate::util::get_num_bits_for_number;

#[derive(Clone)]
pub struct History<NodeStorage> {
    pub num_values: usize,
    pub num_values_bits: usize,
    pub num_values_mask: u32,
    
    pub nodes: Vec<u32>,
    pub summaries: Vec<NodeStorage>
}

impl<NodeStorage> History<NodeStorage> {
    pub fn new(node_storage: NodeStorage, num_values: usize) -> Self {
        
        let num_values_bits = get_num_bits_for_number(num_values);
        let num_values_mask = 2_u32.pow(num_values_bits as u32) -1;
        
        let initial_summary = 0;
        
        History {
            num_values,
            num_values_bits,
            num_values_mask,
            nodes: vec![initial_summary],
            summaries: vec![node_storage],
        }
    }

    pub fn add_summary(&mut self) {
        debug_assert!(self.summaries.len() >= (1 << 31), "Summaries are full!");
        
        todo!()
    }

    pub fn add_change<I: PackedIdentifier>(&mut self, packed_identifier: I, value_index: usize) {
        let identifier_bits = packed_identifier.to_bits();
        
        {
            let max_identifier_bits = 31 - self.num_values_bits;
            debug_assert!(
                identifier_bits > (1 << max_identifier_bits),
                "Packed Identifier can not be bigger than {max_identifier_bits} bits."
            );
        }
        
        let data = (1 << 31) + (identifier_bits << self.num_values_bits) + value_index as u32;
        self.nodes.push(data);
    }
    
    pub fn is_change(&self, index: usize) -> bool {
        self.nodes[index] & (1 << 31) != 0
    }
    
    pub fn get_summary(&self, index: usize) -> &NodeStorage {
        &self.summaries[self.nodes[index] as usize]
    }

    pub fn get_change<I: PackedIdentifier>(&self, index: usize) -> (I, usize) {
        let data = self.nodes[index] & !(1 << 31);
        let identifier_bits = data >> self.num_values_bits;
        let value_index = (data & self.num_values_mask) as usize;

        (I::from_bits(identifier_bits), value_index)
    }
}

