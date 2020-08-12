extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;
use svm_sdk::types::PrimitiveMarker;

use crate::Encoder;

impl<'a, T> Encoder for &[T]
where
    T: Encoder + PrimitiveMarker,
{
    fn encode(&self, w: &mut Vec<u8>) {
        w.push(layout::ARRAY_START);

        for elem in self.iter() {
            elem.encode(w);
        }

        w.push(layout::ARRAY_END);
    }
}

impl<'a, T> Encoder for Vec<T>
where
    T: Encoder + PrimitiveMarker,
{
    #[inline]
    fn encode(&self, w: &mut Vec<u8>) {
        (&self[..]).encode(w)
    }
}
