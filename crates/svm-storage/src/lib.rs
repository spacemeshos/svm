#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! `svm-storage` crate is responsible for the app-storage part of the `SVM`
//! Each app has its own storage

/// Default implementations for crate traits (see `traits.rs`).
pub mod default;

mod app_pages;
mod app_storage;

/// Contains definitions of `Page` related structures. For example: `Page, PageIndex` etc
pub mod page;

/// Contains definitions `State`-related.
pub mod state;

pub use crate::app_pages::AppPages;
pub use crate::app_storage::AppStorage;

/// Storage related traits
pub mod traits;

/// Tests related helpers and asserts
#[macro_use]
pub mod testing;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "svm_memory")] {
        /// in-memory backed implementation for storage
        pub mod memory;
    }
}

cfg_if! {
    if #[cfg(feature = "svm_rocksdb")]  {
        /// `rocksdb` backed implementation for storage
        pub mod rocksdb;
    }
}
