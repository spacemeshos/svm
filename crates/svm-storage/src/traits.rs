use crate::page::{PageHash, PageIndex};

use svm_common::{Address, State};

/// `PagesStorage` is the most low-level trait for dealing with a app's storage.
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
}

/// Implementors are in-charge of calculating a page hash.
pub trait PageHasher {
    /// Hashes the contents of a page. (regardless of its address).
    #[must_use]
    fn hash(data: &[u8]) -> PageHash;
}

/// Determines the app's `State` given its pages-hashes (see also `PageHasher`).
pub trait StateHasher {
    /// `pages_hash` - a slice of `PageHash`
    #[must_use]
    fn hash(pages_hash: &[PageHash]) -> State;
}

/// This trait should be implemented by state-oriented pages storage.
/// Since an app must have a state (like a source control revision) we need to have this
/// capability implemented for real-usage app storage.
pub trait StateAwarePagesStorage: PagesStorage {
    /// Returns the current storage state (i.e revision)
    #[must_use]
    fn get_state(&self) -> State;

    /// Returns the page-hash of a given page indexed by `page_idx`
    #[must_use]
    fn get_page_hash(&self, page_idx: PageIndex) -> PageHash;
}

/// `PageCache` is a marker trait intended for subclassing the  StateAwarePagesStorage` trait.
/// It's intended to mark a  StateAwarePagesStorage` as having a caching layer on top of the backed pages-storage.
pub trait PageCache: StateAwarePagesStorage {}
