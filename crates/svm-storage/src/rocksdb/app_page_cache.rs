use crate::default::DefaultPageCache;
use crate::rocksdb::RocksdbAppPages;

/// `DefaultPageCache` implementation backed by `RocksdbAppPages` pages-storage.
pub type RocksdbAppPageCache = DefaultPageCache<RocksdbAppPages>;
