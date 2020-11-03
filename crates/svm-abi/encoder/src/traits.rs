extern crate alloc;
extern crate core;

use alloc::vec::Vec;
use core::ops::Deref;

/// A trait used to encoding a value (of `Primitive` or `Composite` type)
pub trait Encoder {
    /// Encodes `self` and outputs the data into `w`
    fn encode(&self, w: &mut Vec<u8>);
}

impl<T> Encoder for &T
where
    T: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        (**self).encode(w);
    }
}

impl<T> Encoder for &mut T
where
    T: Encoder,
{
    fn encode(&self, w: &mut Vec<u8>) {
        (**self).encode(w);
    }
}
