use crate::util::{get_mask_from_num_bits, get_num_bits_for_number};
use crate::general_data_structure::identifier::PackedIdentifierT;
use crate::general_data_structure::req::ReqIndex;
use crate::general_data_structure::value::ValueDataT;

pub type ReqByIndex = usize;


/// Packed Node Identifier + Value Nr + Req Index 
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ValueReqBy(u32);

#[derive(Default, Copy, Clone)]
pub struct ValueReqByPacker {
    req_mask: u32,
    value_nr_shift: u32,
    value_nr_mask: u32,
    identifier_shift: u32,
}

impl ValueReqByPacker {
    pub fn new(max_num_values: usize, max_num_reqs: usize) -> Self {
        let num_req_bits = get_num_bits_for_number(max_num_reqs - 1);
        let req_mask = get_mask_from_num_bits(num_req_bits);
        
        let num_values_bits = get_num_bits_for_number(max_num_values - 1);
        let num_values_mask = get_mask_from_num_bits(num_values_bits);
        
        let identifier_shift = num_req_bits + num_values_bits;
        
        ValueReqByPacker {
            req_mask, 
            value_nr_shift: num_req_bits,
            value_nr_mask: num_values_mask,
            identifier_shift,
            
        }
    }
    
    pub fn pack<PI: PackedIdentifierT, VD: ValueDataT>(&self, packed_identifier: PI, value_data: VD, req_index: ReqIndex) -> ValueReqBy {
        let identifier_bits = packed_identifier.to_bits();

        {   // Debug Check
            let max_identifier_bits = 31 - self.identifier_shift;
            debug_assert!(
                identifier_bits < (1 << max_identifier_bits),
                "Packed Identifier can not be bigger than {max_identifier_bits} bits."
            );
        }

        let data = (identifier_bits << self.identifier_shift) 
            + ((value_data.get_value_nr()) << self.value_nr_shift) 
            + (req_index as u32);
        ValueReqBy(data)
    }
    
    pub fn unpack<PI: PackedIdentifierT, VD: ValueDataT>(&self, req_by: ValueReqBy) -> (PI, VD, ReqIndex) {
        let identifier_bits = req_by.0 >> self.identifier_shift;
        let value_nr = (req_by.0 >> self.value_nr_shift) & self.value_nr_shift;
        let req_index = (req_by.0 & self.req_mask) as ReqIndex;
        let value_data = VD::from_value_nr(value_nr);
        
        (PI::from_bits(identifier_bits), value_data, req_index)
    }
}

