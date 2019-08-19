use crate::default::DefaultPageCache;
use crate::memory::MemMerklePages;

/// `DefaultPageCache` implementation backed by `MemMerklePages` pages-storage.
pub type MemMerklePageCache<'pc> = DefaultPageCache<'pc, MemMerklePages>;
