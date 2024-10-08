use octa_force::glam::{IVec2, ivec2};
use crate::general_data_structure::node::NodeT;
use crate::go_back_in_time::node::GoBackNode;
use crate::grid::grid::{ChunkIndex, Grid, NodeIndex, ValueData};
use crate::grid::identifier::{ChunkNodeIndex, GlobalPos};

pub fn mod_ivec2(v: IVec2, t: IVec2) -> IVec2 {
    IVec2::new(v.x % t.x, v.y % t.y)
}

impl<NO: NodeT<ValueData>> Grid<NO> {

    pub fn get_chunk_pos_from_global_pos(&self, pos: GlobalPos) -> IVec2 {
        pos.0 / self.chunk_size
    }

    pub fn get_in_chunk_pos_from_global_pos(&self, pos: GlobalPos) -> IVec2 {
        mod_ivec2(pos.0, self.chunk_size)
    }

    pub fn get_node_index_from_pos_in_chunk(&self, pos: IVec2) -> NodeIndex {
        (pos.x * self.chunk_size.x + pos.y) as usize
    }

    pub fn get_pos_in_chunk_from_node_index(&self, index: NodeIndex) -> IVec2 {
        let x = index as i32 / self.chunk_size.x;
        let y = index as i32 % self.chunk_size.x;
        ivec2(x, y)
    }

    pub fn get_chunk_index_from_chunk_pos(&self, pos: IVec2) -> Option<usize> {
        self.chunks.iter().position(|c| c.pos == pos)
    }

    pub fn get_chunk_and_node_index_from_global_pos(&self, pos: GlobalPos) -> ChunkNodeIndex{
        let in_chunk_pos = self.get_in_chunk_pos_from_global_pos(pos);
        let node_index = self.get_node_index_from_pos_in_chunk(in_chunk_pos);

        let chunk_pos = self.get_chunk_pos_from_global_pos(pos);
        let chunk_index = self.get_chunk_index_from_chunk_pos(chunk_pos).unwrap();

        ChunkNodeIndex { chunk_index, node_index }
    }

    pub fn get_global_pos_from_chunk_and_node_index(&self, chunk_node_index: ChunkNodeIndex) -> GlobalPos {
        let chunk_pos = self.chunks[chunk_node_index.chunk_index].pos;
        let node_pos = self.get_pos_in_chunk_from_node_index(chunk_node_index.node_index);

        GlobalPos(chunk_pos + node_pos)
    }


    pub fn get_mut_node_from_chunk_and_node_index(&mut self, chunk_index: ChunkIndex, node_index: NodeIndex) -> &mut NO {
        &mut self.chunks[chunk_index].nodes[node_index]
    }
}