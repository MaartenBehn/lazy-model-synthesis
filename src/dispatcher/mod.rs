pub mod vec_dispatcher;
pub mod random_dispatcher;

use crate::depth_search::depth_tree::DepthIndex;
use crate::general_data_structure::value::ValueNr;

pub trait WFCDispatcherT<FastIdentifier>: Default + Clone {
    fn push_add(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_add(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn push_remove(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_remove(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn push_select(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr);

    fn pop_select(&mut self) -> Option<(FastIdentifier, ValueNr)>;

    fn select_contains_node(&mut self, fast_identifier: FastIdentifier, value_nr: ValueNr) -> bool;
}

pub trait DepthTreeDispatcherT: Default + Clone {
    fn push_tree_build_tick(&mut self, tree_index: DepthIndex);
    fn pop_tree_build_tick(&mut self) -> Option<DepthIndex>;
    fn push_tree_apply_tick(&mut self, tree_index: DepthIndex);
    fn pop_tree_apply_tick(&mut self) -> Option<DepthIndex>;
}



