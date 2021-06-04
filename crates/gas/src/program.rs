use std::collections::HashMap;

use parity_wasm::elements::Instruction;

use crate::{FuncIndex, Function, Imports};

/// A `Program` aggregates:
///
/// * Information about the import functions
/// * The code of each non-import functions indexed by their id
#[derive(Debug)]
pub struct Program {
    imports: Imports,

    functions: HashMap<FuncIndex, Vec<Instruction>>,
}

impl Program {
    /// Borrows the `Imports`
    pub fn imports(&self) -> &Imports {
        &self.imports
    }

    /// Adds the function `fn_index` and its `code`
    pub fn add_func(&mut self, index: FuncIndex, code: Vec<Instruction>) {
        self.functions.insert(index, code);
    }

    /// Returns whether input function is an import function or not
    pub fn is_imported(&self, func: FuncIndex) -> bool {
        (func.0 as usize) < self.imports.len()
    }

    /// Returns the data of the requested function
    pub fn get_func(&self, index: FuncIndex) -> Function {
        let code = self.functions.get(&index).unwrap();

        Function::new(index, code)
    }

    /// Returns the functions indexes for each non-import function
    pub fn func_indexes(&self) -> Vec<FuncIndex> {
        self.functions.keys().copied().collect()
    }
}

impl Default for Program {
    fn default() -> Self {
        Program {
            imports: Imports::default(),
            functions: HashMap::new(),
        }
    }
}
