use svm_layout::SymbolicVar;

use crate::{SectionKind, SectionLike};

/// Stores high-level definitions of a `Template` persistent storage
#[derive(Debug, Clone, PartialEq)]
pub struct SchemaSection {
    vars: Vec<SymbolicVar>,
}

impl SchemaSection {
    /// Creates a new Section
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Reserves room for `capacity` number of `SymbolicVar`
    ///
    /// See: `push_var`
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vars: Vec::with_capacity(capacity),
        }
    }

    /// Adds a new `SymbolicVar
    pub fn push_var(&mut self, var: SymbolicVar) {
        self.vars.push(var)
    }

    /// Borrows the `SymbolicVar`s of the Schema
    pub fn vars(&self) -> &[SymbolicVar] {
        &self.vars
    }

    /// Returns the number of `SymbolicVar` within the `Schema`
    pub fn var_count(&self) -> usize {
        self.vars.len()
    }
}

impl SectionLike for SchemaSection {
    const KIND: SectionKind = SectionKind::Schema;
}
