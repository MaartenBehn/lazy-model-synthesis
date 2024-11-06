
pub const VALUE_NONE: Value = Value(0);

pub type ValueNr = u32;

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Value (pub u32);

impl Value {
    pub fn is_none(&self) -> bool {
        *self == VALUE_NONE
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    pub fn get_value_nr(&self) -> ValueNr {
        (*self).0 - 1
    }
    pub fn from_value_nr(value_nr: ValueNr) -> Value {
        Value{ 0: value_nr + 1 }
    }
}