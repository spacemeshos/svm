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
use svm_sdk_storage;
use svm_sdk_types;

pub use svm_sdk_alloc::{alloc, Ptr};

pub mod host {
    pub use svm_sdk_host::{ExtHost, MockHost};

    pub use svm_sdk_host::traits;
}

pub mod storage {
    pub use svm_sdk_storage::{ExtStorage, MockStorage};

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

    pub mod traits {
        pub use svm_sdk_storage::Storage;
    }
}

pub use svm_sdk_types::*;
