
pub type ValueNr = u32;

pub trait ValueT<VD: ValueDataT>: Clone + Default {
    fn new(value_data: VD) -> Self;
    fn get_value_data(&self) -> VD;
}

pub trait ValueDataT: Copy + Default {
    fn get_value_nr(&self) -> ValueNr;
}
