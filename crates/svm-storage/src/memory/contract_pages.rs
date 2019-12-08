use crate::contract_pages::ContractPages;
use crate::default::{DefaultPageHasher, DefaultStateHasher};

use svm_kv::memory::MemKVStore;

/// A `ContractPages` implementation backed by `MemKVStore` kv-store.
pub type MemContractPages = ContractPages<MemKVStore, DefaultPageHasher, DefaultStateHasher>;
