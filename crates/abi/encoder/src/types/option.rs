use crate::traits::ByteSize;

use svm_sdk_std::Option;

macro_rules! encode {
    ($W:ty) => {
        impl<T> crate::traits::Encoder<$W> for svm_sdk_std::Option<T>
        where
            T: crate::traits::Encoder<$W>,
        {
            fn encode(&self, w: &mut $W) {
                match self {
                    svm_sdk_std::Option::None => {
                        use svm_abi_layout::layout;

                        w.push(layout::NONE);
                    }
                    svm_sdk_std::Option::Some(val) => val.encode(w),
                }
            }
        }
    };
}

encode!(svm_sdk_std::Vec<u8>);

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
