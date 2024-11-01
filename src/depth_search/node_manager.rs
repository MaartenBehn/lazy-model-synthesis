use std::marker::PhantomData;
use octa_force::log::{debug, error, info, warn};
use octa_force::puffin_egui::puffin;
use crate::depth_search::depth_tree::{DepthTreeController, DepthTreeNode, IdentifierNode, ReqAtIdentifier, DepthIndex};
use crate::depth_search::node::DepthNode;
use crate::depth_search::value::DepthValue;
use crate::dispatcher::{DepthTreeDispatcherT, WFCDispatcherT};
use crate::go_back_in_time::node::{HistoryIndex};
use crate::general_data_structure::node_storage::NodeStorageT;
use crate::general_data_structure::identifier::{FastIdentifierT, GeneralIdentifierT, PackedIdentifierT};
use crate::util::state_saver::State;
use crate::general_data_structure::node::NodeT;
use crate::general_data_structure::req::ValueReq;
use crate::general_data_structure::req_by::{ValueReqByPacker};
use crate::general_data_structure::value::{ValueDataT, ValueNr, ValueT};

#[derive(Default, Clone)]
pub struct DepthNodeManager<N, WFCD, TreeD, GI, FI, PI, VD, const DEBUG: bool>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        WFCD: WFCDispatcherT<FI>,
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
    pub wfc_dispatcher: WFCD,
    pub tree_dispatcher: TreeD,
    pub depth_tree_controller: DepthTreeController<FI, VD>
}

impl<N, WFCD, TreeD, GI, FI, PI, VD, const DEBUG: bool> DepthNodeManager<N, WFCD, TreeD, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        WFCD: WFCDispatcherT<FI>,
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

        if DEBUG {
            for (fast_identifier, _) in self.depth_tree_controller.identifier_nodes.iter() {
                self.node_storage.on_remove_depth_tree_identifier_callback(*fast_identifier);
            }
        }
        self.depth_tree_controller.reset();
        
        self.depth_tree_controller.identifier_nodes.insert(fast_identifier, IdentifierNode::new(vec![(value_data.get_value_nr(), 0)]));
        
        self.depth_tree_controller.nodes.push(DepthTreeNode::new(fast_identifier, value_data, 0));
        self.tree_dispatcher.push_tree_build_tick(0);
        self.tree_dispatcher.push_tree_apply_tick(0);
        
        if DEBUG {
            self.node_storage.on_add_depth_tree_identifier_callback(fast_identifier);
            self.node_storage.on_push_tree_build_queue_callback(fast_identifier, value_data.get_value_nr());
            self.node_storage.on_push_tree_apply_queue_callback(fast_identifier, value_data.get_value_nr());
        }
    }

    
    pub fn tick(&mut self) -> bool {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        let needs_further_ticks = if let Some((fast_identifier, value_nr)) = self.wfc_dispatcher.pop_remove() {
            self.remove_tick(fast_identifier, value_nr);
            true
        } else if let Some((fast_identifier, value_nr)) = self.wfc_dispatcher.pop_select() {
            self.select_tick(fast_identifier, value_nr);
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
    
    fn remove_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        puffin::profile_function!();

        if DEBUG {   // For debugging
            self.node_storage.on_pop_remove_queue_callback(fast_identifier, value_nr);
        }

        if DEBUG {
            let identifier = self.node_storage.general_from_fast(fast_identifier);
            info!("Remove: {:?}", identifier);
        }
        
        // Get the general_data_structure and general_data_structure data of the general_data_structure that will be removed
        let node = self.node_storage.get_node_mut(fast_identifier);
        
        let value_index = node.get_value_index_from_value_nr(value_nr);
        let value_index = if DEBUG && value_index.is_err() {
            let identifier = self.node_storage.general_from_fast(fast_identifier);
            warn!("Remove {:?} general_data_structure nr {:?} not found!",identifier, value_nr);
            return;
        } else {
            value_index.unwrap()
        };
        
        
        /*
        let req_by_len = node.values[value_index].req_by.len();

        // Go over values that require this value and check if they should also be removed.
        for j in 0..req_by_len {

            // Get the req by
            let node = self.node_storage.get_node_mut(fast_identifier);
            let req_by = node.values[value_index].req_by[j];

            // Identify the req that is linked to this general_data_structure
            let (req_packed_identifier, req_value_nr, req_index) = self.req_by_packer.unpack::<PI>(req_by);
            let req_fast_identifier = self.node_storage.fast_from_packed(req_packed_identifier);

            // Get the req node and find the general_data_structure
            let req_node = self.node_storage.get_node_mut(req_fast_identifier);
            let req_value_index = req_node.get_value_index_from_value_nr(req_value_nr);
            if req_value_index.is_err() {
                // The req general_data_structure was already removed -> Skip
                continue;
            }
            let req_value_index = req_value_index.unwrap();

            // Check if the general_data_structure that requires this general_data_structure should be removed
            let req_should_be_removed = req_node.values[req_value_index].reqs[req_index].on_remove_req_by();
            if req_should_be_removed {
                self.wfc_dispatcher.push_remove(req_fast_identifier, req_value_nr);

                if DEBUG {
                    self.node_storage.on_push_remove_queue_callback(req_fast_identifier, req_value_nr);
                }
            }
        }
        
         */

        let node = self.node_storage.get_node_mut(fast_identifier);
        node.values.remove(value_index);

        if node.values.len() == 1 {
            let value_data = node.values[0].value_data;
            let value_nr = value_data.get_value_nr();
            self.node_storage.on_select_value_callback(fast_identifier, value_nr);
        }
        
        if DEBUG {
            self.node_storage.on_remove_value_callback(fast_identifier, value_nr);
        }
    }

    fn select_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        if DEBUG {
            self.node_storage.on_pop_select_queue_callback(fast_identifier, value_nr);

            let identifier = self.node_storage.general_from_fast(fast_identifier);
            info!("Select: {:?}", identifier);
        }
        
        let value_index = self.node_storage.select_value_from_slice(fast_identifier);
        
        
        if DEBUG {
            let node = self.node_storage.get_node_mut(fast_identifier);
            let value_data = node.values[value_index].value_data;
            let value_nr = value_data.get_value_nr();
            self.node_storage.on_select_value_callback(fast_identifier, value_nr);
        }

        let node = self.node_storage.get_node_mut(fast_identifier);
        

        // Swap the selected general_data_structure into first place 
        // All other values will be later removed
        node.values.swap(0, value_index);
        
        
        // Go over all values that are not selected and will be removed
        for i in (1..node.values.len()).rev() {
            let other_value_nr = node.values[i].value_data.get_value_nr();
            self.wfc_dispatcher.push_remove(fast_identifier, other_value_nr)
        }

        /*
        let req_by_len = node.values[0].req_by.len();
        for i in 0..req_by_len {
            // Get the req by
            let node = self.node_storage.get_node_mut(fast_identifier);
            let req_by = node.values[0].req_by[i];

            // Identify the req that is linked to this value
            let (req_packed_identifier, req_value_nr, _) = self.req_by_packer.unpack::<PI>(req_by);
            let req_fast_identifier = self.node_storage.fast_from_packed(req_packed_identifier);

            let req_node = self.node_storage.get_node_mut(req_fast_identifier);
            if req_node.values.len() <= 1 {
                continue;
            }

            if self.wfc_dispatcher.select_contains_node(req_fast_identifier, value_nr) {
                continue;
            }

            self.wfc_dispatcher.push_select(req_fast_identifier, value_nr);

            if DEBUG {
                self.node_storage.on_push_select_queue_callback(req_fast_identifier, value_nr);
            }
        }
        
         */
    }

    pub fn tree_build_tick(&mut self, index: DepthIndex) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }

        let mut depth_tree_node = self.depth_tree_controller.nodes[index].clone();
        depth_tree_node.processed = true;

        if DEBUG {
            self.node_storage.on_pop_tree_build_queue_callback(depth_tree_node.fast_identifier, depth_tree_node.value_data.get_value_nr());
            let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
            info!("Tree Build: {:?}", identifier);
        }
        
        // The needed depth tree node value is not present in the node. 
        // Therefore, we need go over all reqs of the value and 
        // Add a Tree Node for the req value

        let mut needs_tick_nodes = vec![];
        

        let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
        let num_reqs = self.node_storage.get_num_reqs_for_value_data(&depth_tree_node.value_data);
        for req_at_index in 0..num_reqs {
            let req = self.node_storage.get_req_for_value_data(&depth_tree_node.value_data, req_at_index);

            let req_identifier = self.node_storage.get_req_node_identifier(identifier, &req);

            if !self.node_storage.is_identifier_valid(req_identifier) {
                // The node is out of bounds -> Skip requirement
                continue
            }

            let req_fast_identifier = self.node_storage.fast_from_general(req_identifier);

            let mut identifier_node = self.depth_tree_controller.identifier_nodes
                .get(&req_fast_identifier)
                .cloned()
                .unwrap_or(IdentifierNode {
                    tree_nodes: vec![],
                });

            let mut req_nodes_at = ReqAtIdentifier::new(req_fast_identifier);

            let mut already_chosen_req_node_index = None;
            let mut satisfied_req_node_index = None;
            
            let num_possible_value_data = N::get_num_possible_value_data_for_req(&req);
            for req_value_data_index in 0..num_possible_value_data {
                let req_value_data = N::get_value_data_for_req(&req, req_value_data_index);
                let req_value_nr = req_value_data.get_value_nr();
                let req_at_node_index = req_nodes_at.tree_nodes.len();

                let res = identifier_node.tree_nodes
                    .binary_search_by(|(nr, _)| {nr.cmp(&req_value_nr)});

                let req_tree_node_index = if res.is_err() {
                    let req_tree_node_index = self.depth_tree_controller.nodes.len();
                    self.depth_tree_controller.nodes.push(DepthTreeNode::new(req_fast_identifier, req_value_data, depth_tree_node.level + 1));
                    
                    // And mark it the identifier_node
                    identifier_node.tree_nodes.insert(res.unwrap_err(), (req_value_nr, req_tree_node_index));

                    req_tree_node_index
                } else {
                    identifier_node.tree_nodes[res.unwrap()].1
                };

                let req_node = &self.depth_tree_controller.nodes[req_tree_node_index];
                
                // Check if the node has been chosen
                if already_chosen_req_node_index.is_none() {
                    for (test_tree_index, test_req_at_index, test_req_index_in_req_at) in req_node.other_req.iter() {
                        let test_req_at = &self.depth_tree_controller.nodes[*test_tree_index].reqs[*test_req_at_index];

                        if test_req_at.chosen_index == Some(*test_req_index_in_req_at) {
                            already_chosen_req_node_index = Some(req_at_node_index);
                            break
                        }
                    }
                }

                let req_node = &mut self.depth_tree_controller.nodes[req_tree_node_index];
                // Check if the node is satisfied 
                if already_chosen_req_node_index.is_none() {
                    let node = self.node_storage.get_node(req_fast_identifier);
                    let value_index = node.get_value_index_from_value_nr(req_node.value_data.get_value_nr());
                    if value_index.is_ok() {
                        req_node.satisfied = true;
                        satisfied_req_node_index = Some(req_at_node_index);
                    }
                }

                // Mark this req at as incoming edge
                req_node.other_req.push((index, req_at_index, req_at_node_index));
                req_nodes_at.tree_nodes.push((req_value_nr, req_tree_node_index));
            }
            
            // Choose the node for this req at
            let chosen_req_node_index = if already_chosen_req_node_index.is_some() {
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

            self.depth_tree_controller.identifier_nodes.insert(req_fast_identifier, identifier_node);
            if DEBUG {
                self.node_storage.on_add_depth_tree_identifier_callback(req_fast_identifier);
            }
        }

        self.depth_tree_controller.nodes[index] = depth_tree_node;

        needs_tick_nodes.sort_by(|(_, a), (_, b)| a.cmp(&b));
        
        for (needs_tick_index, _) in needs_tick_nodes {
            let tree_node = &self.depth_tree_controller.nodes[needs_tick_index];
            if tree_node.processed {
                continue
            }
            
            self.tree_dispatcher.push_tree_build_tick(needs_tick_index);
            
            if DEBUG {
                self.node_storage.on_push_tree_build_queue_callback(tree_node.fast_identifier, tree_node.value_data.get_value_nr())
            }
        }
    }

    pub fn tree_apply_tick(&mut self, index: DepthIndex) {
        let mut depth_tree_node = self.depth_tree_controller.nodes[index].clone();
        depth_tree_node.applied = true;

        let value_nr = depth_tree_node.value_data.get_value_nr();

        if DEBUG {
            self.node_storage.on_pop_tree_apply_queue_callback(depth_tree_node.fast_identifier, value_nr);
            let identifier = self.node_storage.general_from_fast(depth_tree_node.fast_identifier);
            info!("Tree Apply: {:?}", identifier);
        }

        let node = self.node_storage.get_node_mut(depth_tree_node.fast_identifier);
        let old_value_nr = node.values[0].value_data.get_value_nr();

        node.set_values(vec![DepthValue::new(depth_tree_node.value_data)]);

        if DEBUG {
            self.node_storage.on_remove_value_callback(depth_tree_node.fast_identifier, old_value_nr);
            self.node_storage.on_add_value_callback(depth_tree_node.fast_identifier, value_nr);
            self.node_storage.on_select_value_callback(depth_tree_node.fast_identifier, value_nr);
        }

        for req_at in depth_tree_node.reqs.iter() {
            let chosen_node_index = req_at.tree_nodes[req_at.chosen_index.unwrap()].1;
            let tree_node = &self.depth_tree_controller.nodes[chosen_node_index];
            if !tree_node.applied {
                self.tree_dispatcher.push_tree_apply_tick(chosen_node_index);
                
                if DEBUG {
                    self.node_storage.on_push_tree_apply_queue_callback(tree_node.fast_identifier, tree_node.value_data.get_value_nr())
                }
            }
        }

        self.depth_tree_controller.nodes[index] = depth_tree_node;
    }
    
    /// For Debugging 
    /// Sends the fast identifier of the node that will be processed in the next tick.
    fn send_next_processed_node(&mut self) {
        let mut wfc_dispatcher = self.wfc_dispatcher.clone();
        let mut tree_dispatcher = self.tree_dispatcher.clone();
        
        if let Some((fast_identifier, _)) = wfc_dispatcher.pop_add() {
            self.node_storage.next_processed_node(Some(fast_identifier));
        } else if let Some((fast_identifier, _)) = wfc_dispatcher.pop_remove() {
            self.node_storage.next_processed_node(Some(fast_identifier));
        } else if let Some((fast_identifier, _)) = wfc_dispatcher.pop_select() {
            self.node_storage.next_processed_node(Some(fast_identifier));
        } else if let Some(index) = tree_dispatcher.pop_tree_build_tick() {
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

impl<N, WFCD, TreeD, GI, FI, PI,  VD, const DEBUG: bool> State for DepthNodeManager<N, WFCD, TreeD, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorageT<GI, FI, PI, DepthNode<VD>, DepthValue<VD>, VD>,
        WFCD: WFCDispatcherT<FI>,
        TreeD: DepthTreeDispatcherT,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT {
    fn tick_state(&mut self) -> bool {
        self.tick()
    }
}

