use crate::node::ValueIndex;

pub trait Dispatcher<FastLookup> {
    fn push_add(&mut self, fast_lookup: FastLookup, value_index: ValueIndex);

    fn pop_add(&mut self) -> Option<(FastLookup, ValueIndex)>;
}