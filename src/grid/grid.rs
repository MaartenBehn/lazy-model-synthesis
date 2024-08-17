

use std::iter::repeat_with;
use octa_force::glam::IVec2;
use crate::dispatcher::vec_dispatcher::VecDispatcher;
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos, PackedChunkNodeIndex};
use crate::grid::rules::{NeighborReq, NUM_VALUES, Rule, ValueType};
use crate::grid::visulation::node_render_data::NodeRenderData;
use crate::node::Node;
use crate::node_storage::NodeStorage;
use crate::util::get_num_bits_for_number;
use crate::util::state_saver::State;
use crate::value::ValueDataT;

pub type ChunkIndex = usize;
pub type NodeIndex = usize;
pub const CHUNK_SIZE: usize = 32;

#[derive(Clone)]
#[derive(Default)]
pub struct Grid {
    pub chunk_size: IVec2,
    pub nodes_per_chunk: usize,
    pub bits_for_nodes_per_chunk: u32,
    pub mask_for_node_per_chunk: u32,
    
    pub chunks: Vec<Chunk>,
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct Chunk {
    pub pos: IVec2,
    pub nodes: Vec<Node<ValueData>>,
    pub render_data: Vec<NodeRenderData>
}

#[derive(Default, Copy, Clone)]
pub struct ValueData {
    value_type: ValueType,
}

impl Grid {
    pub fn new() -> Self {
        let chunk_size = CHUNK_SIZE;
        let nodes_per_chunk = chunk_size * chunk_size;
        let bits_for_nodes_per_chunk = get_num_bits_for_number(nodes_per_chunk -1);
        let mask_for_node_per_chunk = (nodes_per_chunk -1) as u32;
        
        Grid {
            chunk_size: IVec2::ONE * chunk_size as i32,
            nodes_per_chunk,
            bits_for_nodes_per_chunk,
            mask_for_node_per_chunk,
            chunks: vec![],
            rules: vec![],
        }
    }

    pub fn add_chunk(&mut self, pos: IVec2) {
        self.chunks.push(Chunk {
            pos,
            nodes: repeat_with(|| Node::new(NUM_VALUES)).take(self.nodes_per_chunk).collect::<Vec<_>>(),
            render_data: vec![NodeRenderData::default(); self.nodes_per_chunk]
        })
    }

    pub fn get_rule_for_value_type(&self, value_type: ValueType) -> Rule {
        self.rules[value_type as usize].to_owned()
    }
}

impl NodeStorage<GlobalPos, ChunkNodeIndex, PackedChunkNodeIndex, ValueData> for Grid {
    
    type Req = NeighborReq;
    type ShuffleSeed = usize;
    
    fn get_mut_node(&mut self, fast_lookup: ChunkNodeIndex) -> &mut Node<ValueData> {
        &mut self.chunks[fast_lookup.chunk_index].nodes[fast_lookup.node_index]
    }

    fn get_reqs_for_value_data(&mut self, value_data: &ValueData) -> Vec<Self::Req> {
        self.get_rule_for_value_type(value_data.value_type).neighbor_reqs.clone()
    }

    fn get_req_node_identifier(&mut self, original_identifier: GlobalPos, req: &NeighborReq) -> GlobalPos {
        GlobalPos(original_identifier.0 + req.offset)
    }

    fn is_identifier_valid(&self, node_pos: GlobalPos) -> bool {
        node_pos.0.x >= 0 && node_pos.0.y >= 0 && node_pos.0.x < self.chunk_size.x && node_pos.0.y < self.chunk_size.y
    }

    fn value_data_matches_req(value_data: &ValueData, req: &Self::Req) -> bool {
        req.req_types.iter().find(|t| {
            value_data.value_type == **t
        }).is_some()
    }

    fn get_possible_value_data_for_req(req: Self::Req) -> Vec<ValueData> {
        req.req_types.to_owned().into_iter().map(|t| {ValueData::new(t)}).collect()
    }

    // For Debugging
    fn on_add_value_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_value_type(value_data.value_type, true);
    }

    fn on_remove_value_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_value_type(value_data.value_type, false);
    }

    fn on_select_value_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_selected_value_type(value_data.value_type);
    }

    fn on_unselect_value_callback(&mut self, fast: ChunkNodeIndex, _: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].unselected_value_type();
    }

    fn on_push_add_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_add_queue(value_data.value_type,true);
    }

    fn on_pop_add_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_add_queue(value_data.value_type, false);
    }

    fn on_push_remove_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_propagate_queue(value_data.value_type,true);
    }

    fn on_pop_remove_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_propagate_queue(value_data.value_type,false);
    }

    fn on_push_select_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_select_queue(value_data.value_type, true);
    }

    fn on_pop_select_queue_callback(&mut self, fast: ChunkNodeIndex, value_data: ValueData) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_select_queue(value_data.value_type, false);
    }
}

impl ValueData {
    pub fn new(value_type: ValueType) -> ValueData {
        ValueData { value_type }
    }
}

impl ValueDataT for ValueData {
    fn get_value_nr(&self) -> crate::value::ValueNr {
        self.value_type.into()
    }
}