use svm_common::Address;

/// `KVStore` is a trait for defining an interface against key-value stores. for example `hashmap / leveldb / rocksdb`
pub trait KVStore {
    /// `K` stands for `key`.
    type K: AsRef<[u8]> + Copy + Clone + std::cmp::PartialEq + Sized;

    /// Retrieves the value pointed by `key` (Optional).
    #[must_use]
    fn get(&self, key: Self::K) -> Option<Vec<u8>>;

    /// Stores a batch of changes. Each change is `key` -> `value` association.
    fn store(&mut self, changes: &[(Self::K, &[u8])]);
}

/// `PagesStorage` is the most low-level trait for dealing with a contract's storage.
/// For performance concerns, we work on pages units (a page is 4096 bytes)
/// Each read / write operation will involve exactly one page
/// That is flushed to the underlying database only when calling `commit`
pub trait PagesStorage {
    /// Retrieves the content of page indexed `page_idx` (Optional)
    #[must_use]
    fn read_page(&mut self, page_idx: u32) -> Option<Vec<u8>>;

    /// Overrides the page indexed `page_idx` with the content of `data` (and marking it as `dirty`)
    /// Important: does NOT persist new page version yet (see: `commit`)
    fn write_page(&mut self, page_idx: u32, data: &[u8]);

    /// Clears all the in-memory cached pages. (main usage: for tests)
    fn clear(&mut self);

    /// Persist the pending in-memory dirty pages into the backed database
    fn commit(&mut self);
}

/// `PageCache` is a marker trait intended subclassing the `PagesStorage` tarit.
/// It's intended to mark a `PagesStorage` as having a caching layer on top of the backing pages storage.
pub trait PageCache: PagesStorage {}

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
    /// Calculates hash derived from an `address` + a `page`
    #[must_use]
    fn hash(address: Address, page: u32) -> [u8; 32];
}
