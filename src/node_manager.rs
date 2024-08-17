use std::marker::PhantomData;
use crate::dispatcher::Dispatcher;
use crate::history::History;
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, IdentifierConverterT, PackedIdentifierT};
use crate::node::ValueIndex;
use crate::node_storage::NodeStorage;
use crate::util::state_saver::State;
use crate::value::{ValueDataT, ValueNr};
use crate::value::req::ValueReq;
use crate::value::req_by::{ValueReqByPacker};

#[derive(Default, Clone)]
pub struct NodeManager<N, D, GI, FI, PI, VD>
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
}

impl<N, D, GI, FI, PI, VD> NodeManager<N, D, GI, FI, PI, VD>
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
            history: History::new(node_storage.clone(), max_num_values),
            current: node_storage,
            .. Default::default()
        }
    }
    
    pub fn get_current(&self) -> &N { &self.current }
    pub fn get_current_mut(&mut self) -> &mut N { &mut self.current }

    pub fn select_initial_value(&mut self, identifier: GI, value_data: VD) {
        let fast_lookup = self.current.fast_from_general(identifier);
        let node = self.current.get_mut_node(fast_lookup);
        
        let value_nr = value_data.get_value_nr();
        node.add_value_with_index(0, value_data);
        self.dispatcher.push_add(fast_lookup, value_nr);
        
        self.dispatcher.push_select(fast_lookup, value_nr);
        
        {   // For debugging
            self.current.on_add_value_callback(fast_lookup, value_data);
            self.current.on_push_add_queue_callback(fast_lookup, value_data);
            self.current.on_select_value_callback(fast_lookup, value_data);
            self.current.on_push_select_queue_callback(fast_lookup, value_data);
        }
    }

    fn tick(&mut self) -> bool {
        if let Some((fast_identifier, value_nr)) = self.dispatcher.pop_add() {
            
            self.add_tick(fast_identifier, value_nr);
            
        } else if let Some((fast_identifier, value_nr)) = self.dispatcher.pop_remove() {

            self.remove_tick(fast_identifier, value_nr);

        } else if let Some((fast_identifier, value_nr)) = self.dispatcher.pop_select() {
            
            self.select_tick(fast_identifier, value_nr);
            
        } else {
            return false
        }

        true
    }

    fn add_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        
        // Get the other identifier of the node the value was added.
        let identifier = self.current.genera_from_fast(fast_identifier);
        let packed_identifier = self.current.packed_from_fast(fast_identifier);
        
        // Get the value_data and value nr of the value that was added
        let node = self.current.get_mut_node(fast_identifier);
        let value_index = node.get_value_index_from_value_nr(value_nr).unwrap() as ValueIndex;
        let value_data = node.values[value_index].value_data.clone();
        let value_nr = value_data.get_value_nr();

        {   // For debugging
            self.current.on_pop_add_queue_callback(fast_identifier, value_data);
        }
        
        // Go over all requirements of the added value 
        for req in self.current.get_reqs_for_value_data(&value_data) {
            
            // Get the identifier node that that should contain a value by requirement.
            let req_identifier = self.current.get_req_node_identifier(identifier, &req);
            
            if !self.current.is_identifier_valid(req_identifier) {
                // The node is out of bounds -> Skip requirement
                continue
            }
            
            // Add a Req of to the added value
            let node = self.current.get_mut_node(fast_identifier);
            let req_index = node.values[value_index].add_value_req(ValueReq::new_node_value_counter());
            
            // Call to mark that one other value will reference this req
            node.values[value_index].on_add_req_by(req_index);
            
            // Get the req node 
            let req_fast_identifier = self.current.fast_from_general(req_identifier);
            let req_node = self.current.get_mut_node(req_fast_identifier);
            
            // Get the value data from requirement and add it to the neighbor node
            let req_value_data = N::get_value_data_for_req(req);
            let req_value_nr = req_value_data.get_value_nr();

            // Check if the node contains the value that is required.
            let req_value_index = req_node.get_value_index_from_value_nr(req_value_nr);
            
            if req_value_index.is_err() {
                // The required value needs to be added
                
                let req_value_index = req_value_index.err().unwrap();
                req_node.add_value_with_index(req_value_index, req_value_data);
                
                // Reference the added node by adding a req by 
                // A req by is a packed node identifier, value nr and req index
                let req_req_by = self.req_by_packer.pack(packed_identifier, value_nr, req_index);
                req_node.values[req_value_index].add_req_by(req_req_by);
                
                // This neighbor node go a new value so push it to be processed
                self.dispatcher.push_add(req_fast_identifier, req_value_nr);

                {   // For debugging
                    self.current.on_add_value_callback(req_fast_identifier, req_value_data);
                    self.current.on_push_add_queue_callback(req_fast_identifier, req_value_data);
                }
                
            } else {
                // The neighbor node already had the value so just add the req by
                let req_req_by = self.req_by_packer.pack(packed_identifier, value_nr, req_index);
                req_node.values[req_value_index.unwrap()].add_req_by(req_req_by);
            }
        }
    }
    
    fn select_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        
        // Get the node of the value that is selected
        let node = self.current.get_mut_node(fast_identifier);
        let value_index = node.get_value_index_from_value_nr(value_nr).unwrap();
        
        // Swap the selected value into first place 
        // All other values will be later removed
        node.values.swap(0, value_index);
        
        // Go over alle values that are not selected and will be removed
        for i in (1..node.values.len()).rev() {
            let other_value_nr = node.values[i].value_data.get_value_nr();
            self.dispatcher.push_remove(fast_identifier, other_value_nr)
        }
        
        {   // For debugging
            let value_data = node.values[0].value_data;
            self.current.on_pop_select_queue_callback(fast_identifier, value_data);
            self.current.on_select_value_callback(fast_identifier, value_data);
        }
    }

    fn remove_tick(&mut self, fast_identifier: FI, value_nr: ValueNr) {
        
        // Get the value and value data of the value that will be removed
        let node = self.current.get_mut_node(fast_identifier);
        let value_index = node.get_value_index_from_value_nr(value_nr);
        if value_index.is_err() {
            return;
        }
        let value_index = value_index.unwrap();

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
                self.dispatcher.push_remove(req_fast_identifier, req_value_nr)
            }
        }

        // Log the removal of this value in the history 
        // by packing the node identifier and value nr.
        let value_nr = value_data.get_value_nr();
        let packed = self.current.packed_from_fast(fast_identifier);
        let history_index = self.history.add_change(packed, value_nr);

        // Save the index of the history node marking the removal in the node of later resetting.
        let node = self.current.get_mut_node(fast_identifier);
        node.last_removed[value_nr as usize] = history_index;

        node.values.remove(value_index);

        {   // For debugging
            self.current.on_pop_remove_queue_callback(fast_identifier, value_data);
            self.current.on_remove_value_callback(fast_identifier, value_data);
        }
    }
    
    
    
    
}

impl<N, D, GI, FI, PI, VD> State for NodeManager<N, D, GI, FI, PI, VD>
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

