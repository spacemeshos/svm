use crate::function::{FuncBody, FuncIndex};
use crate::program::Program;

use std::collections::HashMap;

use parity_wasm::elements::{ImportCountType, Module};
use wasmparser::{Operator, Parser, ParserState, WasmDecoder};

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

#[allow(unused)]
pub(crate) fn parse_program(wasm: &[u8]) -> HashMap<FuncIndex, Vec<Operator>> {
    let mut parser = Parser::new(wasm);
    let mut function_readers = Vec::new();

    while !(parser.eof()) {
        match parser.read() {
            ParserState::BeginFunctionBody { .. } => {
                while !(parser.eof()) {
                    let reader = parser.create_binary_reader();
                    function_readers.push(reader);
                }
            }
            _ => continue,
        }
    }

    for (i, reader) in function_readers.iter_mut().enumerate() {
        while let Ok(ref op) = reader.read_operator() {
            //
        }
    }

    panic!()

    //     let mut functions = HashMap::new();
    //
    //     let module: Module = parity_wasm::deserialize_buffer(wasm).unwrap();
    //
    //     let code_section = module.code_section().expect("no code section");
    //     let imported_count = module.import_count(ImportCountType::Function) as u32;
    //
    //     for (i, func_body) in code_section.bodies().iter().enumerate() {
    //         let fn_idx = FuncIndex((i as u32) + imported_count);
    //         let fn_body = FuncBody(func_body.code().clone());
    //
    //         functions.insert(fn_idx, fn_body);
    //     }
    //
    //     Program {
    //         functions,
    //         imported_count,
    //     }
}
