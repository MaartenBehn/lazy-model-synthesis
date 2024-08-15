pub mod vec_dispatcher;
pub mod random_dispatcher;

use crate::node::ValueIndex;

pub trait Dispatcher<FastIdentifier>: Default + Clone {
    fn push_add(&mut self, fast_identifier: FastIdentifier, value_index: ValueIndex);

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueIndex)>;

    fn push_propagate(&mut self, fast_identifier: FastIdentifier, value_index: ValueIndex);

    fn pop_propagate(&mut self) -> Option<(FastIdentifier, ValueIndex)>;

    fn push_reset(&mut self, fast_identifier: FastIdentifier, value_index: ValueIndex);

    fn pop_reset(&mut self) -> Option<(FastIdentifier, ValueIndex)>;
}


