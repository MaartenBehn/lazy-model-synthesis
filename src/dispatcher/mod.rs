pub mod vec_dispatcher;
pub mod random_dispatcher;

use crate::depth_search::depth_tree::DepthTreeIndex;

pub const ADD_INDEX: usize = 1;
pub const REMOVE_INDEX: usize = 2;
pub const SELECT_INDEX: usize = 3;

pub const TREE_CHECK_INDEX: usize = 1;
pub const TREE_BUILD_INDEX: usize = 2;
pub const TREE_APPLY_INDEX: usize = 3;

pub trait WFCDispatcherT<FI, VD>: Default + Clone {
    fn push_add(&mut self, fast_identifier: FI, value_data: VD);

    fn pop_add(&mut self) -> Option<(FI, VD)>;

    fn push_remove(&mut self, fast_identifier: FI, value_data: VD);

    fn pop_remove(&mut self) -> Option<(FI, VD)>;

    fn push_select(&mut self, fast_identifier: FI, value_data: VD);

    fn pop_select(&mut self) -> Option<(FI, VD)>;

    fn select_contains_node(&mut self, fast_identifier: FI, value_data: VD) -> bool;
}

pub trait DepthTreeDispatcherT: Default + Clone {
    fn push_tree_check_tick(&mut self, tree_index: DepthTreeIndex);
    fn pop_tree_check_tick(&mut self) -> Option<DepthTreeIndex>;
    fn push_tree_build_tick(&mut self, tree_index: DepthTreeIndex);
    fn pop_tree_build_tick(&mut self) -> Option<DepthTreeIndex>;
    fn push_tree_apply_tick(&mut self, tree_index: DepthTreeIndex);
    fn pop_tree_apply_tick(&mut self) -> Option<DepthTreeIndex>;
    fn apply_contains_node(&mut self, tree_index: DepthTreeIndex) -> bool;
}



