
use crate::node::{Node};
use crate::identifier::{FastIdentifierT, GeneralIdentifierT, IdentifierConverterT, PackedIdentifierT};
use crate::value::ValueDataT;

pub trait NodeStorage<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT, VD: ValueDataT>: 
    IdentifierConverterT<GI, FI, PI> + Default + Clone
{
    type Req: Clone;
    type ShuffleSeed: Copy;
    
    fn get_mut_node(&mut self, fast_lookup: FI) -> &mut Node<VD>;

    fn get_reqs_for_value_data(&mut self, value_data: &VD) -> Vec<Self::Req>;

    fn get_req_node_identifier(&mut self, original_identifier: GI, req: &Self::Req) -> GI;

    fn is_identifier_valid(&self, identifier: GI) -> bool;

    fn value_data_matches_req(value_data: &VD, req: &Self::Req) -> bool;

    fn get_possible_value_data_for_req(req: Self::Req) -> Vec<VD>;


    // Callbacks for debug rendering
    fn on_add_value_callback(&mut self, fast: FI, value_data: VD);
    fn on_remove_value_callback(&mut self, fast: FI, value_data: VD);
    fn on_select_value_callback(&mut self, fast: FI, value_data: VD);
    fn on_unselect_value_callback(&mut self, fast: FI, value_data: VD);

    fn on_push_add_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_pop_add_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_push_remove_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_pop_remove_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_push_select_queue_callback(&mut self, fast: FI, value_data: VD);
    fn on_pop_select_queue_callback(&mut self, fast: FI, value_data: VD);
}