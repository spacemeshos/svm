extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;
use svm_nibble::{nib, NibbleWriter};
use svm_sdk::types::PrimitiveMarker;

use crate::Encoder;

impl<'a, T> Encoder for &[T]
where
    T: Encoder + PrimitiveMarker,
{
    fn encode(&self, w: &mut NibbleWriter) {
        w.push(nib!(layout::ARRAY_START));

        for elem in self.iter() {
            elem.encode(w);
        }

        w.push(nib!(layout::ARRAY_END));
    }
}

impl<'a, T> Encoder for Vec<T>
where
    T: Encoder + PrimitiveMarker,
{
    #[inline]
    fn encode(&self, w: &mut NibbleWriter) {
        (&self[..]).encode(w)
    }
}
