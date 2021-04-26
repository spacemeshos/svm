use std::collections::HashMap;

use crate::{FuncBody, FuncIndex, Imports};

#[derive(Debug)]
pub(crate) struct Program {
    pub imports: Imports,

    pub functions: HashMap<FuncIndex, FuncBody>,
}

impl Program {
    pub fn is_imported(&self, func: FuncIndex) -> bool {
        (func.0 as usize) < self.imports.len()
    }

    pub fn get_func_body(&self, func: FuncIndex) -> &FuncBody {
        self.functions.get(&func).as_ref().unwrap()
    }

    pub fn functions(&self) -> Vec<FuncIndex> {
        self.functions.keys().copied().collect()
    }
}
