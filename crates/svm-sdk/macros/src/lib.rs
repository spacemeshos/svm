#![allow(unused)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(windows))] {
        extern crate proc_macro;

        mod storage;
        mod endpoint;
        mod app;
        mod attr;
        mod function;

        use function::Function;
        use attr::{FuncAttribute, FuncAttrKind};

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
        /// ``` rust
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
        ///
        /// The `#[endpoint]` proc-macro attribute facilitates the task of implementing SVM app's endpoint.
        /// Each function annotated with this proc-macro will be transformed into a WASM function export in the compiler's final output.
        ///
        /// # Example
        ///
        /// Here is an example using `#[endpoint]`:
        ///
        /// ```rust
        /// use svm_sdk::{Amount, Address};
        /// use svm_sdk_macros::endpoint;
        ///
        /// #[endpoint]
        /// fn work(a: Amount, to_double: bool) -> Amount {
        ///     if to_double {
        ///         a * Amount(2)
        ///     }
        ///     else {
        ///         a
        ///     }
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
        ///     #[cfg(test)]
        ///     use svm_sdk::host::MockHost as Node;
        ///
        ///     #[cfg(not(test))]
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
        #[proc_macro_attribute]
        pub fn app(
            args: proc_macro::TokenStream,
            input: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            match app::transform(args.into(), input.into()) {
                Err(err) => {
                    dbg!(err);

                    panic!("...")
                }
                Ok(output) => output.into()
            }
        }
    }
}
