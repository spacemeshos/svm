//! High-level `Storage`
use std::collections::HashMap;

mod raw;
use raw::{RawChange, RawStorage};

mod kv;
pub use kv::AccountKVStore;

use svm_layout::{FixedLayout, Id};
use svm_types::State;

///
/// The `AccountStorage` manages a running `Account`'s storage.
///
/// While an `Account` is running it performs read and write operations.
///
/// Reads operations don't modify the `Account`'s storage.
/// Write operations save changes to be committed after an `Account`'s execution succeeded.
///
/// Reading a variable will return the most updated value of the variable.
///
/// If `Account`'s execution fails - no changes will be persisted (`commit` won't be called).
///
pub struct AccountStorage {
    /// Interface to the underlying raw storage.
    raw_storage: RawStorage,

    /// Fixed-Sized variables layout.
    layout: FixedLayout,

    /// Uncommitted changes.
    uncommitted: HashMap<Id, Vec<u8>>,
}

// TODO:
// we need to decide whether `kv_value_size` should be
// part of transaction (next to the `svm_layout::Layout`) or a constant value.
const KV_VALUE_SIZE: u32 = 32;

impl AccountStorage {
    /// New instance for managing an `Account`'s variables specified by `layout`.
    /// `Account`'s storage is backed by key-value store `kv`.
    pub fn new(layout: FixedLayout, account_kv: AccountKVStore) -> Self {
        Self {
            layout,
            raw_storage: RawStorage::new(account_kv, KV_VALUE_SIZE),
            uncommitted: HashMap::new(),
        }
    }

    /// Rewinds the current `Account`'s `State` to point to `state`.
    #[inline]
    pub fn rewind(&mut self, state: &State) {
        self.raw_storage.rewind(state);
    }

    /// Returns the current `Account`'s `State`.
    #[inline]
    pub fn head(&self) -> State {
        self.raw_storage.head()
    }

    /// Reads variable `var_id`.
    pub fn read_var(&self, var_id: Id) -> Vec<u8> {
        let var = self.uncommitted.get(&var_id).cloned();

        var.unwrap_or_else(|| {
            let (off, len) = self.var_layout(var_id);

            let bytes = self.raw_storage.read(off, len);

            debug_assert_eq!(bytes.len(), len as usize);

            bytes
        })
    }

    /// Marks variable as `dirty`. Upon `commit` will persist the variable.
    pub fn write_var(&mut self, var_id: Id, value: Vec<u8>) {
        let (_off, len) = self.var_layout(var_id);

        assert_eq!(value.len(), len as usize);

        self.uncommitted.insert(var_id, value);
    }

    /// Returns the layout of variable `var_id`.
    /// The layout is a tuple of `(offset, length)`.
    #[inline]
    pub fn var_layout(&self, var_id: Id) -> (u32, u32) {
        let var = self.layout.get(var_id);

        (var.offset(), var.byte_size())
    }

    /// Commits modified variables into the raw storage.
    #[must_use]
    pub fn commit(&mut self) -> State {
        let var_offset: HashMap<Id, u32> = self
            .uncommitted
            .keys()
            .map(|var_id| {
                let (off, _len) = self.var_layout(*var_id);

                (*var_id, off)
            })
            .collect();

        let changes = self
            .uncommitted
            .drain()
            .map(|(var_id, data)| {
                let offset = *var_offset.get(&var_id).unwrap();

                RawChange { offset, data }
            })
            .collect::<Vec<_>>();

        self.raw_storage.write(&changes);

        debug_assert!(self.uncommitted.is_empty());

        self.raw_storage.head()
    }
}
