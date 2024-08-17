use crate::util::get_num_bits_for_number;
use crate::value::ValueNr;

/// A general identifier of the node.
/// For example Vec2 for 2D grid.
pub trait GeneralIdentifierT: Copy + Default {
    
}

/// An identifier for fast node access 
/// For example (ChunkIndex, NodeIndex) for 2D Grid
pub trait FastIdentifierT: Copy + Default {
    
}

/// An identifier that is packed to be stored in history
pub trait PackedIdentifierT: Copy + Default {
    fn to_bits(self) -> u32;
    fn from_bits(bits: u32) -> Self;
}


pub trait IdentifierConverterT<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT> {
    fn fast_from_general(&mut self, i: GI) -> FI;
    fn genera_from_fast(&mut self, i: FI) -> GI;

    fn packed_from_general(&mut self, i: GI) -> PI;
    fn general_from_packed(&mut self, i: PI) -> GI;

    fn packed_from_fast(&mut self, i: FI) -> PI;
    fn fast_from_packed(&mut self, i: PI) -> FI;
}





