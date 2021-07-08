use indexmap::IndexMap;
use parity_wasm::elements::Instruction;

use crate::{FuncIndex, Function};

mod visitor;
pub use visitor::ProgramVisitor;

mod import;
pub use import::Imports;

/// Parsed Wasm Program.
#[derive(Debug, Default)]
pub struct Program {
    imports: Imports,
    functions: IndexMap<FuncIndex, Vec<Instruction>>,
}

impl Program {
    /// The functions imports
    pub fn imports(&self) -> &Imports {
        &self.imports
    }

    /// Setting the functions imports
    pub fn set_imports(&mut self, imports: Imports) {
        self.imports = imports;
    }

    /// Adding a function with index` fn_index` and instructions `ops`
    pub fn add_func(&mut self, fn_index: FuncIndex, ops: Vec<Instruction>) {
        self.functions.insert(fn_index, ops);
    }

    /// Returns whether function is an import function or not
    pub fn is_imported(&self, fn_index: FuncIndex) -> bool {
        (fn_index.0 as usize) < self.imports.count()
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
