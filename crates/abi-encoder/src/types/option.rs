use svm_sdk_std::Option;

use crate::traits::{ByteSize, Encoder, Push};

impl<T> Encoder for svm_sdk_std::Option<T>
where
    T: Encoder,
{
    fn encode(&self, w: &mut impl Push<Item = u8>) {
        match self {
            svm_sdk_std::Option::None => {
                w.push(svm_abi_layout::NONE);
            }
            svm_sdk_std::Option::Some(val) => val.encode(w),
        }
    }
}

impl<T> ByteSize for Option<T>
where
    T: ByteSize,
{
    fn byte_size(&self) -> usize {
        match self {
            Option::None => 1,
            Option::Some(val) => val.byte_size(),
        }
    }

    fn max_byte_size() -> usize {
        T::max_byte_size()
    }
}
