use crate::node::ValueIndex;

pub trait Dispatcher<NodeIdentifier> {
    fn push_add(&mut self, identifier: NodeIdentifier, value_index: ValueIndex);

    fn get_add(&mut self) -> Option<(NodeIdentifier, ValueIndex)>;
}