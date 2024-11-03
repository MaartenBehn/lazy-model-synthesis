use std::collections::VecDeque;
use crate::depth_search::depth_tree::DepthTreeIndex;
use crate::dispatcher::{DepthTreeDispatcherT, WFCDispatcherT};
use crate::general_data_structure::identifier::FastIdentifierT;
use crate::general_data_structure::value::ValueDataT;

#[derive(Default, Clone)]
pub struct VecWFCDispatcher<FI: FastIdentifierT, VD: ValueDataT> {
    add: VecDeque<(FI, VD)>,
    remove: VecDeque<(FI, VD)>,
    select: VecDeque<(FI, VD)>,
}

#[derive(Default, Clone)]
pub struct VecTreeDispatcher {
    tree_check: VecDeque<DepthTreeIndex>,
    tree_build: VecDeque<DepthTreeIndex>,
    tree_apply: VecDeque<DepthTreeIndex>,
}

impl<FI: FastIdentifierT, VD: ValueDataT> WFCDispatcherT<FI, VD> for VecWFCDispatcher<FI, VD> {
    fn push_add(&mut self, fast_identifier: FI, value_data: VD) {
        self.add.push_back((fast_identifier, value_data))
    }

    fn pop_add(&mut self) -> Option<(FI, VD)> {
        self.add.pop_front()
    }

    fn push_remove(&mut self, fast_identifier: FI, value_data: VD) {
        self.remove.push_back((fast_identifier, value_data))
    }

    fn pop_remove(&mut self) -> Option<(FI, VD)> {
        self.remove.pop_front()
    }

    fn push_select(&mut self, fast_identifier: FI, value_data: VD) {
        self.select.push_back((fast_identifier, value_data))
    }

    fn pop_select(&mut self) -> Option<(FI, VD)> {
        self.select.pop_front()
    }

    fn select_contains_node(&mut self, fast_identifier: FI, value_data: VD) -> bool {
        self.select.iter().find(|(i, v)| {
            *i == fast_identifier && *v == value_data
        }).is_some()
    }
}

impl DepthTreeDispatcherT for VecTreeDispatcher {

    fn push_tree_check_tick(&mut self, tree_index: DepthTreeIndex) {
        self.tree_check.push_back(tree_index)
    }

    fn pop_tree_check_tick(&mut self) -> Option<DepthTreeIndex> {
        self.tree_check.pop_back()
    }
    fn push_tree_build_tick(&mut self, tree_index: DepthTreeIndex) {
        self.tree_build.push_back(tree_index)
    }

    fn pop_tree_build_tick(&mut self) -> Option<DepthTreeIndex> {
        self.tree_build.pop_back()
    }

    fn push_tree_apply_tick(&mut self, tree_index: DepthTreeIndex) {
        self.tree_apply.push_back(tree_index)
    }

    fn pop_tree_apply_tick(&mut self) -> Option<DepthTreeIndex> {
        self.tree_apply.pop_back()
    }

    fn apply_contains_node(&mut self, tree_index: DepthTreeIndex) -> bool {
        self.tree_apply.iter().find(|i| {
            **i == tree_index
        }).is_some()
    }
}