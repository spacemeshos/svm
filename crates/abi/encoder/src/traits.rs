use num_traits::{AsPrimitive, Bounded};
use svm_abi_layout::layout;

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

// This trait has been added to let to-be-encoded values
// to expose how much bytes they will consume.
//
// A exact byte-size may be dependant on the value to be encoded (a.k.a variable-length encoding).
// Moreover, each Type implementing this trait should have a maximum byte-size that will suffice for encoding any value required.
//
// This trait has been defined as part of the `fixed-gas` efforts.
// The new `Vec` added by the `svm-sdk-std` crate is always being initialized using `Vec::with_capacity` method.
// In other words, a `Vec` should know in initialization time the maximum size it will need to store it's data.
// By knowing that, the `Vec` implementation has no `resize` / `shrink` code (as in the `std::vec::Vec`)
//  which would have resulted in `loop` opcodes when being compiled to Wasm.
pub trait ByteSize {
    fn byte_size(&self) -> usize;

    fn max_byte_size() -> usize;
}

/// Integer layout type information.
pub trait Numeric: AsPrimitive<Self::Unsigned> {
    type Unsigned: Copy + Numeric + AsPrimitive<u64>;
}
