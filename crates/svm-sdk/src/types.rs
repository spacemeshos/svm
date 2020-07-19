/// Markers for encoding types.
/// Each marker consists of a single-byte and it denote a type.
pub mod marker {
    /// `Array` start marker
    pub const ARRAY_START: u8 = 0;

    /// `Array` end marker
    pub const ARRAY_END: u8 = 1;

    /// `Address` marker
    pub const ADDRESS: u8 = 2;

    /// `PubKey` of 256-bit marker
    pub const PUBKEY_256: u8 = 3;
}

/// A marker trait for Primitives
pub trait PrimitiveMarker {}

/// Represents a Primitive type
pub enum Primitive {
    /// denotes `Address` (20 bytes)
    Address,

    /// denotes `PubKey256` (32 bytes)
    PubKey256,
}

/// Represents a Composite type
pub enum Composite<'a> {
    /// `Array` type
    Array(&'a [Type<'a>]),
}

/// Represents any type (`Primitive` or `Composite`)
pub enum Type<'a> {
    /// Primitive type
    Primitive(Primitive),

    /// Composite type
    Composite(Composite<'a>),
}
