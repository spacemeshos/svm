use indexmap::IndexMap;
use parity_wasm::elements::Instruction;

use crate::{FuncIndex, Function};

mod exports;
mod import;
mod visitor;

pub use import::Imports;
pub use visitor::ProgramVisitor;

pub use self::exports::Exports;

/// Parsed Wasm Program.
#[derive(Debug, Default)]
pub struct Program {
    imports: Imports,
    exports: Exports,
    functions: IndexMap<FuncIndex, Vec<Instruction>>,
}

impl Program {
    /// The functions imports
    pub fn imports(&self) -> &Imports {
        &self.imports
    }

    pub fn exports(&self) -> &Exports {
        &self.exports
    }

    /// Setting the functions imports
    pub fn set_imports(&mut self, imports: Imports) {
        self.imports = imports;
    }

    pub fn set_exports(&mut self, exports: Exports) {
        self.exports = exports;
    }

    /// Returns whether function is an import function or not
    pub fn is_imported(&self, fn_index: FuncIndex) -> bool {
        (fn_index.0 as usize) < self.imports.count()
    }

    pub fn is_exported(&self, func_name: &str) -> bool {
        self.exports.contains(func_name)
    }

    /// Adding a function with index` fn_index` and instructions `ops`
    pub fn add_func(&mut self, fn_index: FuncIndex, ops: Vec<Instruction>) {
        self.functions.insert(fn_index, ops);
    }

    /// Returns a `Function` with index `fn_index`
    pub fn get_func(&self, fn_index: FuncIndex) -> Function {
        let code = self.functions.get(&fn_index).unwrap();
        Function::new(fn_index, code)
    }

    /// Returns the indexes of the non-import functions
    pub fn func_indexes(&self) -> Vec<FuncIndex> {
        self.functions.keys().copied().collect()
    }
}
