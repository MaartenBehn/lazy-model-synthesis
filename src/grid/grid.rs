

use std::iter::repeat_with;
use octa_force::glam::IVec2;
use crate::dispatcher::Dispatcher;
use crate::grid::dispatcher::VecDispatcher;
use crate::grid::node_render_data::NodeRenderData;
use crate::grid::rules::{NeighborReq, Rule, ValueType};
use crate::node::Node;
use crate::node_storage::NodeStorage;

pub type ChunkIndex = usize;
pub type NodeIndex = usize;
pub const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
pub struct Grid {
    pub chunk_size: IVec2,
    pub chunks: Vec<Chunk>,
    pub rules: Vec<Rule>,
    pub dispatcher: VecDispatcher,
}

#[derive(Clone)]
pub struct Chunk {
    pub pos: IVec2,
    pub nodes: Vec<Node<ValueData>>,
    pub render_data: Vec<NodeRenderData>
}

#[derive(Copy, Clone)]
pub struct ValueData {
    value_type: ValueType,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            chunk_size: IVec2::ONE * CHUNK_SIZE as i32,
            chunks: vec![],
            rules: vec![],
            dispatcher: VecDispatcher::default()
        }
    }

    pub fn add_chunk(&mut self, pos: IVec2) {
        /*
        let mut node_render_data = NodeRenderData::default();
        node_render_data.set_selector(true);
        node_render_data.set_add_queue(true);
        node_render_data.set_propagate_queue(true);
        node_render_data.set_reset_queue(true);
        node_render_data.set_value_type(ValueType::Stone, true);
        node_render_data.set_value_type(ValueType::Grass, true);
        node_render_data.set_value_type(ValueType::Sand, true);
        node_render_data.set_selected_value_type(ValueType::Stone);
        */

        self.chunks.push(Chunk {
            pos,
            nodes: repeat_with(|| Node::default()).take(CHUNK_SIZE * CHUNK_SIZE).collect::<Vec<_>>(),
            render_data: vec![NodeRenderData::default(); CHUNK_SIZE * CHUNK_SIZE]
        })
    }

    pub fn get_rule_for_value_type(&self, value_type: ValueType) -> Rule {
        self.rules[value_type as usize].to_owned()
    }
}

impl NodeStorage for Grid {

    type ValueData = ValueData;
    type NodeIdentifier = IVec2;
    type FastLookup = (ChunkIndex, NodeIndex);
    type Req = NeighborReq;

    fn get_dispatcher(&mut self) -> &mut impl Dispatcher<Self::FastLookup> {
        &mut self.dispatcher
    }

    fn get_fast_lookup_from_identifier(&mut self, node_pos: IVec2) -> Self::FastLookup {
        self.get_chunk_and_node_index_from_global_pos(node_pos)
    }

    fn get_identifier_from_fast_lookup(&mut self, fast_lookup: Self::FastLookup) -> Self::NodeIdentifier {
        self.get_global_pos_from_chunk_and_node_index(fast_lookup.0, fast_lookup.1)
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

    // For Debugging
    fn on_add_value_callback(&mut self, fast_node_lookup: Self::FastLookup, value_data: ValueData) {
        self.chunks[fast_node_lookup.0].render_data[fast_node_lookup.1].set_value_type(value_data.value_type, true);
    }

    fn on_remove_value_callback(&mut self, fast_node_lookup: Self::FastLookup, value_data: ValueData) {
        self.chunks[fast_node_lookup.0].render_data[fast_node_lookup.1].set_value_type(value_data.value_type, false);
    }

    fn on_push_add_queue_callback(&mut self, fast_node_lookup: Self::FastLookup, value_data: ValueData) {
        self.chunks[fast_node_lookup.0].render_data[fast_node_lookup.1].set_add_queue(value_data.value_type,true);
    }

    fn on_pop_add_queue_callback(&mut self, fast_node_lookup: Self::FastLookup, value_data: ValueData) {
        self.chunks[fast_node_lookup.0].render_data[fast_node_lookup.1].set_add_queue(value_data.value_type, false);
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
        grid.add_chunk(IVec2::ZERO);
        grid.rules = get_example_rules();
        grid.add_initial_value(IVec2::new(1, 1), ValueData::new(ValueType::Stone));

        while grid.tick() {}

        println!("Done")
    }
}