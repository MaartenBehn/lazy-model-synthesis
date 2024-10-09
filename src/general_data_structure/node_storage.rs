use crate::general_data_structure::identifier::{FastIdentifierT, GeneralIdentifierT, IdentifierConverterT, PackedIdentifierT};
use crate::general_data_structure::{ValueDataT, ValueNr};
use crate::general_data_structure::node::{NodeT, ValueIndex};

pub trait NodeStorageT<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT, NO: NodeT<VD>, VD: ValueDataT>: 
    IdentifierConverterT<GI, FI, PI> + Default + Clone
{
    type Req: Clone;
    
    fn get_mut_node(&mut self, fast_lookup: FI) -> &mut NO;

    fn get_num_reqs_for_value_data(&mut self, value_data: &VD) -> usize;
    fn get_req_for_value_data(&mut self, value_data: &VD, index: usize) -> Self::Req;

    fn get_req_node_identifier(&mut self, original_identifier: GI, req: &Self::Req) -> GI;

    fn is_identifier_valid(&self, identifier: GI) -> bool;

    fn value_data_matches_req(value_data: &VD, req: &Self::Req) -> bool;

    fn get_num_possible_value_data_for_req(req: &Self::Req) -> usize;

    fn get_value_data_for_req(req: &Self::Req, index: usize) -> VD;

    fn select_value_from_slice(&mut self, fast: FI) -> ValueIndex;

    // Callbacks for debug rendering
    fn on_add_value_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_remove_value_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_select_value_callback(&mut self, fast: FI, value_nr: ValueNr);

    fn on_push_add_queue_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_pop_add_queue_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_push_remove_queue_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_pop_remove_queue_callback(&mut self, fast: FI, value_nr: ValueNr);
    fn on_push_select_queue_callback(&mut self, fast: FI);
    fn on_pop_select_queue_callback(&mut self, fast: FI);

    fn next_processed_node(&mut self, fast: Option<FI>);
}