
use crate::util::get_num_bits_for_number;
use crate::general_data_structure::identifier::PackedIdentifierT;
use crate::general_data_structure::ValueNr;


pub type SummaryIndex = u32;

/// 1 Bit for Change or Summary + Packed Node Identifier + Value Nr
#[derive(Default, Copy, Clone)]
pub struct HistoryNode(u32);

#[derive(Default, Copy, Clone)]
pub struct HistoryNodePacker {
    pub num_values_bits: u32,
    pub num_values_mask: u32,
}

impl HistoryNodePacker {
    pub fn new(num_values: usize) -> Self {
        let num_values_bits = get_num_bits_for_number(num_values);
        let num_values_mask = 2_u32.pow(num_values_bits as u32) -1;

        HistoryNodePacker {
            num_values_bits,
            num_values_mask,
        }
    }

    pub fn pack_change<I: PackedIdentifierT>(&self, packed_identifier: I, value_nr: ValueNr) -> HistoryNode {
        let identifier_bits = packed_identifier.to_bits();

        {
            let max_identifier_bits = 31 - self.num_values_bits;
            debug_assert!(
                identifier_bits < (1 << max_identifier_bits),
                "Packed Identifier can not be bigger than {max_identifier_bits} bits."
            );
        }

        let data = (1 << 31) + (identifier_bits << self.num_values_bits) + value_nr;
        HistoryNode(data)
    }


    pub fn unpack_change<I: PackedIdentifierT>(&self, node: HistoryNode) -> (I, ValueNr) {
        let data = node.0 & !(1 << 31);
        let identifier_bits = data >> self.num_values_bits;
        let value_index = data & self.num_values_mask;

        (I::from_bits(identifier_bits), value_index)
    }
    
    pub fn pack_summary(&self, index: SummaryIndex) -> HistoryNode {
        HistoryNode(index)
    }

    pub fn unpack_summary(&self, history_node: HistoryNode) -> SummaryIndex {
        history_node.0
    }
}

impl HistoryNode {
    pub fn is_change(&self) -> bool {
        self.0 & (1 << 31) != 0
    }

    pub fn is_summary(&self) -> bool {
        self.0 & (1 << 31) == 0
    }
}
