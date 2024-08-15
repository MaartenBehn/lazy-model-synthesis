
/// A general identifier of the node.
/// For example Vec2 for 2D grid.
pub trait GeneralIdentifier {
    
}

/// An identifier for fast node access 
/// For example (ChunkIndex, NodeIndex) for 2D Grid
pub trait FastIdentifier {
    
}

/// An identifier that is packed to be stored in history
pub trait PackedIdentifier {
    fn to_bits(self) -> u32;
    fn from_bits(bits: u32) -> Self;
}


pub trait IdentifierConverter<GeneralIdentifier, FastIdentifier, PackedIdentifier> {
    fn fast_from_general(&mut self, i: GeneralIdentifier) -> FastIdentifier;
    fn genera_from_fast(&mut self, i: FastIdentifier) -> GeneralIdentifier;

    fn packed_from_general(&mut self, i: GeneralIdentifier) -> PackedIdentifier;
    fn general_from_packed(&mut self, i: PackedIdentifier) -> GeneralIdentifier;

    fn packed_from_fast(&mut self, i: FastIdentifier) -> PackedIdentifier;
    fn fast_from_packed(&mut self, i: PackedIdentifier) -> FastIdentifier;
}

