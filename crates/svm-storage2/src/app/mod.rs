use std::collections::HashMap;

mod raw;
use raw::{RawChange, RawStorage};

mod kv;
pub use kv::AppKVStore;

use svm_common::State;
use svm_layout::{DataLayout, VarId};

///
/// The `AppStorage` manages a running app's storage.
///
/// While an app is running it performs read and write operations.
///
/// Reads operations don't modify the app's storage.
/// Write operations save changes to be commited after an app's execution succeeded.
///
/// Reading a variable will return the most updated value of the variable.
///
/// If app's execution fails - no changes will be persisted (`commit` won't be called).
///
pub struct AppStorage {
    /// Interface to the raw storage (key-value wrapper)
    raw_storage: RawStorage,

    /// App Fixed-Sized variables layout
    layout: DataLayout,

    /// Uncommited changes
    uncommitted: HashMap<VarId, Vec<u8>>,
}

// TODO:
// we need to decide whether `kv_value_size` should be
// part of transaction (next to the `DataLayout`) or a constant value.
const KV_VALUE_SIZE: u32 = 32;

impl AppStorage {
    /// New instance for managing app's variabled specified by `layout`.
    /// App's storage is backed by key-value store `kv`.
    pub fn new(layout: DataLayout, app_kv: AppKVStore) -> Self {
        Self {
            layout,
            raw_storage: RawStorage::new(app_kv, KV_VALUE_SIZE),
            uncommitted: HashMap::new(),
        }
    }

    /// Rewinds the current application `State` to point to `state`.
    #[inline]
    pub fn rewind(&mut self, state: &State) {
        self.raw_storage.rewind(state);
    }

    /// Returns the current `State` of the application.
    #[inline]
    pub fn head(&self) -> State {
        self.raw_storage.head()
    }

    /// Reads variable `var_id`.
    pub fn read_var(&self, var_id: VarId) -> Vec<u8> {
        let var = self.uncommitted.get(&var_id).cloned();

        var.unwrap_or_else(|| {
            let (off, len) = self.var_layout(var_id);

            let bytes = self.raw_storage.read(off, len);

            debug_assert_eq!(bytes.len(), len as usize);

            bytes
        })
    }

    /// Marks variable as `dirty`. Upon `commit` will persist the variable.
    pub fn write_var(&mut self, var_id: VarId, value: Vec<u8>) {
        let (_off, len) = self.var_layout(var_id);

        debug_assert_eq!(value.len(), len as usize);

        self.uncommitted.insert(var_id, value);
    }

    /// Returns the layout of variable `var_id`.
    /// The layout is a tuple of `(offset, length)`.
    #[inline]
    pub fn var_layout(&self, var_id: VarId) -> (u32, u32) {
        self.layout.get_var(var_id)
    }

    /// Commits modified (a.k.a) variables into the raw storage.
    #[must_use]
    pub fn commit(&mut self) -> State {
        let var_offset: HashMap<VarId, u32> = self
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
