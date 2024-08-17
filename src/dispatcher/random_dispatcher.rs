use fastrand::Rng;
use crate::dispatcher::Dispatcher;
use crate::identifier::FastIdentifierT;
use crate::value::ValueNr;

#[derive(Default, Clone)]
pub struct RandomDispatcher<FI> {
    rng: Rng,
    add: Vec<(FI, ValueNr)>,
    remove: Vec<(FI, ValueNr)>,
    select: Vec<(FI, ValueNr)>,
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

    fn push_select(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        Self::push(&mut self.select, fast_identifier, value_nr)
    }

    fn pop_select(&mut self) -> Option<(FI, ValueNr)> {
        Self::pop(&mut self.select, &mut self.rng)
    }
}