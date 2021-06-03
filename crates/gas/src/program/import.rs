use indexmap::IndexMap;

use crate::FuncIndex;

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
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: IndexMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, module: &str, name: &str, fn_index: FuncIndex) {
        self.inner
            .insert(fn_index, (module.to_string(), name.to_string()));
    }

    pub fn try_resolve(&self, fn_index: FuncIndex) -> Option<(&str, &str)> {
        let import = self.inner.get(&fn_index);

        import.map(|(module_name, import_name)| (module_name.as_str(), import_name.as_str()))
    }

    pub fn resolve(&self, fn_index: FuncIndex) -> (&str, &str) {
        self.try_resolve(fn_index).unwrap()
    }

    pub fn count(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> indexmap::map::Iter<FuncIndex, (String, String)> {
        self.inner.iter()
    }
}
