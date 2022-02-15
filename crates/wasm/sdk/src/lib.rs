#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate implements SDK procedural-macros for writing Templates on the SVM platform.
//! Using this crate when writing SVM Templates in Rust isn't mandatory but should be very useful.
//!
//! The crate is compiled with `![no_std]` (no Rust standard library) in order to reduce the compiled WASM size.
//!
//! ### `#[template]` proc-macro:
//!
//! The root procedural-macro is `[template]` and it should decorate a Rust module.
//! Here is an example for a minimal Template:
//!
//! ```rust, no_run
//! use svm_sdk::template;
//!
//! #[template]
//! mod MyTemplate {
//! }
//! ```
//!
//! Generally, each Template should have a way to manage its own storage.
//! And that's what we'll cover now - the `#[storage]` proc-macro.
//!
//! ### `#[storage]`` proc-macro
//!
//! The `#[storage]` proc-macro attribute consumes a struct and translates
//! its field into more low-level code that interacts against the `svm-sdk` Storage.
//! For testing purposes the storage used will be `MockStorage` and `ExtStorage` otherwise.
//!
//! In case the storage field type isn't supported, a compile-time error will be raised.
//! For each field a corresponding getter and setter methods will be generated.
//!
//! Here is a simple example of declaring a storage:
//!
//! ```rust, no_run
//! use svm_sdk::{template, Amount, Address};
//!
//! #[template]
//! mod MyTemplate {
//!   #[storage]
//!   struct MyStorage {
//!     amount: Amount,
//!     addr: Address,
//!     data: [u32; 3],
//!   }
//! }
//! ```
//!
//! The above `MyStorage` struct code will be translated (roughly) in compile-time
//! to the following lower-level code:
//!
//! ```rust, no_run
//! use svm_sdk::{Amount, Address};
//!
//! use svm_sdk::storage::ExtStorage as StorageImpl;
//!
//! struct MyStorage;
//!
//! impl MyStorage {
//!   fn get_amount() -> Amount {
//!       svm_sdk::storage::ops::get_amount::<StorageImpl>(0)
//!   }
//!
//!   fn set_amount(value: Amount) {
//!       svm_sdk::storage::ops::set_amount::<StorageImpl>(0, value);
//!   }
//!
//!   fn get_addr() -> Address {
//!       svm_sdk::storage::ops::get_addr::<StorageImpl>(0)
//!   }
//!
//!   fn set_addr(value: &Address) {
//!       svm_sdk::storage::ops::set_addr::<StorageImpl>(1, value);
//!   }
//!
//!   fn get_data(index: usize) -> u32 {
//!       let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(2, index, 3);
//!       value as u32
//!   }
//!
//!   fn set_data(index: usize, value: u32) {
//!       svm_sdk::storage::ops::array_set32::<StorageImpl>(2, index, 3, value);
//!   }
//! }
//! ```
//!
//! The `#[storage]` attribute consumes a struct and translates
//! its field into more low-level code that interacts against the `svm-sdk` Storage.
//! For testing purposes the storage used will be `MockStorage` and `ExtStorage` otherwise.
//!
//! In case the storage field type isn't supported, a compile-time error will be raised.
//! For each field a corresponding getter and setter methods will be generated.
//!
//! Here is a simple example of declaring a storage:
//!
//! ```rust, no_run
//! use svm_sdk::{template, Amount, Address};
//!
//! #[template]
//! mod MyTemplate {
//!   #[storage]
//!   struct MyStorage {
//!     amount: Amount,
//!     addr: Address,
//!     data: [u32; 3],
//!   }
//! }
//! ```
//!
//! The above `MyStorage` struct code will be translated (roughly) in compile-time
//! to the following lower-level code:
//!
//! ```rust, no_run
//! use svm_sdk::{Amount, Address};
//!
//! use svm_sdk::storage::ExtStorage as StorageImpl;
//!
//! struct MyStorage;
//!
//! impl MyStorage {
//!   fn get_amount() -> Amount {
//!       svm_sdk::storage::ops::get_amount::<StorageImpl>(0)
//!   }
//!
//!   fn set_amount(value: Amount) {
//!       svm_sdk::storage::ops::set_amount::<StorageImpl>(0, value);
//!   }
//!
//!   fn get_addr() -> Address {
//!       svm_sdk::storage::ops::get_addr::<StorageImpl>(0)
//!   }
//!
//!   fn set_addr(value: &Address) {
//!       svm_sdk::storage::ops::set_addr::<StorageImpl>(1, value);
//!   }
//!
//!   fn get_data(index: usize) -> u32 {
//!       let value = svm_sdk::storage::ops::array_get32::<StorageImpl>(2, index, 3);
//!       value as u32
//!   }
//!
//!   fn set_data(index: usize, value: u32) {
//!       svm_sdk::storage::ops::array_set32::<StorageImpl>(2, index, 3, value);
//!   }
//! }
//! ```
//!
//! Besides `#[storage]` each Template should expose a public API for the platform, otherwise
//! no one can use it - that's the role of the endpoints.
//!
//! ### `[endpoint]` proc-macro:
//!
//! The `#[endpoint]` attribute facilitates the task of implementing Template's endpoints.
//! Each function annotated with this proc-macro will be transformed into a WASM function export in the compiler's final output.
//!
//! # Example
//!
//! Here is an example using `#[endpoint]`:
//!
//! ```rust, no_run
//! use svm_sdk::{template, Amount, Address};
//!
//! #[template]
//! mod MyTemplate {
//!   #[endpoint]
//!   fn work(a: Amount, to_double: bool) -> Amount {
//!     if to_double {
//!         a * Amount(2)
//!     }
//!     else {
//!         a
//!     }
//!   }
//! }
//! ```
//!
//! The above method will be translated (roughly) to the following code:
//!
//! ```rust, no_run
//! use svm_sdk::Amount;
//!
//! #[no_mangle]
//! pub extern "C" fn work() {
//!     use svm_sdk::traits::Host;
//!     use svm_sdk::host::ExtHost as Node;
//!
//!     fn __inner__() -> Amount {
//!         use svm_sdk::CallData;
//!
//!         let bytes = Node.calldata();
//!         let mut calldata = CallData::new(bytes);
//!
//!         let a: Amount = calldata.next_1();
//!         let to_double: bool = calldata.next_1();
//!
//!         if to_double {
//!             a * Amount(2)
//!         }
//!         else {
//!             a
//!         }
//!     }
//!
//!     {
//!         use svm_sdk::traits::{Encoder, ByteSize};
//!
//!         let cap = Amount::max_byte_size();
//!         let mut bytes = svm_sdk::Vec::with_capacity(cap);
//!
//!         let rets = __inner__();
//!         rets.encode(&mut bytes);
//!
//!         Node.set_returndata(&bytes);
//!     }
//! }
//! ```
//!
//! ### Funding
//!
//! Each Account (instance of a Template) is holds a balance.
//! When calling an `Account`'s endpoint, the Transaction's `value` field is allowed to be positive.
//!
//! In such case we'd like to let the running Account a chance to be notified about the funding
//! and let it invoke some arbitrary hook to update its state.
//!
//! Thus, each `#[endpoint]` might be annotated with an additional `#[fundable(..)]` attribute.  
//! In addition to that `#[fundable_hook]` should be added so that the `#[fundable(..)]` will use them.
//!
//! Here is an example:
//!
//! ```rust, no_run
//! use svm_sdk::template;
//!
//! #[template]
//! mod MyTemplate {
//!   #[storage]
//!   struct Storage {
//!     coins: Amount
//!   }
//!
//!   #[fundable]
//!   #[endpoint]
//!   fn do_nothing() {}
//!
//!   #[fundable_hook(default)]
//!   fn fund() {
//!     let value = Node::value();
//!
//!     let old_coins = Storage::get_coins();
//!     let new_coins = old_coins + value;
//!
//!     Storage::set_coins(new_coins);
//!   }
//! }
//! ```
//!
//! If we invoke SVM transaction over function `do_nothing` and `value = 100`
//! What logically happens behind-the-scenes is:
//!
//! 1) Since `value > 0` - the transaction `principal` transfers `100` coins
//!  to the `Account`'s balance. (assuming the transaction is valid and verified).
//!
//! 2) Now, SVM call the `Account`'s `update_coins` with `value = Amount(100)`.
//! The reason that this is the fundable-hook to be called is since it's being
//! referenced by the `#[fundable(..)]` of `do_nothing` endpoint.
//!
//! The running of `update_coins` gives the `Account` a chance to update it's state.
//! In our example it updates the `coins` field. That means that real balance
//! of an `Account` in any given point will be at-least the value of the `coins` field.
//!
//! 3) The `do_nothing` endpoint code is being executed.
//!
//!
//! The way things truly works implementation-wise looks more like this:
//!
//! ```rust, no_run
//! fn update_coins(value: svm_sdk::Amount) {
//!   // ...
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn do_nothing() {
//!     use svm_sdk::traits::Host;
//!     use svm_sdk::host::ExtHost as Node;
//!
//!     // we grab the `value` given in the transaction
//!     // if it's positive, we issue a call to `update_coins`
//!     let value: svm_sdk::Amount = Node.value();
//!
//!     if value > svm_sdk::Amount(0) {
//!       update_coins(value);
//!     }
//!
//!     fn __inner__() {
//!        // the logic of `do_nothing` (empty in our example)
//!     }
//!
//!     {
//!         use svm_sdk::traits::{Encoder, ByteSize};
//!
//!         let cap = <()>::max_byte_size();
//!         let mut bytes = svm_sdk::Vec::with_capacity(cap);
//!
//!         let rets = __inner__();
//!         rets.encode(&mut bytes);
//!
//!         Node.set_returndata(&bytes);
//!     }
//! }
//! ```

// Logging API
pub use svm_abi_decoder::{CallData, DecodeError, ReturnData};
pub use svm_sdk_macros::template;

// std
pub use svm_sdk_std::{ensure, log, panic, Option, Result, Vec};

// alloc
//
// exposing the `global allocator` by using the `extern crate` syntax.
extern crate svm_sdk_alloc;

pub use svm_sdk_alloc::{alloc, Ptr};

pub mod host {
    pub use svm_sdk_host_ffi::ExtHost;
}

pub mod traits {
    pub use svm_abi_encoder::{ByteSize, Encoder};
    pub use svm_sdk_host::Host;
    pub use svm_sdk_storage::Storage;
}

pub mod storage {
    pub use svm_sdk_storage_ffi::ExtStorage;

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
