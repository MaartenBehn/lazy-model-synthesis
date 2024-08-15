use std::collections::VecDeque;
use crate::dispatcher::Dispatcher;
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct VecDispatcher<FastIdentifier> {
    add: VecDeque<(FastIdentifier, ValueIndex)>
}

impl<FastIdentifier> Dispatcher<FastIdentifier> for VecDispatcher<FastIdentifier> {
    fn push_add(&mut self, fast_lookup: FastIdentifier, value_index: ValueIndex) {
        self.add.push_back((fast_lookup, value_index))
    }

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueIndex)> {
        self.add.pop_front()
    }
}