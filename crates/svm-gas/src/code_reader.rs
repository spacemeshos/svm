use crate::{
    function::{FuncBody, FuncIndex},
    program::Program,
};

use std::collections::HashMap;

use parity_wasm::elements::{ImportCountType, Module};

/// Reads wasm input and contruct a `Program` struct
pub(crate) fn read_program(wasm: &[u8]) -> Program {
    let mut functions = HashMap::new();

    let module: Module = parity_wasm::deserialize_buffer(wasm).unwrap();

    let code_section = module.code_section().expect("no code section");
    let imported_count = module.import_count(ImportCountType::Function);

    assert!(imported_count <= std::u16::MAX as usize);

    let imported_count = imported_count as u16;

    for (i, func_body) in code_section.bodies().iter().enumerate() {
        let fn_idx = FuncIndex((i as u16) + imported_count);
        let fn_body = FuncBody(func_body.code().clone());

        functions.insert(fn_idx, fn_body);
    }

    Program {
        functions,
        imported_count,
    }
}
