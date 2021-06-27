use svm_layout::SymbolicVar;

use crate::{DeployerAddr, Section, SectionKind};

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaSection {
    vars: Vec<SymbolicVar>,
}

impl SchemaSection {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            vars: Vec::with_capacity(cap),
        }
    }

    pub fn var_count(&self) -> usize {
        self.vars.len()
    }

    pub fn push_var(&mut self, var: SymbolicVar) {
        self.vars.push(var)
    }

    pub fn vars(&self) -> &[SymbolicVar] {
        &self.vars
    }
}

impl Section for SchemaSection {
    const KIND: SectionKind = SectionKind::Schema;
}
