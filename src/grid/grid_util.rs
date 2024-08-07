use octa_force::glam::IVec2;
use crate::grid::{ChunkIndex, Grid, NodeIndex, ValueData};
use crate::node::Node;

pub fn mod_ivec2(v: IVec2, t: IVec2) -> IVec2 {
    IVec2::new(v.x % t.x, v.y % t.y)
}

impl Grid {

    pub fn get_chunk_pos_from_global_pos(&self, pos: IVec2) -> IVec2 {
        pos / self.chunk_size
    }

    pub fn get_in_chunk_pos_from_global_pos(&self, pos: IVec2) -> IVec2 {
        mod_ivec2(pos, self.chunk_size)
    }

    pub fn get_node_index_from_pos_in_chunk(&self, pos: IVec2) -> usize {
        (pos.x * self.chunk_size.x + pos.y) as usize
    }
    pub fn get_node_and_chunk_index_from_global_pos(&mut self, pos: IVec2) -> (ChunkIndex, NodeIndex) {
        let in_chunk_pos = self.get_in_chunk_pos_from_global_pos(pos);
        let node_index = self.get_node_index_from_pos_in_chunk(in_chunk_pos);

        let chunk_pos = self.get_chunk_pos_from_global_pos(pos);
        let chunk_index = self.get_chunk_index_from_chunk_pos(chunk_pos);

        (chunk_index, node_index)
    }

    pub fn get_chunk_index_from_chunk_pos(&mut self, pos: IVec2) -> usize {
        self.chunks.iter().position(|c| c.pos == pos).unwrap_or_else(|| {
            self.add_chunk(pos);
            self.chunks.len() - 1
        })
    }

    pub fn get_mut_node_from_chunk_and_node_index(&mut self, chunk_index: ChunkIndex, node_index: NodeIndex) -> &mut Node<ValueData> {
        &mut self.chunks[chunk_index].nodes[node_index]
    }
}