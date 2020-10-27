extern crate alloc;
use alloc::vec::Vec;

/// A trait used to encoding a value (of `Primitive` or `Composite` type)
pub trait Encoder {
    /// Encodes `self` and outputs the data into `w`
    fn encode(&self, w: &mut Vec<u8>);
}
