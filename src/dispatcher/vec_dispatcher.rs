use std::collections::VecDeque;
use crate::dispatcher::Dispatcher;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::ValueNr;

#[derive(Default, Clone)]
pub struct VecDispatcher<FI: FastIdentifierT> {
    add: VecDeque<(FI, ValueNr)>,
    remove: VecDeque<(FI, ValueNr)>,
    select: VecDeque<(FI, ValueNr)>,
}

impl<FI: FastIdentifierT> Dispatcher<FI> for VecDispatcher<FI> {
    fn push_add(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.add.push_back((fast_identifier, value_nr))
    }

    fn pop_add(&mut self) -> Option<(FI, ValueNr)> {
        self.add.pop_front()
    }

    fn push_remove(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.remove.push_back((fast_identifier, value_nr))
    }

    fn pop_remove(&mut self) -> Option<(FI, ValueNr)> {
        self.remove.pop_front()
    }

    fn push_select(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        self.select.push_back((fast_identifier, value_nr))
    }

    fn pop_select(&mut self) -> Option<(FI, ValueNr)> {
        self.select.pop_front()
    }

    fn select_contains_node(&mut self, fast_identifier: FI, value_nr: ValueNr) -> bool {
        self.select.iter().find(|(i, v)| {
            *i == fast_identifier && *v == value_nr
        }).is_some()
    }
}