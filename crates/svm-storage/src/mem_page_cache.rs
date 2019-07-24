use crate::{
    default::DefaultPageCache,
    memory::{MemKVStore, MemPages},
};

/// `MemPageCache` is a `DefaultPageCache` backed by `MemPages` pages-storage
pub type MemPageCache<'pc, K> = DefaultPageCache<'pc, MemPages<K>>;

/// `MemPageCache` is a `DefaultPageCache` backed by `MemPages` pages-storage with 32 bytes-length keys
pub type MemPageCache32<'pc> = MemPageCache<'pc, [u8; 32]>;
