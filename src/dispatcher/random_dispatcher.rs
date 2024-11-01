use fastrand::Rng;
use crate::dispatcher::WFCDispatcherT;
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::value::ValueDataT;

#[derive(Default, Clone)]
pub struct RandomDispatcher<FI, VD> {
    rng: Rng,
    add: Vec<(FI, VD)>,
    remove: Vec<(FI, VD)>,
    select: Vec<(FI, VD)>,
}

impl<FI: FastIdentifierT, VD> RandomDispatcher<FI, VD> {
    fn push(list: &mut Vec<(FI, VD)>, fast_identifier: FI, value_data: VD) {
        list.push((fast_identifier, value_data))
    }

    fn pop(list: &mut Vec<(FI, VD)>, rng: &mut Rng) -> Option<(FI, VD)> {
        if list.is_empty() {
            return None
        }

        let index = rng.usize(0..list.len());
        Some(list.swap_remove(index))
    }
}

impl<FI: FastIdentifierT, VD: ValueDataT> WFCDispatcherT<FI, VD> for RandomDispatcher<FI, VD> {
    fn push_add(&mut self, fast_identifier: FI, value_data: VD) {
        Self::push(&mut self.add, fast_identifier, value_data)
    }

    fn pop_add(&mut self) -> Option<(FI, VD)> {
        Self::pop(&mut self.add, &mut self.rng)
    }

    fn push_remove(&mut self, fast_identifier: FI, value_data: VD) {
        Self::push(&mut self.remove, fast_identifier, value_data)
    }

    fn pop_remove(&mut self) -> Option<(FI, VD)> {
        Self::pop(&mut self.remove, &mut self.rng)
    }

    fn push_select(&mut self, fast_identifier: FI, value_data: VD) {
        self.select.push((fast_identifier, value_data))
    }

    fn pop_select(&mut self) -> Option<(FI, VD)> {
        if self.select.is_empty() {
            return None
        }

        let index = self.rng.usize(0..self.select.len());
        Some(self.select.swap_remove(index))
    }

    fn select_contains_node(&mut self, fast_identifier: FI, value_nr: VD) -> bool {
        self.select.iter().find(|(i, v)| {
            *i == fast_identifier && *v == value_nr
        }).is_some()
    }
}