/// Default `StateHasher` implementation.
mod state_hasher;

/// Default `PageAddrHasher` implementation.
mod page_addr_hasher;

/// Default `PageHasher` implementation.
mod page_hasher;

/// Default `PagesStorage` implementation.
mod pages_storage;

/// Default `PageCache` implementation.
mod page_cache;

pub use page_addr_hasher::DefaultPageAddrHasher;
pub use page_cache::DefaultPageCache;
pub use page_hasher::DefaultPageHasher;
pub use pages_storage::DefaultPagesStorage;
pub use state_hasher::DefaultStateHasher;
