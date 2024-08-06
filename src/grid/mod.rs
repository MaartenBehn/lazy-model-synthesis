mod rules;
mod grid_util;
mod dispatcher;

use std::iter::repeat_with;
use ultraviolet::{IVec2};
use crate::dispatcher::Dispatcher;
use crate::grid::dispatcher::VecDispatcher;
use crate::grid::rules::{NeighborReq, Rule, ValueType};
use crate::node::Node;
use crate::node_storage::NodeStorage;

type ChunkIndex = usize;
type NodeIndex = usize;
const CHUNK_SIZE: usize = 32;

pub struct Grid {
    chunk_size: IVec2,
    chunks: Vec<Chunk>,
    rules: Vec<Rule>,
    dispatcher: VecDispatcher,
}

pub struct Chunk {
    pos: IVec2,
    nodes: Vec<Node<ValueData>>,
}

#[derive(Clone)]
pub struct ValueData {
    value_type: ValueType,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            chunk_size: IVec2::one() * CHUNK_SIZE as i32,
            chunks: vec![],
            rules: vec![],
            dispatcher: VecDispatcher::default()
        }
    }

    pub fn add_chunk(&mut self, pos: IVec2) {
        self.chunks.push(Chunk {
            pos,
            nodes: repeat_with(|| Node::default()).take(CHUNK_SIZE * CHUNK_SIZE).collect::<Vec<_>>(),
        })
    }

    pub fn get_rule_for_value_type(&self, value_type: ValueType) -> Rule {
        self.rules[value_type as usize].to_owned()
    }
}

impl NodeStorage for Grid {

    type ValueData = ValueData;
    type NodeIdentifier = IVec2;
    type FastNodeLookup = (ChunkIndex, NodeIndex);
    type Req = NeighborReq;

    fn get_dispatcher(&mut self) -> &mut impl Dispatcher<IVec2> {
        &mut self.dispatcher
    }

    fn get_fast_lookup_of_node(&mut self, node_pos: IVec2) -> Self::FastNodeLookup {
        self.get_node_and_chunk_index_from_global_pos(node_pos)
    }

    fn get_mut_node_from_fast_lookup(&mut self, fast_lookup: (ChunkIndex, NodeIndex)) -> &mut Node<ValueData> {
        &mut self.chunks[fast_lookup.0].nodes[fast_lookup.1]
    }

    fn get_reqs_for_value_data(&mut self, value_data: &ValueData) -> Vec<Self::Req> {
        self.get_rule_for_value_type(value_data.value_type).neighbor_reqs.clone()
    }

    fn get_req_node_identifier(&mut self, original_identifier: IVec2, req: &NeighborReq) -> Self::NodeIdentifier {
        original_identifier + req.offset
    }

    fn is_identifier_valid(&self, node_pos: IVec2) -> bool {
        node_pos.x >= 0 && node_pos.y >= 0 && node_pos.x < self.chunk_size.x && node_pos.y < self.chunk_size.y
    }

    fn value_data_matches_req(value_data: &Self::ValueData, req: &Self::Req) -> bool {
        value_data.value_type == req.req_type
    }

    fn get_value_data_for_req(req: Self::Req) -> Self::ValueData {
        ValueData::new(req.req_type)
    }
}

impl ValueData {
    pub fn new(value_type: ValueType) -> ValueData {
        ValueData { value_type }
    }
}



#[cfg(test)]
mod tests {
    use crate::grid::rules::get_example_rules;
    use super::*;

    #[test]
    fn example_grid() {
       let mut grid = Grid::new();
        grid.add_chunk(IVec2::zero());
        grid.rules = get_example_rules();
        grid.add_initial_value(IVec2::new(1, 1), ValueData::new(ValueType::Stone));

        while grid.tick() {}

        println!("Done")
    }
}