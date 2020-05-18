use std::collections::HashMap;

mod raw;
use raw::{RawChange, RawStorage};

mod kv;
pub use kv::AppKVStore;

use crate::layout::{DataLayout, VarId};

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

impl AppStorage {
    /// New instance for managing app's variabled specified by `layout`.
    /// App's storage is backed by key-value store `kv`.
    pub fn new(layout: DataLayout, app_kv: AppKVStore) -> Self {
        Self {
            layout,
            raw_storage: RawStorage::new(app_kv, 32),
            uncommitted: HashMap::new(),
        }
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
    pub fn commit(&mut self) {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use svm_common::Address;

    macro_rules! app_kv {
        ($app_addr:expr) => {{
            use std::{cell::RefCell, rc::Rc};

            use crate::app::AppKVStore;
            use crate::kv::FakeKV;

            use svm_kv::traits::KVStore;

            let raw_kv: Rc<RefCell<dyn KVStore>> = Rc::new(RefCell::new(FakeKV::new()));
            AppKVStore::new($app_addr, &raw_kv)
        }};
    }

    macro_rules! assert_vars {
        ($app:expr, $($var_id:expr => $expected:expr), *) => {{
            $(
                let actual = $app.read_var(VarId($var_id));
                assert_eq!(actual, $expected);
             )*
        }};
    }

    macro_rules! write_vars {
        ($app:expr, $($var_id:expr => $value:expr), *) => {{
            $(
                $app.write_var(VarId($var_id), $value.to_vec());
             )*
        }};
    }

    #[test]
    fn app_vars_are_persisted_on_commit() {
        // `var #0` consumes 4 bytes
        // `var #1` consumes 2 bytes
        let layout: DataLayout = vec![4, 2].into();
        let addr = Address::of("my-app");
        let kv = app_kv!(addr);

        // we create clones for later
        let layout_clone2 = layout.clone();
        let layout_clone3 = layout.clone();
        let kv_clone2 = Rc::clone(&kv.raw_kv);
        let kv_clone3 = Rc::clone(&kv.raw_kv);

        let mut app = AppStorage::new(layout, kv);

        // vars are initialized with zeros
        assert_vars!(app, 0 => [0, 0, 0, 0], 1 => [0, 0]);

        write_vars!(app, 0 => [10, 20, 30, 40], 1 => [50, 60]);

        // vars latest version are in memory
        assert_vars!(app, 0 => [10, 20, 30, 40], 1 => [50, 60]);

        // spin a new app with no in-memory dirty data
        let addr = Address::of("my-app");
        let kv2 = AppKVStore::new(addr, &kv_clone2);
        let app2 = AppStorage::new(layout_clone2, kv2);
        assert_vars!(app2, 0 => [0, 0, 0, 0], 1 => [0, 0]);

        // now, we'll persist `app` dirty changes
        app.commit();

        // we'll spin a new app with no caching
        let addr = Address::of("my-app");
        let kv3 = AppKVStore::new(addr, &kv_clone3);
        let mut app3 = AppStorage::new(layout_clone3, kv3);
        write_vars!(app3, 0 => [10, 20, 30, 40], 1 => [50, 60]);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn write_var_value_should_match_layout_length() {
        // `var #0` consumes 4 bytes
        let layout: DataLayout = vec![4].into();
        let addr = Address::of("my-app");
        let kv = app_kv!(addr);

        let mut app = AppStorage::new(layout, kv);
        app.write_var(VarId(0), vec![0, 0]);
    }
}
