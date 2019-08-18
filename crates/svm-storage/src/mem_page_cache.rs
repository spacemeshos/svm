use crate::{
    default::DefaultPageCache,
    memory::{MemMerklePages, MemPages},
};

/// `MemPageCache` is a `DefaultPageCache` backed by `MemPages` pages-storage
pub type MemPageCache<'pc> = DefaultPageCache<'pc, MemMerklePages>;

/// `MemPageCache` is a `DefaultPageCache` backed by `MemPages` pages-storage with 32 bytes-length keys
pub type MemPageCache32<'pc> = MemPageCache<'pc>;
