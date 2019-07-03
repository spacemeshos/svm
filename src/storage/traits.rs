use crate::common::Address;

/// `KVStore` is a trait for defining an interface against key-value stores. for example `hashmap / leveldb / rocksdb`
pub trait KVStore {
    type K: AsRef<[u8]> + Copy + Clone + std::cmp::PartialEq + Sized;

    #[must_use]
    fn get(&self, key: Self::K) -> Vec<u8>;

    fn store(&mut self, key: Self::K, value: &[u8]);
}

/// `StoragePages` is the most low-level trait for dealing with a contract's storage.
/// For performance concerns, we work on pages units (a page is 4096 bytes)
/// Each read / write operation will involve exactly one page
/// That is flushed to the underlying database only when calling `commit`
pub trait StoragePages {
    #[must_use]
    fn read_page(&mut self, page: u32) -> Vec<u8>;

    fn write_page(&mut self, page: u32, data: &[u8]);

    fn clear(&mut self);

    fn commit(&mut self);
}

/// `StoragePageHasher` is a trait defining that a contract storage-page hash must be determied by
/// both the contract storage and the page index.
///
/// We must have both parameters taken into account since:
/// * Computing a page-hash for two differnt contracts and the same `page index` must result in a different page-hash.
///   That's why we need the contract address.
///
/// * Similarly, computing a page-hash two variables located at different storage-pages under the same contract
/// must also result in a different page-hash.
pub trait StoragePageHasher {
    #[must_use]
    fn hash(address: Address, page: u32) -> [u8; 32];
}

/// The `declare_storage_api` will be imported by the smart contract wasm programs.
/// Reading data will be copied from the backing storage into temporary buffers.
/// Then the smart contract program will copy that data into the wasm instance's memory / stack.
#[allow(unused)]
macro_rules! declare_storage_api {
    () => {{
        extern "C" {
            fn mem_copy_to_reg_copy(reg: i32, mem_ptr: i32, mem_len: i32);

            fn reg_copy_to_mem_copy(reg: i32, mem_ptr: i32, mem_len: i32);

            fn storage_read_to_reg(var_page: i32, var_page_offset: i32, var_len: i32, reg: i32);

            fn storage_set_from_reg(var_page: i32, var_page_offset: i32, reg: i32);
        }
    }};
}
