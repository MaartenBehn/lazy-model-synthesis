use ultraviolet::IVec2;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ValueType {
    Stone = 0,
    Grass,
    Sand,
}

#[derive(Clone)]
pub struct Rule {
    pub value_type: ValueType,
    pub neighbor_reqs: Vec<NeighborReq>,
}

#[derive(Clone, Copy)]
pub struct NeighborReq {
    pub req_type: ValueType,
    pub offset: IVec2,
}

pub fn get_example_rules() -> Vec<Rule> {
    vec![
        Rule {
            value_type: ValueType::Stone,
            neighbor_reqs: vec![
                NeighborReq {
                    req_type: ValueType::Stone,
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Stone,
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Stone,
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_type: ValueType::Stone,
                    offset: IVec2::new(0, -1),
                },
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(-1, 0),
                },
            ],
        },
        Rule {
            value_type: ValueType::Grass,
            neighbor_reqs: vec![
                NeighborReq {
                    req_type: ValueType::Stone,
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(0, -1),
                },
                NeighborReq {
                    req_type: ValueType::Sand,
                    offset: IVec2::new(-1, 0),
                },
            ],
        },

        Rule {
            value_type: ValueType::Sand,
            neighbor_reqs: vec![
                NeighborReq {
                    req_type: ValueType::Grass,
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Sand,
                    offset: IVec2::new(1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Sand,
                    offset: IVec2::new(-1, 0),
                },
                NeighborReq {
                    req_type: ValueType::Sand,
                    offset: IVec2::new(0, 1),
                },
                NeighborReq {
                    req_type: ValueType::Sand,
                    offset: IVec2::new(0, -1),
                },
            ],
        }
    ]
}

