use crate::general_data_structure::value::{ValueDataT, ValueT};

#[derive(Copy, Clone, Default)]
pub struct DepthValue<VD> {
    pub value_data: VD,
}

impl<VD: ValueDataT> ValueT<VD> for DepthValue<VD> {
    fn new(user_data: VD) -> Self {
        DepthValue {
            value_data: user_data,
        }
    }

    fn get_value_data(&self) -> VD {
        self.value_data
    }
}