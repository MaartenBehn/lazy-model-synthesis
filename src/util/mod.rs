
pub mod state_saver;

pub fn get_num_bits_for_number(num: usize) -> usize {
    (num.ilog2() + 1) as usize
}