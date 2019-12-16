use crate::default::DefaultPageCache;
use crate::rocksdb::RocksdbContractPages;

/// `DefaultPageCache` implementation backed by `RocksdbContractPages` pages-storage.
pub type RocksdbContractPageCache = DefaultPageCache<RocksdbContractPages>;
