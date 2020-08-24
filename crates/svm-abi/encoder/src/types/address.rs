extern crate alloc;
use alloc::vec::Vec;

use svm_abi_layout::layout;
use svm_sdk::{
    types::PrimitiveMarker,
    value::{Address, AddressOwned},
};

use crate::Encoder;

macro_rules! impl_primitive_encoder {
    ($ty:ty, $marker:path) => {
        impl Encoder for $ty {
            /// Encodes `self` (of type `$ty`) and outputs the data into `w`
            fn encode(&self, w: &mut Vec<u8>) {
                w.push($marker);
                w.extend_from_slice(&self.0[..]);
            }
        }
    };
}

impl_primitive_encoder!(Address<'_>, layout::ADDRESS);
impl_primitive_encoder!(AddressOwned, layout::ADDRESS);
