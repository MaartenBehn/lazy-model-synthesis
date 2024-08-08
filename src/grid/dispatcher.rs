use octa_force::glam::IVec2;
use crate::dispatcher::Dispatcher;
use crate::grid::grid::{ChunkIndex, NodeIndex};
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct VecDispatcher {
    add: Vec<((ChunkIndex, NodeIndex), ValueIndex)>
}

impl Dispatcher<(ChunkIndex, NodeIndex)> for VecDispatcher {
    fn push_add(&mut self, fast_lookup: (ChunkIndex, NodeIndex), value_index: ValueIndex) {
        self.add.push((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<((ChunkIndex, NodeIndex), ValueIndex)> {
        self.add.pop()
    }
}