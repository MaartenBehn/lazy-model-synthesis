use std::collections::VecDeque;
use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::grid::grid::{ChunkIndex, NodeIndex};
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct VecDispatcher {
    add: VecDeque<((ChunkIndex, NodeIndex), ValueIndex)>
}

impl Dispatcher<(ChunkIndex, NodeIndex)> for VecDispatcher {
    fn push_add(&mut self, fast_lookup: (ChunkIndex, NodeIndex), value_index: ValueIndex) {
        self.add.push_back((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<((ChunkIndex, NodeIndex), ValueIndex)> {
        self.add.pop_front()
    }
}


#[derive(Default, Clone)]
pub struct RandomDispatcher {
    rng: Rng,
    add: Vec<((ChunkIndex, NodeIndex), ValueIndex)>
}

impl Dispatcher<(ChunkIndex, NodeIndex)> for RandomDispatcher {
    fn push_add(&mut self, fast_lookup: (ChunkIndex, NodeIndex), value_index: ValueIndex) {
        self.add.push((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<((ChunkIndex, NodeIndex), ValueIndex)> {
        if self.add.is_empty() {
            return None
        }
        
        let index = self.rng.usize(0..self.add.len());
        Some(self.add.swap_remove(index))
    }
}