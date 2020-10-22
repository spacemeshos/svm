extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;

use svm_sdk::types::PrimitiveMarker;
use svm_sdk::value::Value;

use crate::Encoder;

macro_rules! impl_encode {
    () => {
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
    };
}

impl<T: Encoder> Encoder for &[T] {
    impl_encode!();
}

impl Encoder for &[&dyn Encoder] {
    impl_encode!();
}

impl<T: Encoder> Encoder for Vec<T> {
    #[inline]
    fn encode(&self, w: &mut Vec<u8>) {
        (&self[..]).encode(w)
    }
}

impl Encoder for Vec<&dyn Encoder> {
    #[inline]
    fn encode(&self, w: &mut Vec<u8>) {
        (&self[..]).encode(w)
    }
}

macro_rules! impl_array_encode {
    ($($N:expr),*) => {
        $( impl_array_encode!{@ $N} )*
    };

    (@ $N:expr) => {
        impl<T: Encoder> Encoder for [T; $N] {
            #[inline]
            fn encode(&self, w: &mut Vec<u8>) {
                (&self[..]).encode(w)
            }
        }
    };
}

impl_array_encode!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
