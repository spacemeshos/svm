#![allow(unused)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {
        extern crate proc_macro;

        mod storage;
        mod endpoint;

        use storage::parse_storage;
        use endpoint::parse_endpoint;

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
        ///
        /// use svm_sdk::{Amount, Address};
        /// use svm_sdk_macros::storage;
        ///
        /// #[storage]
        /// struct MyStorage {
        ///   amount: Amount,
        ///   addr: Address,
        ///   data: [u32; 3],
        /// }
        /// ```
        ///
        /// The above `MyStorage` struct code will be translated (roughly) in compile-time
        /// to the following lower-level code:
        ///
        /// ```rust
        /// use svm_sdk::{Amount, Address};
        ///
        /// #[cfg(not(test))]
        /// use svm_sdk::storage::ExtStorage as StorageImpl;
        ///
        /// #[cfg(test)]
        /// use svm_sdk::storage::MockStorage as StorageImpl;
        ///
        /// struct MyStorage;
        ///
        /// impl MyStorage {
        ///   fn get_amount() -> Amount {
        ///       svm_sdk::storage::get_amount::<StorageImpl>(0)
        ///   }
        ///
        ///   fn set_amount(value: Amount) {
        ///       svm_sdk::storage::set_amount::<StorageImpl>(0, value);
        ///   }
        ///
        ///   fn get_addr() -> Address {
        ///       svm_sdk::storage::get_addr::<StorageImpl>(0)
        ///   }
        ///
        ///   fn set_addr(value: &Address) {
        ///       svm_sdk::storage::set_addr::<StorageImpl>(1, value);
        ///   }
        ///
        ///   fn get_data(index: usize) -> u32 {
        ///       let value = svm_sdk::storage::array_get32::<StorageImpl>(2, index, 3);
        ///       value as u32
        ///   }
        ///
        ///   fn set_data(index: usize, value: u32) {
        //        svm_sdk::storage::array_set_addr::<StorageImpl>(2, index, 3, value)
        ///   }
        /// }
        /// ```
        ///
        #[proc_macro_attribute]
        pub fn storage(
            _args: proc_macro::TokenStream,
            input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            parse_storage(input)
        }

        #[proc_macro_attribute]
        pub fn endpoint(
            _args: proc_macro::TokenStream,
            input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            parse_endpoint(input)
        }
    }
}

#[proc_macro_attribute]
pub fn endpoint(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    parse_endpoint(args, input)
}
