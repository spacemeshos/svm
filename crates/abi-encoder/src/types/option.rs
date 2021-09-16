use svm_sdk_std::Option;

use crate::traits::{ABIEncoder, ByteSize, Push};

impl<T> ABIEncoder for svm_sdk_std::Option<T>
where
    T: ABIEncoder,
{
    fn encode(&self, w: &mut impl Push<Item = u8>) {
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
