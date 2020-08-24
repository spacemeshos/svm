/// A marker trait for Primitives
pub trait PrimitiveMarker {}

macro_rules! mark_primitive {
    ($($ty:ty),*) => {
        $( impl PrimitiveMarker for $ty {} )*
    };
}

mark_primitive!(bool);
mark_primitive!(u8, i8);
mark_primitive!(u16, i16);
mark_primitive!(u32, i32);
mark_primitive!(u64, i64);

/// Represents a Primitive type
pub enum Primitive {
    Bool,

    I32,

    I64,

    Address,
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
