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
        assert!(self.len() < 255);

        let marker = match self.len() {
            0 => layout::ARR_0,
            1 => layout::ARR_1,
            2 => layout::ARR_2,
            3 => layout::ARR_3,
            4 => layout::ARR_4,
            5 => layout::ARR_5,
            6 => layout::ARR_6,
            7..256 => layout::ARR_0_255,
            _ => unreachable!(),
        };

        w.push(marker);

        for elem in self.iter() {
            elem.encode(w);
        }
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
