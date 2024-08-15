use crate::dispatcher::Dispatcher;
use crate::node::{Node, ValueIndex};
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, IdentifierConverter, PackedIdentifierT};
use crate::value::ValueReq;

pub trait NodeStorage<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT, VD: Copy>: 
    IdentifierConverter<GI, FI, PI> + Default + Clone
{
    type Req: Copy;
    type ShuffleSeed: Copy;
    
    fn get_mut_node(&mut self, identifier: GI) -> &mut Node<VD> {
        let fast_lookup = self.fast_from_general(identifier);
        self.get_mut_node_from_fast_lookup(fast_lookup)
    }

    fn get_mut_node_from_fast_lookup(&mut self, fast_lookup: FI) -> &mut Node<VD>;

    fn get_reqs_for_value_data(&mut self, value_data: &VD) -> Vec<Self::Req>;

    fn get_req_node_identifier(&mut self, original_identifier: GI, req: &Self::Req) -> GI;

    fn is_identifier_valid(&self, identifier: GI) -> bool;

    fn value_data_matches_req(value_data: &VD, req: &Self::Req) -> bool;

    fn get_value_data_for_req(req: Self::Req) -> VD;


    // Callbacks for debug rendering
    fn on_add_value_callback(&mut self, fast: FI, value_data: VD);
    fn on_remove_value_callback(&mut self, fast: FI, value_data: VD);

    fn on_push_add_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_pop_add_queue_callback(&mut self, fast: FI, value_data: VD);
}