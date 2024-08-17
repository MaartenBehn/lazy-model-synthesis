use std::collections::VecDeque;
use crate::dispatcher::Dispatcher;
use crate::identifier::FastIdentifierT;
use crate::node::ValueIndex;
use crate::value::ValueNr;

#[derive(Default, Clone)]
pub struct VecDispatcher<FI: FastIdentifierT> {
    add: VecDeque<(FI, ValueNr)>,
    propagate: VecDeque<(FI, ValueNr)>,
    rest: VecDeque<(FI, ValueNr)>,
}

impl<FI: FastIdentifierT> Dispatcher<FI> for VecDispatcher<FI> {
    fn push_add(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.add.push_back((fast_identifier, value_nr))
    }

    fn pop_add(&mut self) -> Option<(FI, ValueNr)> {
        self.add.pop_front()
    }

    fn push_remove(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.propagate.push_back((fast_identifier, value_nr))
    }

    fn pop_remove(&mut self) -> Option<(FI, ValueNr)> {
        self.propagate.pop_front()
    }

    fn push_select(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.rest.push_back((fast_identifier, value_nr))
    }

    fn pop_select(&mut self) -> Option<(FI, ValueNr)> {
        self.rest.pop_front()
    }
}