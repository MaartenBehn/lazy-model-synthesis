use std::marker::PhantomData;
use octa_force::log::{debug, error, info, warn};
use octa_force::puffin_egui::puffin;
use crate::depth_search::depth_tree::{DepthTreeController, DepthTreeNode, ReqAtIdentifier, DepthIndex};
use crate::depth_search::node::DepthNode;
use crate::depth_search::value::DepthValue;
use crate::dispatcher::{DepthTreeDispatcherT, WFCDispatcherT, TREE_APPLY_INDEX, TREE_BUILD_INDEX, TREE_CHECK_INDEX};
use crate::general_data_structure::node_storage::NodeStorageT;
use crate::general_data_structure::identifier::{FastIdentifierT, GeneralIdentifierT, PackedIdentifierT};
use crate::util::state_saver::State;
use crate::general_data_structure::req_by::{ValueReqByPacker};
use crate::general_data_structure::value::{ValueDataT, ValueT};

#[derive(Default, Clone)]
pub struct DepthNodeManager<N, TreeD, GI, FI, PI, VD, const DEBUG: bool>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        TreeD: DepthTreeDispatcherT,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT,
{
    phantom_data0: PhantomData<GI>,
    phantom_data1: PhantomData<FI>,
    phantom_data2: PhantomData<PI>,
    phantom_data3: PhantomData<VD>,

    pub req_by_packer: ValueReqByPacker,
    
    pub node_storage: N,

    pub tree_dispatcher: TreeD,
    pub depth_tree_controller: DepthTreeController<FI, VD>
}

impl<N, TreeD, GI, FI, PI, VD, const DEBUG: bool> DepthNodeManager<N, TreeD, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        TreeD: DepthTreeDispatcherT,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT,
{
    pub fn new(mut node_storage: N, max_num_values: usize, max_num_reqs: usize) -> Self {
        DepthNodeManager {
            req_by_packer: ValueReqByPacker::new(max_num_values, max_num_reqs),
            node_storage,
            .. Default::default()
        }
    }

    pub fn select_value(&mut self, identifier: GI, value_data: VD) {
        let fast_identifier = self.node_storage.fast_from_general(identifier);
        
        self.start_search(fast_identifier, value_data);
    }

    pub fn start_search(&mut self, fast_identifier: FI, value_data: VD) {
        self.depth_tree_controller.reset();
        
        self.depth_tree_controller.nodes.push(DepthTreeNode::new(fast_identifier, value_data, 0));
        
        let node = self.node_storage.get_node_mut(fast_identifier);
        node.tree_nodes = vec![(value_data, 0)];
        node.fixed_value = Some(value_data);
        
        self.tree_dispatcher.push_tree_build_tick(0);
        self.tree_dispatcher.push_tree_apply_tick(0);
        
        if DEBUG {
            self.node_storage.on_add_value_callback(fast_identifier, value_data);

            self.node_storage.on_push_queue_callback(fast_identifier, value_data, TREE_BUILD_INDEX);
            self.node_storage.on_push_queue_callback(fast_identifier, value_data, TREE_APPLY_INDEX);
        }
    }

    
    pub fn tick(&mut self) -> bool {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        let needs_further_ticks = if let Some(index) = self.tree_dispatcher.pop_tree_check_tick() {
            self.tree_check_tick(index);
            true
        } else if let Some(index) = self.tree_dispatcher.pop_tree_build_tick() {
            self.tree_build_tick(index);
            true
        } else if let Some(index) = self.tree_dispatcher.pop_tree_apply_tick() {
            self.tree_apply_tick(index);
            true
        } else {
            false
        };
        
        if DEBUG {
            self.send_next_processed_node();
        }

        needs_further_ticks
    }

    pub fn tree_build_tick(&mut self, index: DepthIndex) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        let mut depth_tree_node = self.depth_tree_controller.nodes[index].clone();
        depth_tree_node.build = true;

        if DEBUG {
            self.node_storage.on_pop_queue_callback(depth_tree_node.fast_identifier, depth_tree_node.value_data, TREE_BUILD_INDEX);
            let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
            info!("Tree Build: {:?}", identifier);
        }
        
        let mut needs_tick_nodes = vec![];
        
        let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
        let num_reqs = self.node_storage.get_num_reqs_for_value_data(&depth_tree_node.value_data);
        for req_index in 0..num_reqs {
            let req = self.node_storage.get_req_for_value_data(&depth_tree_node.value_data, req_index);

            let req_identifier = self.node_storage.get_req_node_identifier(identifier, &req);

            if !self.node_storage.is_identifier_valid(req_identifier) {
                // The node is out of bounds -> Skip requirement
                continue
            }

            let req_fast_identifier = self.node_storage.fast_from_general(req_identifier);
            let mut req_node = self.node_storage.get_node(req_fast_identifier).clone();

            let mut req_nodes_at = ReqAtIdentifier::new(req_fast_identifier);
            let req_node_at_index = depth_tree_node.reqs.len();
            
            if req_node.fixed_value.is_some() {
                if !N::value_nr_matches_req(req_node.fixed_value.unwrap(), &req) {
                    // A Req points to a node that is fixed, but the fixed value is not possible
                    // -> This node is not possible
                    depth_tree_node.possible = false;
                    
                    // Check all incoming edges
                    for (depth_index, _, _) in depth_tree_node.req_by.iter() {
                        self.tree_dispatcher.push_tree_check_tick(*depth_index);
                        
                        if DEBUG {
                            let tree_node = &self.depth_tree_controller.nodes[*depth_index];
                            self.node_storage.on_push_queue_callback(tree_node.fast_identifier, tree_node.value_data, TREE_CHECK_INDEX);
                        }
                    }
                    break;
                } else {
                    
                    let req_value_data =  req_node.fixed_value.unwrap();
                    let req_tree_node_index = self.get_req_tree_node_index(
                        &mut req_node, 
                        req_fast_identifier,
                        req_value_data, 
                        &depth_tree_node);
                    
                }
            }
            
            

            let mut fixed_req_node_index = None;
            let mut already_chosen_req_node_index = None;
            let mut satisfied_req_node_index = None;
            
            let num_possible_value_data = N::get_num_possible_value_data_for_req(&req);
            for req_value_data_index in 0..num_possible_value_data {
                let req_value_data = N::get_value_data_for_req(&req, req_value_data_index);
                
                let req_tree_node_index = self.get_req_tree_node_index(&mut req_node, req_fast_identifier, req_value_data, &depth_tree_node);

                let mut req_tree_node = self.depth_tree_controller.nodes[req_tree_node_index].clone();

                let req_at_node_index = req_nodes_at.tree_nodes.len();
                
                // Check if the node has been chosen
                if already_chosen_req_node_index.is_none() {
                    for (test_tree_index, test_req_at_index, test_req_index_in_req_at) in req_tree_node.req_by.iter() {
                        let test_req_at = &self.depth_tree_controller.nodes[*test_tree_index].reqs[*test_req_at_index];

                        if test_req_at.chosen_index == Some(*test_req_index_in_req_at) {
                            already_chosen_req_node_index = Some(req_at_node_index);
                            break
                        }
                    }
                }
                
                if req_node.value == Some(DepthValue::new(req_tree_node.value_data)) {
                    req_tree_node.satisfied = true;
                    
                    if satisfied_req_node_index.is_none() {
                        satisfied_req_node_index = Some(req_at_node_index);
                    }
                }

                // Mark this req at as incoming edge
                req_tree_node.req_by.push((index, req_node_at_index, req_at_node_index));
                req_nodes_at.tree_nodes.push((req_value_data, req_tree_node_index));

                
                self.depth_tree_controller.nodes[req_tree_node_index] = req_tree_node;
            }
            
            // Choose the node for this req at
            let chosen_req_node_index = if fixed_req_node_index.is_some() {
                fixed_req_node_index.unwrap()
            } else if already_chosen_req_node_index.is_some() {
                already_chosen_req_node_index.unwrap()
            } else if satisfied_req_node_index.is_some() {
                satisfied_req_node_index.unwrap()
            } else {
                // Mark as needs further work
                needs_tick_nodes.push((req_nodes_at.tree_nodes[0].1, req_nodes_at.tree_nodes.len()));
                
                // Just choose the first
                0
            };
            req_nodes_at.chosen_index = Some(chosen_req_node_index);
            
            depth_tree_node.reqs.push(req_nodes_at);

            self.node_storage.set_node(req_fast_identifier, req_node);
        }

        self.depth_tree_controller.nodes[index] = depth_tree_node;

        needs_tick_nodes.sort_by(|(_, a), (_, b)| a.cmp(&b));
        
        for (needs_tick_index, _) in needs_tick_nodes {
            let tree_node = &self.depth_tree_controller.nodes[needs_tick_index];
            if tree_node.build {
                continue
            }
            
            self.tree_dispatcher.push_tree_build_tick(needs_tick_index);
            
            if DEBUG {
                self.node_storage.on_push_queue_callback(tree_node.fast_identifier, tree_node.value_data, TREE_BUILD_INDEX)
            }
        }
    }
    
    pub fn get_req_tree_node_index(
        &mut self, 
        req_node: &mut DepthNode<VD>,
        req_fast_identifier: FI,
        req_value_data: VD,  
        depth_tree_node: &DepthTreeNode<FI, VD>) -> usize {
        
        let res = req_node.tree_nodes
            .binary_search_by(|(data, _)| {data.get_value_nr().cmp(&req_value_data.get_value_nr())});

        if res.is_err() {
            let req_tree_node_index = self.depth_tree_controller.nodes.len();
            self.depth_tree_controller.nodes.push(DepthTreeNode::new(req_fast_identifier, req_value_data, depth_tree_node.level + 1));

            if DEBUG {
                self.node_storage.on_add_value_callback(req_fast_identifier, req_value_data);
            }

            // And mark it the identifier_node
            req_node.tree_nodes.insert(res.unwrap_err(), (req_value_data, req_tree_node_index));

            req_tree_node_index
        } else {
            req_node.tree_nodes[res.unwrap()].1
        }
    }

    pub fn tree_check_tick(&mut self, index: DepthIndex) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }

        let mut depth_tree_node = self.depth_tree_controller.nodes[index].clone();

        if DEBUG {
            self.node_storage.on_pop_queue_callback(depth_tree_node.fast_identifier, depth_tree_node.value_data, TREE_CHECK_INDEX);
            let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
            info!("Tree Check: {:?}", identifier);
        }
        
        
    }

    pub fn tree_apply_tick(&mut self, index: DepthIndex) {
        let mut depth_tree_node = self.depth_tree_controller.nodes[index].clone();
        depth_tree_node.applied = true;

        if DEBUG {
            self.node_storage.on_pop_queue_callback(depth_tree_node.fast_identifier, depth_tree_node.value_data, TREE_APPLY_INDEX);
            let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
            info!("Tree Apply: {:?}", identifier);
        }

        let mut node = self.node_storage.get_node(depth_tree_node.fast_identifier).clone();
        if DEBUG {
            for (value_nr, index) in node.tree_nodes.iter() {
                let depth_node = &self.depth_tree_controller.nodes[*index];
                self.node_storage.on_remove_value_callback(depth_node.fast_identifier, *value_nr);
            }
        }
        node.tree_nodes.clear();

        node.value = Some(DepthValue::new(depth_tree_node.value_data));

        if DEBUG {
            self.node_storage.on_select_value_callback(depth_tree_node.fast_identifier, depth_tree_node.value_data);
        }
        self.node_storage.set_node(depth_tree_node.fast_identifier, node);

        for req_at in depth_tree_node.reqs.iter() {
            let chosen_node_index = req_at.tree_nodes[req_at.chosen_index.unwrap()].1;
            let tree_node = &self.depth_tree_controller.nodes[chosen_node_index];
            if !tree_node.applied && !self.tree_dispatcher.apply_contains_node(chosen_node_index) {
                self.tree_dispatcher.push_tree_apply_tick(chosen_node_index);
                
                if DEBUG {
                    self.node_storage.on_push_queue_callback(tree_node.fast_identifier, tree_node.value_data, TREE_APPLY_INDEX)
                }
            }
        }

        self.depth_tree_controller.nodes[index] = depth_tree_node;
    }
    
    /// For Debugging 
    /// Sends the fast identifier of the node that will be processed in the next tick.
    fn send_next_processed_node(&mut self) {
        let mut tree_dispatcher = self.tree_dispatcher.clone();
        
        if let Some(index) = tree_dispatcher.pop_tree_build_tick() {
            let depth_node = &self.depth_tree_controller.nodes[index];
            self.node_storage.next_processed_node(Some(depth_node.fast_identifier));
        } else if let Some(index) = tree_dispatcher.pop_tree_apply_tick() {
            let depth_node = &self.depth_tree_controller.nodes[index];
            self.node_storage.next_processed_node(Some(depth_node.fast_identifier));
        } else {
            self.node_storage.next_processed_node(None);
        }
    }
}

impl<N, TreeD, GI, FI, PI,  VD, const DEBUG: bool> State for DepthNodeManager<N, TreeD, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        TreeD: DepthTreeDispatcherT,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT {
    fn tick_state(&mut self) -> bool {
        self.tick()
    }
}

