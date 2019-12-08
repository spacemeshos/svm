use crate::default::DefaultPageCache;
use crate::memory::MemContractPages;

/// `DefaultPageCache` implementation backed by `MemContractPages` pages-storage.
pub type MemContractPageCache = DefaultPageCache<MemContractPages>;
