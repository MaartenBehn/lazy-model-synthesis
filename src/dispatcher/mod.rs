pub mod vec_dispatcher;
pub mod random_dispatcher;

use crate::node::ValueIndex;

pub trait Dispatcher<FastIdentifier> {
    fn push_add(&mut self, fast_lookup: FastIdentifier, value_index: ValueIndex);

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueIndex)>;
}


