use std::collections::HashMap;

use parity_wasm::elements::{CodeSection, External, ImportCountType, ImportEntry, Module};

use crate::{FuncIndex, Function, Imports, Program, ProgramError};

/// Reads a Wasm program and constructs a `Program` struct
pub fn read_program(wasm: &[u8]) -> Result<Program, ProgramError> {
    let module = read_module(wasm)?;
    let code = read_code(&module)?;
    let imports = read_imports(&module)?;
    let mut program = Program::default();

    for (i, func_body) in code.bodies().iter().enumerate() {
        let fn_index = i + imports.len();

        let index = FuncIndex(fn_index as u16);
        let code = func_body.code().elements().to_vec();

        program.add_func(index, code);
    }

    Ok(program)
}

fn read_module(wasm: &[u8]) -> Result<Module, ProgramError> {
    let module = parity_wasm::deserialize_buffer(wasm);

    module.map_err(|_| ProgramError::InvalidWasm)
}

fn read_code(module: &Module) -> Result<CodeSection, ProgramError> {
    match module.code_section() {
        Some(code) => Ok(code.clone()),
        None => Err(ProgramError::MissingCodeSection),
    }
}

fn read_imports<'m>(module: &Module) -> Result<Imports, ProgramError> {
    let import_section = module.import_section();

    if let Some(import_section) = import_section {
        let count = module_import_count(module)?;

        let mut imports = Imports::with_capacity(count as usize);

        import_section.entries().iter().for_each(|import| {
            if let External::Function(func) = import.external() {
                let module = import.module();
                let name = import.field();
                let func = FuncIndex(*func as u16);

                imports.add_import(module, name, func);
            }
        });

        Ok(imports)
    } else {
        Ok(Imports::new())
    }
}

fn module_import_count(module: &Module) -> Result<u16, ProgramError> {
    let import_count = module.import_count(ImportCountType::Function);

    if import_count <= std::u16::MAX as usize {
        Ok(import_count as u16)
    } else {
        Err(ProgramError::TooManyFunctionImports)
    }
}
