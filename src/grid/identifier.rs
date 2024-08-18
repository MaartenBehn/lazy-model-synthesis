use octa_force::glam::{IVec2};
use crate::grid::grid::Grid;
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, IdentifierConverterT, PackedIdentifierT};


#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct GlobalPos(pub IVec2);

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct ChunkNodeIndex {
    pub chunk_index: usize,
    pub node_index: usize,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct PackedChunkNodeIndex(pub u32);

impl GeneralIdentifierT for GlobalPos {

}

impl FastIdentifierT for ChunkNodeIndex {

}

impl PackedIdentifierT for PackedChunkNodeIndex {
    fn to_bits(self) -> u32 { self.0 }

    fn from_bits(bits: u32) -> Self {
        PackedChunkNodeIndex(bits)
    }
}


impl IdentifierConverterT<GlobalPos, ChunkNodeIndex, PackedChunkNodeIndex> for Grid {
    fn fast_from_general(&mut self, i: GlobalPos) -> ChunkNodeIndex {
        self.get_chunk_and_node_index_from_global_pos(i)
    }

    fn general_from_fast(&mut self, i: ChunkNodeIndex) -> GlobalPos {
        self.get_global_pos_from_chunk_and_node_index(i)
    }

    fn packed_from_general(&mut self, i: GlobalPos) -> PackedChunkNodeIndex {
        let fast = self.fast_from_general(i);
        self.packed_from_fast(fast)
    }

    fn general_from_packed(&mut self, i: PackedChunkNodeIndex) -> GlobalPos {
        let fast = self.fast_from_packed(i);
        self.general_from_fast(fast)
    }

    fn packed_from_fast(&mut self, i: ChunkNodeIndex) -> PackedChunkNodeIndex {
        PackedChunkNodeIndex(((i.chunk_index << self.bits_for_nodes_per_chunk) + i.node_index) as u32)
    }

    fn fast_from_packed(&mut self, i: PackedChunkNodeIndex) -> ChunkNodeIndex {
        let chunk_index = (i.0 >> self.bits_for_nodes_per_chunk) as usize;
        let node_index = (i.0 & self.mask_for_node_per_chunk) as usize;

        ChunkNodeIndex{
            chunk_index,
            node_index,
        }
    }
}



