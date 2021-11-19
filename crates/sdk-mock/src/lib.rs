#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(all(feature = "static-alloc", feature = "dynamic-alloc"))]
compile_error!("Cannot have both `static-alloc` and `dynamic-alloc` features turned-on");

#[cfg(not(any(feature = "static-alloc", feature = "dynamic-alloc")))]
compile_error!("Must have either `static-alloc` or `dynamic-alloc` features turned-on");

/// Logging API
pub use svm_abi_decoder::{CallData, DecodeError, ReturnData};
pub use svm_sdk_macros::template;

pub use svm_sdk_std::{ensure, log};
/// std
pub use svm_sdk_std::{Option, Result, Vec};

// alloc
//
// exposing the `global allocator` by using the `extern crate` syntax.
extern crate svm_sdk_alloc;

pub use svm_sdk_alloc::{alloc, Ptr};

#[cfg(not(any(feature = "ffi", feature = "mock")))]
compile_error!("must have at least one feature flag turned-on (`ffi` or `mock`)");

#[cfg(all(feature = "ffi", feature = "mock"))]
compile_error!("cannot have both feature-flags `ffi` and `mock` turned-on");

pub mod host {
    #[cfg(feature = "ffi")]
    pub use svm_sdk_host::ExtHost;

    #[cfg(feature = "mock")]
    pub use svm_sdk_host::MockHost;
}

pub mod traits {
    pub use svm_abi_encoder::{ByteSize, Encoder};
    pub use svm_sdk_host::traits::Host;
    pub use svm_sdk_storage::Storage;
}

pub mod storage {
    #[cfg(feature = "ffi")]
    pub use svm_sdk_storage::ExtStorage;

    #[cfg(feature = "mock")]
    pub use svm_sdk_storage::MockStorage;

    pub mod ops {
        #[rustfmt::skip]
        pub use svm_sdk_storage::{
            get32,
            set32,

            get64,
            set64,

            get_bool,
            set_bool,

            get_amount,
            set_amount,

            load160,
            store160,

            get_addr,
            set_addr,

            array_get_bool,
            array_set_bool,

            array_get32,
            array_set32,

            array_get64,
            array_set64,
            
            array_get_amount,
            array_set_amount,

            array_get_addr,
            array_set_addr
        };
    }
}

pub use svm_sdk_types::{Address, Amount, LayerId};
