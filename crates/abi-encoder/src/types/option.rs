use svm_sdk_std::Option;

use crate::traits::{ByteSize, Encoder, Push};

impl<T, W> Encoder<W> for svm_sdk_std::Option<T>
where
    T: Encoder<W>,
    W: Push<Item = u8>,
{
    fn encode(&self, w: &mut W) {
        match self {
            svm_sdk_std::Option::None => {
                use svm_abi_layout::layout;

                w.push(layout::NONE);
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
