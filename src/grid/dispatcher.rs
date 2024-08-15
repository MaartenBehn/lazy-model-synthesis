use std::collections::VecDeque;
use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::grid::grid::{ChunkIndex, NodeIndex};
use crate::grid::identifier::ChunkNodeIndex;
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct VecDispatcher {
    add: VecDeque<(ChunkNodeIndex, ValueIndex)>
}

impl Dispatcher<ChunkNodeIndex> for VecDispatcher {
    fn push_add(&mut self, fast_lookup: ChunkNodeIndex, value_index: ValueIndex) {
        self.add.push_back((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<(ChunkNodeIndex, ValueIndex)> {
        self.add.pop_front()
    }
}


#[derive(Default, Clone)]
pub struct RandomDispatcher {
    rng: Rng,
    add: Vec<(ChunkNodeIndex, ValueIndex)>
}

impl Dispatcher<ChunkNodeIndex> for RandomDispatcher {
    fn push_add(&mut self, fast_lookup: ChunkNodeIndex, value_index: ValueIndex) {
        self.add.push((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<(ChunkNodeIndex, ValueIndex)> {
        if self.add.is_empty() {
            return None
        }
        
        let index = self.rng.usize(0..self.add.len());
        Some(self.add.swap_remove(index))
    }
}