use std::collections::HashMap;

use parity_wasm::elements::{
    CodeSection, ExportSection, External, ImportCountType, ImportEntry, Module,
};

use crate::{Exports, FuncIndex, Function, Imports, NodeLabel, Program};

type ProgramError = crate::ProgramError<FuncIndex>;

/// Reads a Wasm program and constructs a `Program` struct
pub fn read_program(wasm: &[u8]) -> Result<Program, ProgramError> {
    let module = read_module(wasm)?;

    let code = read_code(&module)?;
    let imports = read_imports(&module)?;
    let exports = read_exports(&module)?;

    let mut program = Program::default();

    for (i, fn_body) in code.bodies().iter().enumerate() {
        let fn_index = i + imports.count();

        let fn_index = FuncIndex(fn_index as u32);
        let fn_code = fn_body.code().elements().to_vec();

        program.add_func(fn_index, fn_code);
    }

    program.set_imports(imports);
    program.set_exports(exports);

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
        let import_count = module_import_count(module)?;

        let mut imports = Imports::with_capacity(import_count as usize);
        let mut offset = 0;

        import_section.entries().iter().for_each(|import| {
            if let External::Function(..) = import.external() {
                let module = import.module();
                let name = import.field();
                let fn_index = FuncIndex(offset);

                imports.insert(module, name, fn_index);

                offset += 1;
            }
        });

        Ok(imports)
    } else {
        Ok(Imports::new())
    }
}

fn read_exports(module: &Module) -> Result<Exports, ProgramError> {
    let empty_exports_section = ExportSection::with_entries(vec![]);

    let mut exports = Exports::default();
    let items = module
        .export_section()
        .unwrap_or(&empty_exports_section)
        .entries()
        .iter()
        .filter_map(|entry| {
            if let parity_wasm::elements::Internal::Function(i) = entry.internal() {
                Some((entry.field().to_string(), *i))
            } else {
                None
            }
        });
    for (name, func_index) in items {
        exports.insert(name, FuncIndex(func_index))
    }
    Ok(exports)
}

fn module_import_count(module: &Module) -> Result<u16, ProgramError> {
    let import_count = module.import_count(ImportCountType::Function);

    if import_count <= std::u16::MAX as usize {
        Ok(import_count as u16)
    } else {
        Err(ProgramError::TooManyFunctionImports)
    }
}
