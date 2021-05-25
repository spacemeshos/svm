use std::collections::HashMap;

use parity_wasm::elements::Instruction;

use crate::{FuncIndex, Function};

mod visitor;
pub use visitor::ProgramVisitor;

mod import;
pub use import::Imports;

#[derive(Debug)]
pub struct Program {
    imports: Imports,

    functions: HashMap<FuncIndex, Vec<Instruction>>,
}

impl Program {
    pub fn imports(&self) -> &Imports {
        &self.imports
    }

    pub fn set_imports(&mut self, imports: Imports) {
        self.imports = imports;
    }

    pub fn add_func(&mut self, index: FuncIndex, code: Vec<Instruction>) {
        self.functions.insert(index, code);
    }

    pub fn is_imported(&self, func: FuncIndex) -> bool {
        (func.0 as usize) < self.imports.count()
    }

    pub fn get_func(&self, index: FuncIndex) -> Function {
        let code = self.functions.get(&index).unwrap();

        Function::new(index, code)
    }

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
