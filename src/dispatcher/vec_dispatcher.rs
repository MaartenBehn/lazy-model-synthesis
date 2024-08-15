use std::collections::VecDeque;
use crate::dispatcher::Dispatcher;
use crate::identifier::FastIdentifierT;
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct VecDispatcher<FI: FastIdentifierT> {
    add: VecDeque<(FI, ValueIndex)>,
    propagate: VecDeque<(FI, ValueIndex)>,
    rest: VecDeque<(FI, ValueIndex)>,
}

impl<FI: FastIdentifierT> Dispatcher<FI> for VecDispatcher<FI> {
    fn push_add(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        self.add.push_back((fast_identifier, value_index))
    }

    fn pop_add(&mut self) -> Option<(FI, ValueIndex)> {
        self.add.pop_front()
    }

    fn push_propagate(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        self.propagate.push_back((fast_identifier, value_index))
    }

    fn pop_propagate(&mut self) -> Option<(FI, ValueIndex)> {
        self.propagate.pop_front()
    }

    fn push_reset(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        self.rest.push_back((fast_identifier, value_index))
    }

    fn pop_reset(&mut self) -> Option<(FI, ValueIndex)> {
        self.rest.pop_front()
    }
}