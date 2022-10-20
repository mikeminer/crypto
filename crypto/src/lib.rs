pub use winter_utils::{
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};
pub use winterfell::crypto::{Digest, ElementHasher, Hasher as HashFn};
pub use winterfell::math::{
    fields::{f64::BaseElement as Felt, QuadExtension},
    ExtensionOf, FieldElement, StarkField,
};

pub mod hash;
pub mod merkle;

// TYPE ALIASES
// ================================================================================================

pub type Word = [Felt; 4];

// CONSTANTS
// ================================================================================================

/// Field element representing ZERO in the base field of the VM.
pub const ZERO: Felt = Felt::ZERO;

/// Field element representing ONE in the base field of the VM.
pub const ONE: Felt = Felt::ONE;
