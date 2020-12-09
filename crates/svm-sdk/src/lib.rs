#![no_std]
#![feature(maybe_uninit_uninit_array)]

//! This crate implements SDK for SVM.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust stdlib) annotation in order to reduce the compiled WASM size.

#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod log;

/// Logging API
pub use log::log;

/// `ensure` macro
#[macro_use]
pub mod ensure;

use svm_sdk_alloc;
use svm_sdk_macros;
use svm_sdk_storage;
use svm_sdk_types;

pub use svm_sdk_macros::app;

pub use svm_sdk_alloc::{alloc, Ptr};

pub use svm_abi_decoder::{CallData, DecodeError, ReturnData};

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
    pub use svm_abi_encoder::Encoder;
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

pub use svm_sdk_types::*;
