

use std::iter::repeat_with;
use fastrand::Rng;
use octa_force::glam::IVec2;
use crate::go_back_in_time::node::GoBackNode;
use crate::general_data_structure::node_storage::NodeStorageT;
use crate::grid::rules::{NeighborReq, NUM_VALUES, Rule, ValueType};
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos, PackedChunkNodeIndex};
use crate::grid::render::node_render_data::NodeRenderData;
use crate::util::get_num_bits_for_number;
use crate::general_data_structure::{ValueDataT, ValueNr};
use crate::general_data_structure::node::{NodeT, ValueIndex};

pub type ChunkIndex = usize;
pub type NodeIndex = usize;

#[derive(Clone)]
#[derive(Default)]
pub struct Grid<NO: NodeT<ValueData>> {
    pub chunk_size: IVec2,
    pub nodes_per_chunk: usize,
    pub bits_for_nodes_per_chunk: u32,
    pub mask_for_node_per_chunk: u32,

    pub last_processed_node: Option<ChunkNodeIndex>,
    pub chunks: Vec<Chunk<NO>>,
    pub rules: Vec<Rule>,

    pub rng: Rng,
}

#[derive(Clone)]
pub struct Chunk<NO: NodeT<ValueData>> {
    pub pos: IVec2,
    pub nodes: Vec<NO>,
    pub render_data: Vec<NodeRenderData>
}

#[derive(Default, Copy, Clone)]
pub struct ValueData {
    value_type: ValueType,
}

impl<NO: NodeT<ValueData>> Grid<NO> {
    pub fn new(chunk_size: usize) -> Self {
        let nodes_per_chunk = chunk_size * chunk_size;
        let bits_for_nodes_per_chunk = get_num_bits_for_number(nodes_per_chunk -1);
        let mask_for_node_per_chunk = (nodes_per_chunk -1) as u32;
        
        Grid {
            chunk_size: IVec2::ONE * chunk_size as i32,
            nodes_per_chunk,
            bits_for_nodes_per_chunk,
            mask_for_node_per_chunk,
            last_processed_node: None,
            chunks: vec![],
            rules: vec![],
            rng: Default::default(),
        }
    }

    pub fn add_chunk(&mut self, pos: IVec2) {
        self.chunks.push(Chunk {
            pos,
            nodes: repeat_with(|| NO::new(NUM_VALUES)).take(self.nodes_per_chunk).collect::<Vec<_>>(),
            render_data: vec![NodeRenderData::default(); self.nodes_per_chunk]
        })
    }
}

impl<NO: NodeT<ValueData>> NodeStorageT<GlobalPos, ChunkNodeIndex, PackedChunkNodeIndex, NO, ValueData> for Grid<NO> {
    
    type Req = NeighborReq;

    fn get_node(&self, fast_lookup: ChunkNodeIndex) -> &NO {
        &self.chunks[fast_lookup.chunk_index].nodes[fast_lookup.node_index]
    }
    fn get_node_mut(&mut self, fast_lookup: ChunkNodeIndex) -> &mut NO {
        &mut self.chunks[fast_lookup.chunk_index].nodes[fast_lookup.node_index]
    }
    
    fn get_num_reqs_for_value_data(&mut self, value_data: &ValueData) -> usize {
        self.rules[value_data.value_type as usize].neighbor_reqs.len()
    }

    fn get_req_for_value_data(&mut self, value_data: &ValueData, index: usize) -> Self::Req {
        self.rules[value_data.value_type as usize].neighbor_reqs[index].clone()
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

    fn get_num_possible_value_data_for_req(req: &Self::Req) -> usize {
        req.req_types.len()
    }

    fn get_value_data_for_req(req: &Self::Req, index: usize) -> ValueData {
        ValueData::new(req.req_types[index])
    }

    fn select_value_from_slice(&mut self, fast: ChunkNodeIndex) -> ValueIndex {
        let value_len = self.chunks[fast.chunk_index].nodes[fast.node_index].get_values().len();
        let value_index = self.rng.usize(0..value_len) as ValueIndex;
        
        value_index
    }

    // For Debugging
    fn on_add_value_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_value_type(value_nr, true);
    }

    fn on_remove_value_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_value_type(value_nr, false);
    }

    fn on_select_value_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_selected_value_type(value_nr);
    }

    fn on_push_add_queue_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_add_queue(value_nr,true);
    }

    fn on_pop_add_queue_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_add_queue(value_nr, false);
    }

    fn on_push_remove_queue_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_remove_queue(value_nr, true);
    }

    fn on_pop_remove_queue_callback(&mut self, fast: ChunkNodeIndex, value_nr: ValueNr) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_remove_queue(value_nr, false);
    }

    fn on_push_select_queue_callback(&mut self, fast: ChunkNodeIndex) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_select_queue(true);
    }

    fn on_pop_select_queue_callback(&mut self, fast: ChunkNodeIndex) {
        self.chunks[fast.chunk_index].render_data[fast.node_index].set_select_queue(false);
    }

    fn next_processed_node(&mut self, fast: Option<ChunkNodeIndex>) {
        if self.last_processed_node.is_some() {
            self.chunks[self.last_processed_node.unwrap().chunk_index]
                .render_data[self.last_processed_node.unwrap().node_index]
                .set_next(false);
        }

        if fast.is_some() {
            self.chunks[fast.unwrap().chunk_index]
                .render_data[fast.unwrap().node_index]
                .set_next(true);
            self.last_processed_node = fast;
        }

    }
}

impl ValueData {
    pub fn new(value_type: ValueType) -> ValueData {
        ValueData { value_type }
    }
}

impl ValueDataT for ValueData {
    fn get_value_nr(&self) -> ValueNr {
        self.value_type.into()
    }
}