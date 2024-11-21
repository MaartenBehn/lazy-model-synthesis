use image::Rgba;

pub type ValueNr = u8;

pub const VALUE_NONE: Value = Value {
    color_index: 0,
    debug: 0,
    fill: 0,
    fill2: 0,
};

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Value{
    pub color_index: u8,
    debug: u8,
    fill: u8,
    fill2: u8,
}

#[derive(Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ValueColor{
    r: u8, 
    g: u8, 
    b: u8,
    fill: u8,
}

impl Value {
    pub fn is_none(&self) -> bool {
        self.color_index == 0
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    pub fn get_value_nr(&self) -> ValueNr {
        (self.color_index - 1 ) as ValueNr
    }
    pub fn from_value_nr(value_nr: ValueNr) -> Value {
        Value{
            color_index: (value_nr + 1) as u8,
            debug: 0,
            fill: 0,
            fill2: 0,
        }
    }
    
    pub fn set_order(&mut self, val: bool) {
        if val {
            self.debug |= 1; 
        } else {
            self.debug &= !1;
        }
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