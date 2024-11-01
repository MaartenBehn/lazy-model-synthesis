use std::fmt::Debug;

pub trait ValueT<VD: ValueDataT>: Clone + Default + Eq + PartialEq {
    fn new(value_data: VD) -> Self;
    fn get_value_data(&self) -> VD;
}

pub trait ValueDataT: Copy + Default + Eq + PartialEq + Debug {
    fn get_value_nr(&self) -> u32;
    fn from_value_nr(value_nr: u32) -> Self;
    
}
