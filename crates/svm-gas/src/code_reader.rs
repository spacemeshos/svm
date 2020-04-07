use crate::{
    error::ProgramError,
    function::{FuncBody, FuncIndex},
    program::Program,
};

use std::collections::HashMap;

use parity_wasm::elements::{ImportCountType, Module};

/// Reads wasm input and contruct a `Program` struct
pub(crate) fn read_program(wasm: &[u8]) -> Result<Program, ProgramError> {
    let mut functions = HashMap::new();

    let module = read_wasm(wasm)?;
    let code_section = module.code_section().expect("no code section");
    let import_count = module_import_count(&module)?;

    for (i, func_body) in code_section.bodies().iter().enumerate() {
        let fn_idx = FuncIndex((i as u16) + import_count);
        let fn_body = FuncBody(func_body.code().clone());

        functions.insert(fn_idx, fn_body);
    }

    let program = Program {
        functions,
        import_count,
    };

    Ok(program)
}

fn module_import_count(module: &Module) -> Result<u16, ProgramError> {
    let import_count = module.import_count(ImportCountType::Function);

    if import_count <= std::u16::MAX as usize {
        Ok(import_count as u16)
    } else {
        Err(ProgramError::TooManyFunctionImports)
    }
}

#[inline]
fn read_wasm(wasm: &[u8]) -> Result<Module, ProgramError> {
    parity_wasm::deserialize_buffer(wasm).map_err(|_| ProgramError::InvalidWasm)
}
