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
