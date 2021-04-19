use svm_abi_layout::layout;
use svm_sdk_types::Address;

use crate::{ByteSize, Encoder};

macro_rules! impl_primitive_encoder {
    ($W:ty) => {
        impl Encoder<$W> for Address {
            /// Encodes `self` (of type `$ty`) and outputs the data into `w`
            fn encode(&self, w: &mut $W) {
                w.push(layout::ADDRESS);

                let bytes = self.as_slice();

                // This code calls for using a loop.
                // In order to assure there is no usage of `loop` when compiled to Wasm,
                // we manually inject multiple statements.
                //
                // A future feature will add an ergonomic way to achieve the same
                // without this tedious copy-pase ugly code.
                //
                // TODO:
                // There is an issue for that: [Issue #230](https://github.com/spacemeshos/svm/issues/230)
                w.push(bytes[0]);
                w.push(bytes[1]);
                w.push(bytes[2]);
                w.push(bytes[3]);
                w.push(bytes[4]);
                w.push(bytes[5]);
                w.push(bytes[6]);
                w.push(bytes[7]);
                w.push(bytes[8]);
                w.push(bytes[9]);
                w.push(bytes[10]);
                w.push(bytes[11]);
                w.push(bytes[12]);
                w.push(bytes[13]);
                w.push(bytes[14]);
                w.push(bytes[15]);
                w.push(bytes[16]);
                w.push(bytes[17]);
                w.push(bytes[18]);
                w.push(bytes[19]);
            }
        }
    };
}

impl_primitive_encoder!(svm_sdk_std::Vec<u8>);

impl ByteSize for Address {
    fn byte_size(&self) -> usize {
        21
    }

    fn max_byte_size() -> usize {
        21
    }
}
