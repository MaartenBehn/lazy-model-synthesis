use ultraviolet::IVec2;
use crate::dispatcher::Dispatcher;
use crate::node::ValueIndex;

#[derive(Default)]
pub struct VecDispatcher {
    add: Vec<(IVec2, ValueIndex)>
}

impl Dispatcher<IVec2> for VecDispatcher {
    fn push_add(&mut self, node_pos: IVec2, value_index: ValueIndex) {
        self.add.push((node_pos, value_index))
    }

    fn get_add(&mut self) -> Option<(IVec2, ValueIndex)> {
        self.add.pop()
    }
}