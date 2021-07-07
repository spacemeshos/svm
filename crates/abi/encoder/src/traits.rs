use num_traits::AsPrimitive;

/// A trait used to encoding a value (of `Primitive` or `Composite` type)
pub trait Encoder<W> {
    /// Encodes `self` and outputs the data into `w`
    fn encode(&self, w: &mut W);
}

pub trait Push {
    type Item;

    fn push(&mut self, item: Self::Item);
}

impl<T> Push for svm_sdk_std::Vec<T> {
    type Item = T;

    fn push(&mut self, item: Self::Item) {
        svm_sdk_std::Vec::push(self, item);
    }
}

/// This trait has been added to let to-be-encoded values to expose how much
/// bytes they will consume.
//
/// A exact byte-size may be dependant on the value to be encoded (a.k.a
/// variable-length encoding).
/// Moreover, each Type implementing this trait should have a maximum byte-size
/// that will suffice for encoding any value required.
//
/// This trait has been defined as part of the `fixed-gas` efforts.
/// The new `Vec` added by the `svm-sdk-std` crate is always being initialized
/// using `Vec::with_capacity` method.
/// In other words, a `Vec` should know in initialization time the maximum size
/// it will need to store it's data.
/// By knowing that, the `Vec` implementation has no `resize` / `shrink` code
/// (as in the `std::vec::Vec`) which would have resulted in `loop` opcodes when
/// being compiled to Wasm.
pub trait ByteSize {
    /// Returns the expected size in bytes that will be required to store
    /// `self`. This is *not* an estimate and rather must be exact.
    fn byte_size(&self) -> usize;

    /// Returns the absolute maximum space in bytes that might be needed to
    /// store any instance of `Self`.
    fn max_byte_size() -> usize;
}

/// Integer layout type information. This is needed for encoding numeric types
/// and accessing type information about them (we need to cast everything to the
/// unsigned type of the same width).
pub trait Numeric: AsPrimitive<Self::Unsigned> {
    type Unsigned: Copy + Numeric + AsPrimitive<u64>;
}
