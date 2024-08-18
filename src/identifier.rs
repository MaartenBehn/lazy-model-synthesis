use std::fmt::Debug;

/// A general identifier of the node.
/// For example Vec2 for 2D grid.
pub trait GeneralIdentifierT: Copy + Default + Debug + Eq {
    
}

/// An identifier for fast node access 
/// For example (ChunkIndex, NodeIndex) for 2D Grid
pub trait FastIdentifierT: Copy + Default + Debug + Eq {
    
}

/// An identifier that is packed to be stored in history
pub trait PackedIdentifierT: Copy + Default + Debug + Eq {
    fn to_bits(self) -> u32;
    fn from_bits(bits: u32) -> Self;
}


pub trait IdentifierConverterT<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT> {
    fn fast_from_general(&mut self, i: GI) -> FI;
    fn general_from_fast(&mut self, i: FI) -> GI;

    fn packed_from_general(&mut self, i: GI) -> PI;
    fn general_from_packed(&mut self, i: PI) -> GI;

    fn packed_from_fast(&mut self, i: FI) -> PI;
    fn fast_from_packed(&mut self, i: PI) -> FI;
}





