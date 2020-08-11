extern crate alloc;

use alloc::vec::Vec;

/// A trait used to encoding a value (of `primitive` or `composite` type)
pub trait Encoder {
    /// Encodes `self` and outputs the data into `buf`
    fn encode(&self, buf: &mut Vec<u8>);
}
