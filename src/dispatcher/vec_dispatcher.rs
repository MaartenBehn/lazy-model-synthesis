use std::collections::VecDeque;
use crate::depth_search::depth_tree::DepthIndex;
use crate::dispatcher::{DepthTreeDispatcherT, WFCDispatcherT};
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::value::ValueNr;

#[derive(Default, Clone)]
pub struct VecWFCDispatcher<FI: FastIdentifierT> {
    add: VecDeque<(FI, ValueNr)>,
    remove: VecDeque<(FI, ValueNr)>,
    select: VecDeque<(FI, ValueNr)>,
}

#[derive(Default, Clone)]
pub struct VecTreeDispatcher {
    tree_build: VecDeque<DepthIndex>,
    tree_apply: VecDeque<DepthIndex>,
}

impl<FI: FastIdentifierT> WFCDispatcherT<FI> for VecWFCDispatcher<FI> {
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

impl DepthTreeDispatcherT for VecTreeDispatcher {
    fn push_tree_build_tick(&mut self, tree_index: DepthIndex) {
        self.tree_build.push_back(tree_index)
    }

    fn pop_tree_build_tick(&mut self) -> Option<DepthIndex> {
        self.tree_build.pop_back()
    }

    fn push_tree_apply_tick(&mut self, tree_index: DepthIndex) {
        self.tree_apply.push_back(tree_index)
    }

    fn pop_tree_apply_tick(&mut self) -> Option<DepthIndex> {
        self.tree_apply.pop_back()
    }
}