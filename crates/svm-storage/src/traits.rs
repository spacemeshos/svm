use crate::page::{PageHash, PageIndex};
use crate::state::StateHash;
use svm_common::{Address, State};

/// `PagesStorage` is the most low-level trait for dealing with a contract's storage.
/// For performance concerns, we work on pages units (a page is 4096 bytes)
/// Each read / write operation will involve exactly one page
/// That is flushed to the underlying database only when calling `commit`
pub trait PagesStorage {
    /// Retrieves the content of page indexed `page_idx` (Optional)
    #[must_use]
    fn read_page(&mut self, page_idx: PageIndex) -> Option<Vec<u8>>;

    /// Overrides the page indexed `page_idx` with the content of `data` (and marking it as `dirty`)
    /// Important: does NOT persist new page version yet (see: `commit`)
    fn write_page(&mut self, page_idx: PageIndex, data: &[u8]);

    /// Clears all the in-memory cached pages. (main usage: for tests)
    fn clear(&mut self);

    /// Persist the pending in-memory dirty pages into the backed database
    fn commit(&mut self);

    fn close(&mut self);
}

/// `PageIndexHasher` is a trait defining that a contract storage-page hash must be determined by
/// both the contract storage and the page index.
///
/// We must have both parameters taken into account since:
/// * Computing a page-hash for two different contracts and the same `page index` must result in a different page-hash.
///   That's why we need the contract address.
///
/// * Similarly, computing a page-hash two variables located at different storage-pages under the same contract
/// must also result in a different page-hash.
pub trait PageIndexHasher {
    /// Calculates a hash derived from an `address` + a `page-index`
    #[must_use]
    fn hash(address: Address, page_idx: PageIndex) -> [u8; 32];
}

/// Implementors are in-charge of calculating a page hash.
/// The page hash is derived from 3 components: `contract address` + `page-index` + `page-data`
pub trait PageHasher {
    /// `address`  - The Smart Contract account address
    /// `page_idx` - The page index we want to calculate its hash
    /// `page_data - The raw content of the page
    #[must_use]
    fn hash(address: Address, page_idx: PageIndex, page_data: &[u8]) -> PageHash;
}

/// Implementors are in-charge of calculating a page hash.
/// The page hash isderived from 3 components: `contract address` + `page-index` + `page-data`
pub trait StateHasher {
    /// `pages_hash` - a slice of `PageHash`
    #[must_use]
    fn hash(pages_hash: &[PageHash]) -> StateHash;
}

/// This trait should be implemented by state-oriented pages storage.
/// Since a Smart Contract must have a state (like a source control revision) we need to have this
/// capability implemented for real-usage Smart Contract storage.
pub trait PagesStateStorage: PagesStorage {
    /// Returns the current storage state (i.e revision)
    #[must_use]
    fn get_state(&self) -> State;

    /// Returns the page-hash of a given page indexed by `page_idx`
    #[must_use]
    fn get_page_hash(&self, page_idx: PageIndex) -> PageHash;
}

/// `PageCache` is a marker trait intended for subclassing the `PagesStateStorage` trait.
/// It's intended to mark a `PagesStateStorage` as having a caching layer on top of the backed pages-storage.
pub trait PageCache: PagesStateStorage {}
