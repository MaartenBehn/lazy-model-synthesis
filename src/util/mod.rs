
pub mod state_saver;

pub fn get_num_bits_for_number(num: usize) -> u32 {
    num.ilog2() + 1
}

pub fn get_mask_from_num_bits(num_bits: u32) -> u32 {
    2_u32.pow(num_bits) -1
}