use num_enum::{IntoPrimitive, TryFromPrimitive};
use octa_force::glam::IVec2;
use crate::value::Value;

pub const NUM_VALUES: usize = 3;
pub const NUM_REQS: usize = 8;


#[derive(Clone)]
pub struct Rule {
    pub value_type: Value,
    pub neighbor_reqs: Vec<NeighborReq>,
}

#[derive(Clone)]
pub struct NeighborReq {
    pub req_types: Vec<Value>,
    pub offset: IVec2,
}

/*
pub fn get_example_rules() -> Vec<Rule> {
    vec![
        Rule {
            value_type: Value::Stone,
            neighbor_reqs: vec![
                NeighborReq {
                    req_types: vec![Value::Stone, Value::Grass],
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Stone, Value::Grass],
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Stone],
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_types: vec![Value::Stone],
                    offset: IVec2::new(0, -1),
                },
            ],
        },
        Rule {
            value_type: Value::Grass,
            neighbor_reqs: vec![
                NeighborReq {
                    req_types: vec![Value::Grass, Value::Stone, Value::Sand],
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Grass, Value::Stone, Value::Sand],
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Grass],
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_types: vec![Value::Grass],
                    offset: IVec2::new(0, -1),
                },
            ],
        },
        Rule {
            value_type: Value::Sand,
            neighbor_reqs: vec![
                NeighborReq {
                    req_types: vec![Value::Grass, Value::Sand],
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Grass, Value::Sand],
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_types: vec![Value::Sand],
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_types: vec![Value::Sand],
                    offset: IVec2::new(0, -1),
                },
            ],
        },
    ]
}
 */


