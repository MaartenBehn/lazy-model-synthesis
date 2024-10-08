use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::ValueNr;

#[derive(Default, Clone)]
pub struct RandomDispatcher<FI> {
    rng: Rng,
    add: Vec<(FI, ValueNr)>,
    remove: Vec<(FI, ValueNr)>,
    select: Vec<FI>,
}

impl<FI: FastIdentifierT> RandomDispatcher<FI> {
    fn push(list: &mut Vec<(FI, ValueNr)>, fast_identifier: FI, value_nr: ValueNr) {
        list.push((fast_identifier, value_nr))
    }

    fn pop(list: &mut Vec<(FI, ValueNr)>, rng: &mut Rng) -> Option<(FI, ValueNr)> {
        if list.is_empty() {
            return None
        }

        let index = rng.usize(0..list.len());
        Some(list.swap_remove(index))
    }
}

impl<FI: FastIdentifierT> Dispatcher<FI> for RandomDispatcher<FI> {
    fn push_add(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        Self::push(&mut self.add, fast_identifier, value_nr)
    }

    fn pop_add(&mut self) -> Option<(FI, ValueNr)> {
        Self::pop(&mut self.add, &mut self.rng)
    }

    fn push_remove(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        Self::push(&mut self.remove, fast_identifier, value_nr)
    }

    fn pop_remove(&mut self) -> Option<(FI, ValueNr)> {
        Self::pop(&mut self.remove, &mut self.rng)
    }

    fn push_select(&mut self, fast_identifier: FI) {
        self.select.push(fast_identifier)
    }

    fn pop_select(&mut self) -> Option<FI> {
        if self.select.is_empty() {
            return None
        }

        let index = self.rng.usize(0..self.select.len());
        Some(self.select.swap_remove(index))
    }

    fn select_contains_node(&mut self, fast_identifier: FI) -> bool {
        self.select.iter().find(|i| {
            **i == fast_identifier
        }).is_some()
    }
}