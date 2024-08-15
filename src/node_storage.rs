use std::fmt::Debug;
use crate::dispatcher::Dispatcher;
use crate::node::{Node, ValueIndex};
use crate::node_identifier::{IdentifierConverter};
use crate::value::ValueReq;

pub trait NodeStorage<GeneralIdentifier: Copy, FastIdentifier: Copy, PackedIdentifier: Copy>: 
    IdentifierConverter<GeneralIdentifier, FastIdentifier, PackedIdentifier> 
{

    type ValueData: Copy;
    type Req: Copy;
    type ShuffleSeed: Copy;

    fn add_initial_value(&mut self, identifier: GeneralIdentifier, value_data: Self::ValueData) -> ValueIndex {
        let fast_lookup = self.fast_from_general(identifier);
        let node = self.get_mut_node_from_fast_lookup(fast_lookup);
        let value_index = node.add_value(value_data);

        {   // For debugging
            self.on_add_value_callback(fast_lookup, value_data);
        }

        self.get_dispatcher().push_add(fast_lookup, value_index);

        {   // For debugging
            self.on_push_add_queue_callback(fast_lookup, value_data);
        }
        
        value_index
    }

    fn tick(&mut self) -> bool {
        let dispatcher = self.get_dispatcher();
        if let Some((fast_lookup, value_index)) = dispatcher.pop_add() {
            {   // For debugging
                let node = self.get_mut_node_from_fast_lookup(fast_lookup);
                let value_data = node.values[value_index].value_data.clone();
                self.on_pop_add_queue_callback(fast_lookup, value_data);
            }

            self.add_value_tick(fast_lookup, value_index);
        } else {
            return false
        }

        true
    }

    fn add_value_tick(&mut self, node_fast_lookup: FastIdentifier, value_index: ValueIndex) {
        
        let identifier = self.genera_from_fast(node_fast_lookup);

        let node = self.get_mut_node_from_fast_lookup(node_fast_lookup);
        let value_data = node.values[value_index].value_data.clone();

        for req in self.get_reqs_for_value_data(&value_data) {
            let req_identifier = self.get_req_node_identifier(identifier, &req);
            
            if !self.is_identifier_valid(req_identifier) {
                continue
            }

            let node = self.get_mut_node_from_fast_lookup(node_fast_lookup);
            let rc = node.values[value_index].add_value_req(ValueReq::new_node_value_counter()).clone();

            let neighbor_fast_lookup = self.fast_from_general(req_identifier);
            let neighbor_node = self.get_mut_node_from_fast_lookup(neighbor_fast_lookup);

            let neighbor_value_index = neighbor_node.get_value_index(|value_data| {
                Self::value_data_matches_req(value_data, &req)
            });

            if neighbor_value_index.is_none() {
                let req_value_data = Self::get_value_data_for_req(req);

                let neighbor_value_index = neighbor_node.add_value(req_value_data);
                neighbor_node.values[neighbor_value_index].add_ref(rc);

                {   // For debugging
                    self.on_add_value_callback(neighbor_fast_lookup, req_value_data);
                }
                
                self.get_dispatcher().push_add(neighbor_fast_lookup, neighbor_value_index);

                {   // For debugging
                    self.on_push_add_queue_callback(neighbor_fast_lookup, req_value_data);
                }
                
            } else {
                neighbor_node.values[neighbor_value_index.unwrap()].add_ref(rc);
            }
        }

    }

    fn get_dispatcher(&mut self) -> &mut impl Dispatcher<FastIdentifier>;

    fn get_mut_node(&mut self, identifier: GeneralIdentifier) -> &mut Node<Self::ValueData> {
        let fast_lookup = self.fast_from_general(identifier);
        self.get_mut_node_from_fast_lookup(fast_lookup)
    }

    fn get_mut_node_from_fast_lookup(&mut self, fast_lookup: FastIdentifier) -> &mut Node<Self::ValueData>;

    fn get_reqs_for_value_data(&mut self, value_data: &Self::ValueData) -> Vec<Self::Req>;

    fn get_req_node_identifier(&mut self, original_identifier: GeneralIdentifier, req: &Self::Req) -> GeneralIdentifier;

    fn is_identifier_valid(&self, identifier: GeneralIdentifier) -> bool;

    fn value_data_matches_req(value_data: &Self::ValueData, req: &Self::Req) -> bool;

    fn get_value_data_for_req(req: Self::Req) -> Self::ValueData;


    // Callbacks for debug rendering
    fn on_add_value_callback(&mut self, fast: FastIdentifier, value_data: Self::ValueData);
    fn on_remove_value_callback(&mut self, fast: FastIdentifier, value_data: Self::ValueData);

    fn on_push_add_queue_callback(&mut self, fast: FastIdentifier, value_data: Self::ValueData);
    fn on_pop_add_queue_callback(&mut self, fast: FastIdentifier, value_data: Self::ValueData);
}