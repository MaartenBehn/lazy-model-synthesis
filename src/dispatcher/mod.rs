pub mod vec_dispatcher;
pub mod random_dispatcher;

use crate::general_data_structure::ValueNr;

pub trait Dispatcher<FastIdentifier>: Default + Clone {
    fn push_add(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn push_remove(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_remove(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn push_select(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_select(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn select_contains_node(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr) -> bool;
}


