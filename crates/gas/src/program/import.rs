use indexmap::IndexMap;

use crate::FuncIndex;

/// Stores a mapping between a function index to its corresponding `(module_name, import_name)`
#[derive(Debug)]
pub struct Imports {
    inner: IndexMap<FuncIndex, (String, String)>,
}

impl Default for Imports {
    fn default() -> Self {
        Imports::new()
    }
}

impl Imports {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Creates a new instance and reserve `cap` entries
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(cap),
        }
    }

    /// Insert a mapping between `fn_index` to `(module_name, import_name)`
    pub fn insert(&mut self, module_name: &str, import_name: &str, fn_index: FuncIndex) {
        self.inner
            .insert(fn_index, (module_name.to_string(), import_name.to_string()));
    }

    /// Given a function index `fn_index` - tries to resolve the matching `(module_name, import_name)` if there's one
    pub fn try_resolve(&self, fn_index: FuncIndex) -> Option<(&str, &str)> {
        let import = self.inner.get(&fn_index);

        import.map(|(module_name, import_name)| (module_name.as_str(), import_name.as_str()))
    }

    /// Given a function index `fn_index` - returns the matching `(module_name, import_name)`
    ///
    /// #Panics
    ///
    /// Panics if there is no corresponding `(module_name, import_name)` to the parameter `fn_index`
    pub fn resolve(&self, fn_index: FuncIndex) -> (&str, &str) {
        self.try_resolve(fn_index).unwrap()
    }

    /// Returns the number of imports mapped
    pub fn count(&self) -> usize {
        self.inner.len()
    }
}
