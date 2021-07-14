use indexmap::IndexMap;

use crate::FuncIndex;

/// Stores a mapping between an export name to its corresponding function index.
#[derive(Debug, Clone, Default)]
pub struct Exports {
    inner: IndexMap<String, FuncIndex>,
}

impl Exports {
    /// Insert a mapping between `export_naem` to `fn_index`
    pub fn insert<S: Into<String>>(&mut self, export_name: S, fn_index: FuncIndex) {
        self.inner.insert(export_name.into(), fn_index);
    }

    pub fn contains(&self, func_name: &str) -> bool {
        self.inner.contains_key(func_name)
    }

    pub fn get(&self, func_name: &str) -> Option<FuncIndex> {
        self.inner.get(func_name).copied()
    }

    /// Returns the number of imports mapped
    pub fn count(&self) -> usize {
        self.inner.len()
    }
}
