use crate::function::{FuncBody, FuncIndex};
use crate::program::Program;

use std::collections::HashMap;

use parity_wasm::elements::{ImportCountType, Module};

#[allow(unused)]
pub(crate) fn read_program(wasm: &[u8]) -> Program {
    let mut functions = HashMap::new();

    let module: Module = parity_wasm::deserialize_buffer(wasm).unwrap();

    let code_section = module.code_section().expect("no code section");
    let imported_count = module.import_count(ImportCountType::Function) as u32;

    for (i, func_body) in code_section.bodies().iter().enumerate() {
        let fn_idx = FuncIndex((i as u32) + imported_count);
        let fn_body = FuncBody(func_body.code().clone());

        functions.insert(fn_idx, fn_body);
    }

    Program {
        functions,
        imported_count,
    }
}
