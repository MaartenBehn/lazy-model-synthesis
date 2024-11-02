use std::fmt::Debug;
use std::hash::Hash;

/// A general identifier of the node.
/// For example Vec2 for 2D grid.
pub trait GeneralIdentifierT: Copy + Default + Debug + Eq + Hash + Ord {
    
}

/// An identifier for fast node access 
/// For example (ChunkIndex, NodeIndex) for 2D Grid
pub trait FastIdentifierT: Copy + Default + Debug + Eq + Hash + Ord {
    
}

/// An identifier that is packed to be stored in history
pub trait PackedIdentifierT: Copy + Default + Debug + Eq + Hash + Ord {
    fn to_bits(self) -> u32;
    fn from_bits(bits: u32) -> Self;
}


pub trait IdentifierConverterT<GI: GeneralIdentifierT, FI: FastIdentifierT, PI: PackedIdentifierT> {
    fn fast_from_general(&self, i: GI) -> FI;
    fn general_from_fast(&self, i: FI) -> GI;

    fn packed_from_general(&self, i: GI) -> PI;
    fn general_from_packed(&self, i: PI) -> GI;

    fn packed_from_fast(&self, i: FI) -> PI;
    fn fast_from_packed(&self, i: PI) -> FI;
}





