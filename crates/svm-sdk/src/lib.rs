#![no_std]
#![feature(maybe_uninit_uninit_array)]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate implements SDK procedural-macros for writing apps (templates if to be more precise) on the SVM platform.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust standard library) in order to reduce the compiled WASM size.
///
/// ### `#[app]` proc-macro:
///
/// The root procedural-macro is `[app]` and it should decorate a Rust module.
/// Here is an example for a minimum App:
///
/// ```rust
/// use svm_sdk::app;
///
/// #[app]
/// mod App {
/// }
/// ```
///
/// Generally, each app should have a way to manage its own storage.
/// And that's what we'll cover now - the `#[storage]` proc-macro.
///
/// ### `#[storage]`` proc-macro
///
/// The `#[storage]` proc-macro attribute consumes a struct and translates
/// its field into more low-level code that interacts against the `svm-sdk` Storage.
/// For testing purposes the storage used will be `MockStorage` and `ExtStorage` otherwise.
///
/// In case the storage field type isn't supported, a compile-time error will be raised.
/// For each field a corresponding getter and setter methods will be generated.
///
/// Here is a simple example of declaring a storage:
///
/// ```rust
/// use svm_sdk::{app, Amount, Address};
///
/// #[app]
/// mod App {
///   #[storage]
///   struct MyStorage {
///     amount: Amount,
///     addr: Address,
///     data: [u32; 3],
///   }
/// }
/// ```
///
/// The above `MyStorage` struct code will be translated (roughly) in compile-time
/// to the following lower-level code:
///
/// ```rust
/// use svm_sdk::{Amount, Address};
///
/// #[cfg(feature = "ffi")]
/// use svm_sdk::storage::ExtStorage as StorageImpl;
///
/// #[cfg(feature = "mock")]
/// use svm_sdk::storage::MockStorage as StorageImpl;
///
/// struct MyStorage;
///
/// impl MyStorage {
///   fn get_amount() -> Amount {
///       svm_sdk::storage::ops::get_amount::<StorageImpl>(0)
///   }
///
///   fn set_amount(value: Amount) {
///       svm_sdk::storage::ops::set_amount::<StorageImpl>(0, value);
///   }
///
///   fn get_addr() -> Address {
///       svm_sdk::storage::ops::get_addr::<StorageImpl>(0)
///   }
///
///   fn set_addr(value: &Address) {
///       svm_sdk::storage::ops::set_addr::<StorageImpl>(1, value);
///   }
///
///   fn get_data(index: usize) -> u32 {
///       let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(2, index, 3);
///       value as u32
///   }
///
///   fn set_data(index: usize, value: u32) {
//        svm_sdk::storage::ops::array_set_addr::<StorageImpl>(2, index, 3, value)
///   }
/// }
/// ```
///
/// The `#[storage]` attribute consumes a struct and translates
/// its field into more low-level code that interacts against the `svm-sdk` Storage.
/// For testing purposes the storage used will be `MockStorage` and `ExtStorage` otherwise.
///
/// In case the storage field type isn't supported, a compile-time error will be raised.
/// For each field a corresponding getter and setter methods will be generated.
///
/// Here is a simple example of declaring a storage:
///
/// ```rust
/// use svm_sdk::{app, Amount, Address};
///
/// #[app]
/// mod App {
///   #[storage]
///   struct MyStorage {
///     amount: Amount,
///     addr: Address,
///     data: [u32; 3],
///   }
/// }
/// ```
///
/// The above `MyStorage` struct code will be translated (roughly) in compile-time
/// to the following lower-level code:
///
/// ```rust
/// use svm_sdk::{Amount, Address};
///
/// #[cfg(feature = "ffi")]
/// use svm_sdk::storage::ExtStorage as StorageImpl;
///
/// #[cfg(feature = "mock")]
/// use svm_sdk::storage::MockStorage as StorageImpl;
///
/// struct MyStorage;
///
/// impl MyStorage {
///   fn get_amount() -> Amount {
///       svm_sdk::storage::ops::get_amount::<StorageImpl>(0)
///   }
///
///   fn set_amount(value: Amount) {
///       svm_sdk::storage::ops::set_amount::<StorageImpl>(0, value);
///   }
///
///   fn get_addr() -> Address {
///       svm_sdk::storage::ops::get_addr::<StorageImpl>(0)
///   }
///
///   fn set_addr(value: &Address) {
///       svm_sdk::storage::ops::set_addr::<StorageImpl>(1, value);
///   }
///
///   fn get_data(index: usize) -> u32 {
///       let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(2, index, 3);
///       value as u32
///   }
///
///   fn set_data(index: usize, value: u32) {
//        svm_sdk::storage::ops::array_set_addr::<StorageImpl>(2, index, 3, value)
///   }
/// }
/// ```
///
/// Besides `#[storage]` each app should expose a public API for the platform, otherwise
/// no one can use it - that's the role of the endpoints.
///
/// ### `[endpoint]` proc-macro:
///
/// The `#[endpoint]` attribute facilitates the task of implementing SVM app's endpoint.
/// Each function annotated with this proc-macro will be transformed into a WASM function export in the compiler's final output.
///
/// # Example
///
/// Here is an example using `#[endpoint]`:
///
/// ```rust
/// use svm_sdk::{app, Amount, Address};
///
/// #[app]
/// mod App {
///   #[endpoint]
///   fn work(a: Amount, to_double: bool) -> Amount {
///     if to_double {
///         a * Amount(2)
///     }
///     else {
///         a
///     }
///   }
/// }
/// ```
///
/// The above method will be translated (roughly) to the following code:
///
/// ```rust
/// use svm_sdk::Amount;
///
/// #[no_mangle]
/// pub extern "C" fn work() {
///     use svm_sdk::traits::Host;
///
///     #[cfg(feature = "mock")]
///     use svm_sdk::host::MockHost as Node;
///
///     #[cfg(feature = "ffi")]
///     use svm_sdk::host::ExtHost as Node;
///
///     fn __inner__() -> Amount {
///         use svm_sdk::CallData;
///
///         let bytes = Node.get_calldata();
///         let mut calldata = CallData::new(bytes);
///
///         let a: Amount = calldata.next_1();
///         let to_double: bool = calldata.next_1();
///
///         if to_double {
///             a * Amount(2)
///         }
///         else {
///             a
///         }
///     }
///
///     {
///         use svm_sdk::traits::Encoder;
///
///         let mut bytes = Vec::new();
///
///         let rets = __inner__();
///         rets.encode(&mut bytes);
///
///         Node.set_returndata(&bytes);
///     }
/// }
/// ```
///
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

pub use svm_abi_decoder::{CallData, DecodeError, ReturnData};
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
