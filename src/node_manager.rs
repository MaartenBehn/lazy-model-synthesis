use std::marker::PhantomData;
use crate::dispatcher::Dispatcher;
use crate::history::History;
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, PackedIdentifierT};
use crate::node::ValueIndex;
use crate::node_storage::NodeStorage;
use crate::util::state_saver::State;
use crate::value::{ValueDataT, ValueReq};

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
    pub fn new(node_storage: N, num_values: usize) -> Self {
        NodeManager {
            history: History::new(node_storage.clone(), num_values),
            current: node_storage,
            .. Default::default()
        }
    }
    
    pub fn get_current(&self) -> &N { &self.current }
    pub fn get_current_mut(&mut self) -> &mut N { &mut self.current }

    pub fn add_initial_value(&mut self, identifier: GI, value_data: VD) -> ValueIndex {
        let fast_lookup = self.current.fast_from_general(identifier);
        let node = self.current.get_mut_node_from_fast_lookup(fast_lookup);
        let value_index = node.add_value(value_data);

        {   // For debugging
            self.current.on_add_value_callback(fast_lookup, value_data);
        }

        self.dispatcher.push_add(fast_lookup, value_index);

        {   // For debugging
            self.current.on_push_add_queue_callback(fast_lookup, value_data);
        }

        value_index
    }

    fn tick(&mut self) -> bool {
        if let Some((fast_lookup, value_index)) = self.dispatcher.pop_add() {
            {   // For debugging
                let node = self.current.get_mut_node_from_fast_lookup(fast_lookup);
                let value_data = node.values[value_index].value_data.clone();
                self.current.on_pop_add_queue_callback(fast_lookup, value_data);
            }

            self.add_value_tick(fast_lookup, value_index);
        } else {
            return false
        }

        true
    }

    fn add_value_tick(&mut self, node_fast_lookup: FI, value_index: ValueIndex) {

        let identifier = self.current.genera_from_fast(node_fast_lookup);

        let node = self.current.get_mut_node_from_fast_lookup(node_fast_lookup);
        let value_data = node.values[value_index].value_data.clone();

        for req in self.current.get_reqs_for_value_data(&value_data) {
            let req_identifier = self.current.get_req_node_identifier(identifier, &req);

            if !self.current.is_identifier_valid(req_identifier) {
                continue
            }

            let node = self.current.get_mut_node_from_fast_lookup(node_fast_lookup);
            let rc = node.values[value_index].add_value_req(ValueReq::new_node_value_counter()).clone();

            let neighbor_fast_lookup = self.current.fast_from_general(req_identifier);
            let neighbor_node = self.current.get_mut_node_from_fast_lookup(neighbor_fast_lookup);

            let neighbor_value_index = neighbor_node.get_value_index(|value_data| {
                N::value_data_matches_req(value_data, &req)
            });

            if neighbor_value_index.is_none() {
                let req_value_data = N::get_value_data_for_req(req);

                let neighbor_value_index = neighbor_node.add_value(req_value_data);
                neighbor_node.values[neighbor_value_index].add_ref(rc);

                {   // For debugging
                    self.current.on_add_value_callback(neighbor_fast_lookup, req_value_data);
                }

                self.dispatcher.push_add(neighbor_fast_lookup, neighbor_value_index);

                {   // For debugging
                    self.current.on_push_add_queue_callback(neighbor_fast_lookup, req_value_data);
                }

            } else {
                neighbor_node.values[neighbor_value_index.unwrap()].add_ref(rc);
            }
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
    fn tick_state(&mut self) {
        self.tick();
    }
}

