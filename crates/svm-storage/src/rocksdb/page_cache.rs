use crate::default::DefaultPageCache;
use crate::rocksdb::RocksdbPages;

/// `DefaultPageCache` implementation backed by `RocksdbPages` pages-storage.
pub type RocksdbPageCache = DefaultPageCache<RocksdbPages>;
