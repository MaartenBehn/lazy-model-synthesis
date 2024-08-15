use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::node::ValueIndex;

#[derive(Default, Clone)]
pub struct RandomDispatcher<FastIdentifier> {
    rng: Rng,
    add: Vec<(FastIdentifier, ValueIndex)>
}

impl<FastIdentifier> Dispatcher<FastIdentifier> for RandomDispatcher<FastIdentifier> {
    fn push_add(&mut self, fast_identifier: FastIdentifier, value_index: ValueIndex) {
        self.add.push((fast_identifier, value_index))
    }

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueIndex)> {
        if self.add.is_empty() {
            return None
        }

        let index = self.rng.usize(0..self.add.len());
        Some(self.add.swap_remove(index))
    }
}