use crate::default::DefaultPageCache;
use crate::memory::MemAppPages;

/// `DefaultPageCache` implementation backed by `MemAppPages` pages-storage.
pub type MemAppPageCache = DefaultPageCache<MemAppPages>;
