use image::Rgba;

pub const VALUE_NONE: Value = Value(0);

pub type ValueNr = u32;

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Value(pub u32);

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ValueColor{
    r: u8, 
    g: u8, 
    b: u8,
    fill: u8,
}

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

impl ValueColor {
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ValueColor {r, g, b, fill: 0}
    }
    pub fn from_rgba(c: Rgba<u8>) -> Self {
        ValueColor {
            r: c.0[0],
            g: c.0[1],
            b: c.0[2],
            fill: 0,
        }
    }
}