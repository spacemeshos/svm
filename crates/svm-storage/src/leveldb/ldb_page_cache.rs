use crate::default::DefaultPageCache;
use crate::leveldb::LDBPages;

/// `DefaultPageCache` implementation backed by `LDBPages` pages-storage.
pub type LDBMerklePageCache<'pc> = DefaultPageCache<'pc, LDBPages>;
