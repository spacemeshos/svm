use crate::function::{FuncBody, FuncIndex};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Program {
    pub imported_count: u32,
    pub functions: HashMap<FuncIndex, FuncBody>,
}

impl Program {
    pub fn is_imported(&self, func_idx: FuncIndex) -> bool {
        func_idx.0 < self.imported_count
    }

    pub fn get_function_body(&self, func_idx: FuncIndex) -> &FuncBody {
        self.functions.get(&func_idx).as_ref().unwrap()
    }

    pub fn functions_ids(&self) -> Vec<FuncIndex> {
        self.functions.keys().map(|k| *k).collect()
    }
}
