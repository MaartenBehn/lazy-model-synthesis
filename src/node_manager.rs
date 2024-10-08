use std::marker::PhantomData;
use octa_force::log::{debug, error, info, warn};
use octa_force::puffin_egui::puffin;
use crate::dispatcher::Dispatcher;
use crate::history::History;
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, PackedIdentifierT};
use crate::node::{HistoryIndex, ValueIndex};
use crate::node_storage::NodeStorage;
use crate::util::state_saver::State;
use crate::value::{ValueDataT, ValueNr};
use crate::value::req::ValueReq;
use crate::value::req_by::{ValueReqByPacker};

#[derive(Default, Clone)]
pub struct NodeManager<N, D, GI, FI, PI, VD, const DEBUG: bool>
    where
        N: NodeStorage<GI, FI, PI, VD>,
        D: Dispatcher<FI>,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT,
{
    phantom_data0: PhantomData<GI>,
    phantom_data1: PhantomData<FI>,
    phantom_data2: PhantomData<PI>,
    phantom_data4: PhantomData<VD>,

    req_by_packer: ValueReqByPacker,
    
    history: History<N>,
    current: N,
    dispatcher: D,
    last_remove_befor_select: HistoryIndex,
    first_remove: bool
}

impl<N, D, GI, FI, PI, VD, const DEBUG: bool> NodeManager<N, D, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorage<GI, FI, PI, VD>,
        D: Dispatcher<FI>,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT,
{
    pub fn new(node_storage: N, max_num_values: usize, max_num_reqs: usize) -> Self {
        NodeManager {
            req_by_packer: ValueReqByPacker::new(max_num_values, max_num_reqs),
            history: History::new(max_num_values),
            current: node_storage,
            last_remove_befor_select: 0,
            first_remove: true,
            .. Default::default()
        }
    }
    
    pub fn get_current(&self) -> &N { &self.current }
    pub fn get_current_mut(&mut self) -> &mut N { &mut self.current }
    pub fn get_history(&self) -> &History<N> { &self.history }

    pub fn select_value(&mut self, identifier: GI, value_data: VD) {
        let fast_identifier = self.current.fast_from_general(identifier);
        let node = self.current.get_mut_node(fast_identifier);
        let value_nr = value_data.get_value_nr();
        
        // Check if node already had the value once
        let history_index = node.last_removed[value_nr as usize];
        if history_index == 0 {
            // The node never had this value
            
            // If the node doesn't had the value add it.
            let value_index = node.get_value_index_from_value_nr(value_nr);
            if value_index.is_err() {
                let value_index = value_index.err().unwrap();
                node.add_value_with_index(value_index, value_data);

                self.dispatcher.push_add(fast_identifier, value_nr);
                if DEBUG {
                    self.current.on_push_add_queue_callback(fast_identifier, value_nr);
                    self.current.on_add_value_callback(fast_identifier, value_nr);
                }
            }

            self.dispatcher.push_select(fast_identifier);

            if DEBUG {   // For debugging
                self.current.on_push_select_queue_callback(fast_identifier);
            }
        } else {
            // The node already had the value once -> Reset world to earlier state
            
            info!("Go back in time to {history_index}");
            self.go_back_in_time(history_index as usize);

            let node = self.current.get_mut_node(fast_identifier);
            let value_index = node.get_value_index_from_value_nr(value_nr).unwrap();
            self.perform_select(fast_identifier, value_index);
        }
    }

    pub fn go_back_in_time(&mut self, index: usize) {
        let summary_index = self.history.last_summary_before_change(index);
        self.current = self.history.get_summary(summary_index).clone();
        
        for i in (summary_index + 1)..index {
            let (packed_identifier, value_nr) = self.history.get_change(i);
            
            let fast_identifier = self.current.fast_from_packed(packed_identifier);
            let node = self.current.get_mut_node(fast_identifier);
            
            let value_index = node.get_value_index_from_value_nr(value_nr);
            if value_index.is_err() {
                continue;
            }
            let value_index = value_index.unwrap();

            let req_by_len = node.values[value_index].req_by.len();

            /*
            // Go over values that require this value and check if they should also be removed.
            for j in 0..req_by_len {

                // Get the req by
                let node = self.current.get_mut_node(fast_identifier);
                if node.values.len() <= value_index {
                    continue
                }
                
                let req_by = node.values[value_index].req_by[j];

                // Identify the req that is linked to this value
                let (req_packed_identifier, req_value_nr, req_index) = self.req_by_packer.unpack::<PI>(req_by);
                let req_fast_identifier = self.current.fast_from_packed(req_packed_identifier);

                // Get the req node and find the value
                let req_node = self.current.get_mut_node(req_fast_identifier);
                let req_value_index = req_node.get_value_index_from_value_nr(req_value_nr);
                if req_value_index.is_err() {
                    // The req value was already removed -> Skip
                    continue;
                }
                let req_value_index = req_value_index.unwrap();

                /*
                // Check if the value that requires this value should be removed
                let req_should_be_removed = req_node.values[req_value_index].reqs[req_index].on_remove_req_by();
                if req_should_be_removed {
                    //self.dispatcher.push_remove(req_fast_identifier, req_value_nr);

                    if DEBUG {
                        self.current.on_push_remove_queue_callback(req_fast_identifier, req_value_nr);
                    }
                }
                 */
            }
            
             */

            let node = self.current.get_mut_node(fast_identifier);
            node.values.remove(value_index);

            if node.values.len() == 1 {
                node.selected = true;

                let last_value_nr = node.values[0].value_data.get_value_nr();
                self.current.on_select_value_callback(fast_identifier, last_value_nr);
            }

            if DEBUG {
                self.current.on_remove_value_callback(fast_identifier, value_nr);
            }
        }
        
        
        self.history.remove_all_after_with_last_summary_index(index, summary_index);
    }

    pub fn tick(&mut self) -> bool {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        let needs_further_ticks = if let Some((fast_identifier, value_nr)) = self.dispatcher.pop_add() {
            self.add_tick(fast_identifier, value_nr);
            true
        } else if let Some((fast_identifier, value_nr)) = self.dispatcher.pop_remove() {
            self.remove_tick(fast_identifier, value_nr);
            true
        } else if let Some(fast_identifier) = self.dispatcher.pop_select() {
            self.select_tick(fast_identifier);
            true
        } else {
            false
        };
        
        if DEBUG {
            self.send_next_processed_node();
        }

        needs_further_ticks
    }

    fn add_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }

        if DEBUG {
            self.current.on_pop_add_queue_callback(fast_identifier, value_nr);
        }
        
        // Get the other identifier of the node the value was added.
        let identifier = self.current.general_from_fast(fast_identifier);
        let packed_identifier = self.current.packed_from_fast(fast_identifier);
        
        if DEBUG {
            info!("ADD: {:?}", identifier);
        }
        
        // Get the value_data and value nr of the value that was added
        let node = self.current.get_mut_node(fast_identifier);
        
        
        let value_index = node.get_value_index_from_value_nr(value_nr);
        let value_index = if DEBUG && value_index.is_err() {
            warn!("Add {:?} value nr {:?} not found!", identifier, value_nr);
            return;
        } else {
            value_index.unwrap()
        };
        
        let value_data = node.values[value_index].value_data.clone();
        
        let num_reqs = self.current.get_num_reqs_for_value_data(&value_data);
        // Go over all requirements of the added value 
        for req_index in 0..num_reqs {
            let req = self.current.get_req_for_value_data(&value_data, req_index);
            
            // Get the identifier node that that should contain a value by requirement.
            let req_identifier = self.current.get_req_node_identifier(identifier, &req);
            
            if !self.current.is_identifier_valid(req_identifier) {
                // The node is out of bounds -> Skip requirement
                continue
            }
            
            // Add a Req of to the added value
            let node = self.current.get_mut_node(fast_identifier);
            let req_index = node.values[value_index].add_value_req(ValueReq::new_node_value_counter());
            
            let req_fast_identifier = self.current.fast_from_general(req_identifier);
            
            let num_possible_value_data = N::get_num_possible_value_data_for_req(&req);
            for req_value_data_index in 0..num_possible_value_data {
                
                let req_value_data = N::get_value_data_for_req(&req, req_value_data_index);
                let req_value_nr = req_value_data.get_value_nr();

                // Check if the node contains the value that is required.
                let req_node = self.current.get_mut_node(req_fast_identifier);
                let req_value_index = req_node.get_value_index_from_value_nr(req_value_nr);

                if req_value_index.is_err() {
                    // The required value needs to be added

                    let req_value_index = req_value_index.err().unwrap();
                    req_node.add_value_with_index(req_value_index, req_value_data);

                    // Reference the added node by adding a req by 
                    // A req by is a packed node identifier, value nr and req index
                    let req_req_by = self.req_by_packer.pack(packed_identifier, value_nr, req_index);
                    req_node.values[req_value_index].add_req_by(req_req_by);

                    // Call to mark that one other value will reference this req
                    let node = self.current.get_mut_node(fast_identifier);
                    node.values[value_index].on_add_req_by(req_index);

                    // This neighbor node go a new value so push it to be processed
                    self.dispatcher.push_add(req_fast_identifier, req_value_nr);

                    if DEBUG {   // For debugging
                        self.current.on_add_value_callback(req_fast_identifier, req_value_nr);
                        self.current.on_push_add_queue_callback(req_fast_identifier, req_value_nr);
                    }

                } else {
                    // The neighbor node already had the value so just add the req by
                    let req_req_by = self.req_by_packer.pack(packed_identifier, value_nr, req_index);
                    req_node.values[req_value_index.unwrap()].add_req_by(req_req_by);

                    // Call to mark that one other value will reference this req
                    let node = self.current.get_mut_node(fast_identifier);
                    node.values[value_index].on_add_req_by(req_index);
                }
            }
        }
    }
    
    fn remove_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        puffin::profile_function!();

        if DEBUG {   // For debugging
            self.current.on_pop_remove_queue_callback(fast_identifier, value_nr);
        }

        if DEBUG {
            let identifier = self.current.general_from_fast(fast_identifier);
            info!("Remove: {:?}", identifier);
        }
        
        // Get the value and value data of the value that will be removed
        let node = self.current.get_mut_node(fast_identifier);
        
        let value_index = node.get_value_index_from_value_nr(value_nr);
        let value_index = if DEBUG && value_index.is_err() {
            let identifier = self.current.general_from_fast(fast_identifier);
            warn!("Remove {:?} value nr {:?} not found!",identifier, value_nr);
            return;
        } else {
            value_index.unwrap()
        };
        

        let value_data = node.values[value_index].value_data;
        let req_by_len = node.values[value_index].req_by.len();

        // Go over values that require this value and check if they should also be removed.
        for j in 0..req_by_len {

            // Get the req by
            let node = self.current.get_mut_node(fast_identifier);
            let req_by = node.values[value_index].req_by[j];

            // Identify the req that is linked to this value
            let (req_packed_identifier, req_value_nr, req_index) = self.req_by_packer.unpack::<PI>(req_by);
            let req_fast_identifier = self.current.fast_from_packed(req_packed_identifier);

            // Get the req node and find the value
            let req_node = self.current.get_mut_node(req_fast_identifier);
            let req_value_index = req_node.get_value_index_from_value_nr(req_value_nr);
            if req_value_index.is_err() {
                // The req value was already removed -> Skip
                continue;
            }
            let req_value_index = req_value_index.unwrap();

            // Check if the value that requires this value should be removed
            let req_should_be_removed = req_node.values[req_value_index].reqs[req_index].on_remove_req_by();
            if req_should_be_removed {
                self.dispatcher.push_remove(req_fast_identifier, req_value_nr);

                if DEBUG {
                    self.current.on_push_remove_queue_callback(req_fast_identifier, req_value_nr);
                }
            }
        }

        // Log the removal of this value in the history 
        // by packing the node identifier and value nr.
        let value_nr = value_data.get_value_nr();
        let packed = self.current.packed_from_fast(fast_identifier);
        self.history.add_change(packed, value_nr);

        // Save the index of the history node marking the removal in the node of later resetting.
        let node = self.current.get_mut_node(fast_identifier);
        node.last_removed[value_nr as usize] = self.last_remove_befor_select;

        node.values.remove(value_index);

        if DEBUG {
            self.current.on_remove_value_callback(fast_identifier, value_nr);
        }
    }

    fn select_tick(&mut self, fast_identifier: FI) {
        if cfg!(debug_assertions) {
            puffin::profile_function!();
        }
        
        if DEBUG {
            self.current.on_pop_select_queue_callback(fast_identifier);

            let identifier = self.current.general_from_fast(fast_identifier);
            info!("Select: {:?}", identifier);
            
            let node = self.current.get_mut_node(fast_identifier);
            if node.selected {
                let identifier = self.current.general_from_fast(fast_identifier);
                warn!("Node {:?} already selected!", identifier);
                return;
            }
        }

        if self.first_remove {
            self.history.add_summary(self.current.clone());
            self.first_remove = false;
        }
        
        let value_index = self.current.select_value_from_slice(fast_identifier);

        self.perform_select(fast_identifier, value_index);
    }
    
    fn perform_select(&mut self, fast_identifier: FI, value_index: ValueIndex) {
        if DEBUG {
            let node = self.current.get_mut_node(fast_identifier);
            let value_data = node.values[value_index].value_data;
            let value_nr = value_data.get_value_nr();
            self.current.on_select_value_callback(fast_identifier, value_nr);
        }

        let node = self.current.get_mut_node(fast_identifier);
        let req_by_len = node.values[value_index].req_by.len();

        // Swap the selected value into first place 
        // All other values will be later removed
        node.values.swap(0, value_index);
        node.selected = true;

        self.last_remove_befor_select = self.history.last_index();

        // Go over all values that are not selected and will be removed
        for i in (1..node.values.len()).rev() {
            let other_value_nr = node.values[i].value_data.get_value_nr();
            self.dispatcher.push_remove(fast_identifier, other_value_nr)
        }

        for i in 0..req_by_len {
            // Get the req by
            let node = self.current.get_mut_node(fast_identifier);
            let req_by = node.values[value_index].req_by[i];

            // Identify the req that is linked to this value
            let (req_packed_identifier, req_value_nr, _) = self.req_by_packer.unpack::<PI>(req_by);
            let req_fast_identifier = self.current.fast_from_packed(req_packed_identifier);

            let req_node = self.current.get_mut_node(req_fast_identifier);
            if req_node.selected {
                continue;
            }

            if self.dispatcher.select_contains_node(req_fast_identifier) {
                continue;
            }

            self.dispatcher.push_select(req_fast_identifier);

            if DEBUG {
                self.current.on_push_select_queue_callback(req_fast_identifier);
            }
        }
    }
    
    /// For Debugging 
    /// Sends the fast identifier of the node that will be processed in the next tick.
    fn send_next_processed_node(&mut self) {
        let mut dispatcher = self.dispatcher.clone();
        
        if let Some((fast_identifier, _)) = dispatcher.pop_add() {
            self.current.next_processed_node(Some(fast_identifier));
        } else if let Some((fast_identifier, _)) = dispatcher.pop_remove() {
            self.current.next_processed_node(Some(fast_identifier));
        } else if let Some(fast_identifier) = dispatcher.pop_select() {
            self.current.next_processed_node(Some(fast_identifier));
        } else {
            self.current.next_processed_node(None);
        }
    }
}

impl<N, D, GI, FI, PI, VD, const DEBUG: bool> State for NodeManager<N, D, GI, FI, PI, VD, DEBUG>
    where
        N: NodeStorage<GI, FI, PI, VD>,
        D: Dispatcher<FI>,
        GI: GeneralIdentifierT,
        FI: FastIdentifierT,
        PI: PackedIdentifierT,
        VD: ValueDataT {
    fn tick_state(&mut self) -> bool {
        self.tick()
    }
}

