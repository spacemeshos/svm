use svm_common::Address;

/// `KVStore` is a trait for defining an interface against key-value stores. for example `hashmap / leveldb / rocksdb`
pub trait KVStore {
    type K: AsRef<[u8]> + Copy + Clone + std::cmp::PartialEq + Sized;

    #[must_use]
    fn get(&self, key: Self::K) -> Option<Vec<u8>>;

    fn store(&mut self, key: Self::K, value: &[u8]);
}

/// `PagesStorage` is the most low-level trait for dealing with a contract's storage.
/// For performance concerns, we work on pages units (a page is 4096 bytes)
/// Each read / write operation will involve exactly one page
/// That is flushed to the underlying database only when calling `commit`
pub trait PagesStorage {
    #[must_use]
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>>;

    fn write_page(&mut self, page_idx: u32, data: &[u8]);

    fn clear(&mut self);

    fn commit(&mut self);
}

/// `PageHasher` is a trait defining that a contract storage-page hash must be determied by
/// both the contract storage and the page index.
///
/// We must have both parameters taken into account since:
/// * Computing a page-hash for two differnt contracts and the same `page index` must result in a different page-hash.
///   That's why we need the contract address.
///
/// * Similarly, computing a page-hash two variables located at different storage-pages under the same contract
/// must also result in a different page-hash.
pub trait PageHasher {
    #[must_use]
    fn hash(address: Address, page: u32) -> [u8; 32];
}

/// The `include_extern_storage` will be imported by the smart contract wasm programs.
/// Reading data will be copied from the backing storage into temporary buffers.
/// Then the smart contract program will copy that data into the wasm instance's memory / stack.
#[macro_export]
macro_rules! include_extern_storage {
    () => {
        extern "C" {
            // memory => buffer
            fn mem_to_buf_copy(src_mem_ptr: i32, dst_buf: i32, offset: i32, len: i32);

            // buffer => memory
            fn buf_to_mem_copy(src_buf: i32, dst_mem_ptr: i32, offset: i32, len: i32);

            // memory => register
            fn mem_to_reg_copy(src_mem_ptr: i32, dst_reg: i32, offset: i32, len: i32);

            // register => memory
            fn reg_to_mem_copy(src_reg: i32, dst_mem_ptr: i32, offset: i32, len: i32);

            // stoarge => register
            fn storage_read_to_reg(src_page: i32, dst_reg: i32, offset: i32, len: i32);

            // register => stoarge
            fn storage_set_from_reg(src_reg: i32, dst_page: i32, offset: i32, len: i32);
        }
    };
}
