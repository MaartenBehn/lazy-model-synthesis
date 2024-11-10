use octa_force::glam::IVec2;
use crate::value::Value;

pub const NUM_VALUES: usize = 3;
pub const NUM_REQS: usize = 8;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Rule {
    pub value: Value,
    pub reqs: Vec<RuleReq>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RuleReq {
    pub reqs: Vec<(IVec2, Value)>
}

impl Rule {
    pub fn new(value: Value) -> Self {
        Rule{
            value,
            reqs: vec![],
        }
    } 
}

impl RuleReq {
    pub fn new() -> Self {
        RuleReq {
            reqs: vec![],
        }
    }
}
