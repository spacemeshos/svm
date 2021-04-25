use std::collections::HashMap;

use parity_wasm::elements::{ImportCountType, Module};

use crate::{FuncBody, FuncIndex, Program, ProgramError};

/// Reads a Wasm program and constructs a `Program` struct
pub(crate) fn read_program(wasm: &[u8]) -> Result<Program, ProgramError> {
    let mut functions = HashMap::new();

    let module = read_wasm(wasm)?;
    let code_section = module.code_section();

    if code_section.is_none() {
        return Err(ProgramError::MissingCodeSection);
    }

    let code_section = code_section.unwrap();
    let import_count = module_import_count(&module)?;

    for (i, func_body) in code_section.bodies().iter().enumerate() {
        // dbg!(format!(
        //     "Reading function with relative-index = {} (absolute-index = {})",
        //     i,
        //     i as u16 + import_count
        // ));

        // if i == 3 {
        //     let ops = func_body.code().elements();

        //     dbg!(&ops[0..10]);
        // }

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

#[inline]
fn read_wasm(wasm: &[u8]) -> Result<Module, ProgramError> {
    parity_wasm::deserialize_buffer(wasm).map_err(|_| ProgramError::InvalidWasm)
}

fn module_import_count(module: &Module) -> Result<u16, ProgramError> {
    let import_count = module.import_count(ImportCountType::Function);

    if import_count <= std::u16::MAX as usize {
        Ok(import_count as u16)
    } else {
        Err(ProgramError::TooManyFunctionImports)
    }
}
