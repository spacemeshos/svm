use indexmap::IndexMap;
use parity_wasm::elements::{ExportSection, Internal, Module};

use crate::{FuncIndex, ProgramError};

/// Stores a mapping between an export name to its corresponding function index.
#[derive(Debug, Clone, Default)]
pub struct Exports {
    inner: IndexMap<String, FuncIndex>,
}

impl Exports {
    pub(crate) fn read(module: &Module) -> Result<Exports, ProgramError> {
        let empty_exports_section = ExportSection::with_entries(vec![]);

        let mut exports = Exports::default();
        let items = module
            .export_section()
            .unwrap_or(&empty_exports_section)
            .entries()
            .iter()
            .filter_map(|entry| {
                if let Internal::Function(i) = entry.internal() {
                    Some((entry.field().to_string(), *i))
                } else {
                    None
                }
            });
        for (name, func_index) in items {
            exports.insert(name, FuncIndex(func_index))
        }
        Ok(exports)
    }
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
