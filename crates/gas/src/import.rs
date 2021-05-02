use std::collections::HashMap;
use std::hash::Hash;

use crate::{FuncIndex, Gas};

#[derive(Debug)]
pub struct Imports<T = FuncIndex> {
    entries: HashMap<T, (String, String)>,
}

impl Default for Imports {
    fn default() -> Self {
        Imports::new()
    }
}

impl<T> Imports<T>
where
    T: PartialEq + Eq + Copy + Clone + Hash + 'static,
{
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
        }
    }

    pub fn add_import(&mut self, module: &str, name: &str, func: T) {
        self.entries
            .insert(func, (module.to_string(), name.to_string()));
    }

    pub fn get_import(&self, func: T) -> (&str, &str) {
        let (module, name) = self.entries.get(&func).unwrap();

        (module, name)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
