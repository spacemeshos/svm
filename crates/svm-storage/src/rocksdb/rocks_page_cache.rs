use crate::default::DefaultPageCache;
use crate::rocksdb::RocksPages;

/// `DefaultPageCache` implementation backed by `RocksPages` pages-storage.
pub type RocksMerklePageCache<'pc> = DefaultPageCache<'pc, RocksPages>;
