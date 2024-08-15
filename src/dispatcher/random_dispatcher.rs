use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::identifier::FastIdentifierT;
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct RandomDispatcher<FI> {
    rng: Rng,
    add: Vec<(FI, ValueIndex)>,
    propagate: Vec<(FI, ValueIndex)>,
    reset: Vec<(FI, ValueIndex)>,
}

impl<FI: FastIdentifierT> RandomDispatcher<FI> {
    fn push(list: &mut Vec<(FI, ValueIndex)>, fast_identifier: FI, value_index: ValueIndex) {
        list.push((fast_identifier, value_index))
    }

    fn pop(list: &mut Vec<(FI, ValueIndex)>, rng: &mut Rng) -> Option<(FI, ValueIndex)> {
        if list.is_empty() {
            return None
        }

        let index = rng.usize(0..list.len());
        Some(list.swap_remove(index))
    }
}

impl<FI: FastIdentifierT> Dispatcher<FI> for RandomDispatcher<FI> {
    fn push_add(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        Self::push(&mut self.add, fast_identifier, value_index)
    }

    fn pop_add(&mut self) -> Option<(FI, ValueIndex)> {
        Self::pop(&mut self.add, &mut self.rng)
    }

    fn push_propagate(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        Self::push(&mut self.propagate, fast_identifier, value_index)
    }

    fn pop_propagate(&mut self) -> Option<(FI, ValueIndex)> {
        Self::pop(&mut self.propagate, &mut self.rng)
    }

    fn push_reset(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        Self::push(&mut self.reset, fast_identifier, value_index)
    }

    fn pop_reset(&mut self) -> Option<(FI, ValueIndex)> {
        Self::pop(&mut self.reset, &mut self.rng)
    }
}