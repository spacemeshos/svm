use std::collections::HashMap;

use crate::function::{FuncBody, FuncIndex};

#[derive(Debug)]
pub(crate) struct Program {
    pub import_count: u16,
    pub functions: HashMap<FuncIndex, FuncBody>,
}

impl Program {
    pub fn is_imported(&self, func_idx: FuncIndex) -> bool {
        func_idx.0 < self.import_count
    }

    pub fn get_function_body(&self, func_idx: FuncIndex) -> &FuncBody {
        self.functions.get(&func_idx).as_ref().unwrap()
    }

    pub fn functions_ids(&self) -> Vec<FuncIndex> {
        self.functions.keys().copied().collect()
    }
}
