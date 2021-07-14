use svm_gas::{FuncPrice, ProgramVisitor};
use wasmer::{ExternType, FunctionType};
use wasmparser::Operator;

pub fn calculate_gas_limit(wasm_module: &[u8], _gas_limit: u64) -> FuncPrice {
    let program = svm_gas::read_program(wasm_module).unwrap();
    let pricing_resolver = svm_gas::resolvers::ExampleResolver::default();
    let program_pricing = svm_gas::ProgramPricing::new(pricing_resolver);

    let prices = program_pricing.visit(&program).unwrap();
    prices
}

/// Checks whether `wasm_module` exports a well-defined `svm_alloc` function.
/// `svm_alloc` is required by SVM for all WASM code and must have a `I32 ->
/// I32` type signature.
pub fn validate_svm_alloc(wasm_module: &[u8]) -> bool {
    let store = crate::new_store();
    let module = if let Ok(m) = wasmer::Module::new(&store, wasm_module) {
        m
    } else {
        return false;
    };
    for export in module.exports() {
        let expected_type_signature =
            ExternType::Function(FunctionType::new([wasmer::Type::I32], [wasmer::Type::I32]));
        if export.name() == "svm_alloc" && export.ty() == &expected_type_signature {
            return true;
        }
    }
    false
}

// Checks whether `wasm_module` only contains the WASM opcodes that are
// supported by SVM.
pub fn validate_opcodes(wasm_module: &[u8]) -> bool {
    use wasmparser::{Parser, Payload};

    let parser = Parser::default();
    let mut parser_events = parser.parse_all(wasm_module);

    parser_events.all(|event_res| match event_res {
        Err(_) => false,
        Ok(event) => {
            // We only validate opcodes in the WASM code section. Other sections
            // don't interest us.
            if let Payload::CodeSectionEntry(function_body) = event {
                let operators = function_body.get_operators_reader().unwrap();
                for op in operators.into_iter() {
                    if !opcode_is_valid(op.unwrap()) {
                        return false;
                    }
                }
            }
            true
        }
    })
}

fn opcode_is_valid(op: Operator) -> bool {
    match op {
        Operator::Unreachable
        | Operator::Nop
        | Operator::Block { .. }
        | Operator::If { .. }
        | Operator::Loop { .. }
        | Operator::Else
        | Operator::End
        | Operator::Br { .. }
        | Operator::BrIf { .. }
        | Operator::BrTable { .. }
        | Operator::Return
        | Operator::Call { .. }
        | Operator::CallIndirect { .. }
        | Operator::Drop
        | Operator::Select
        | Operator::LocalGet { .. }
        | Operator::LocalSet { .. }
        | Operator::LocalTee { .. }
        | Operator::GlobalGet { .. }
        | Operator::GlobalSet { .. }
        | Operator::I32Load { .. }
        | Operator::I64Load { .. }
        | Operator::I32Load8S { .. }
        | Operator::I32Load8U { .. }
        | Operator::I32Load16S { .. }
        | Operator::I32Load16U { .. }
        | Operator::I64Load8S { .. }
        | Operator::I64Load8U { .. }
        | Operator::I64Load16S { .. }
        | Operator::I64Load16U { .. }
        | Operator::I64Load32S { .. }
        | Operator::I64Load32U { .. }
        | Operator::I32Store { .. }
        | Operator::I64Store { .. }
        | Operator::I32Store8 { .. }
        | Operator::I32Store16 { .. }
        | Operator::I64Store8 { .. }
        | Operator::I64Store16 { .. }
        | Operator::I64Store32 { .. }
        | Operator::MemorySize { .. }
        | Operator::MemoryGrow { .. }
        | Operator::I32Const { .. }
        | Operator::I64Const { .. }
        | Operator::I32Eqz
        | Operator::I32Eq
        | Operator::I32Ne
        | Operator::I32LtS
        | Operator::I32LtU
        | Operator::I32GtS
        | Operator::I32GtU
        | Operator::I32LeS
        | Operator::I32LeU
        | Operator::I32GeS
        | Operator::I32GeU
        | Operator::I64Eqz
        | Operator::I64Eq
        | Operator::I64Ne
        | Operator::I64LtS
        | Operator::I64LtU
        | Operator::I64GtS
        | Operator::I64GtU
        | Operator::I64LeS
        | Operator::I64LeU
        | Operator::I64GeS
        | Operator::I64GeU
        | Operator::I32Clz
        | Operator::I32Ctz
        | Operator::I32Popcnt
        | Operator::I32Add
        | Operator::I32Sub
        | Operator::I32Mul
        | Operator::I32DivS
        | Operator::I32DivU
        | Operator::I32RemS
        | Operator::I32RemU
        | Operator::I32And
        | Operator::I32Or
        | Operator::I32Xor
        | Operator::I32Shl
        | Operator::I32ShrS
        | Operator::I32ShrU
        | Operator::I32Rotl
        | Operator::I32Rotr
        | Operator::I64Clz
        | Operator::I64Ctz
        | Operator::I64Popcnt
        | Operator::I64Add
        | Operator::I64Sub
        | Operator::I64Mul
        | Operator::I64DivS
        | Operator::I64DivU
        | Operator::I64RemS
        | Operator::I64RemU
        | Operator::I64And
        | Operator::I64Or
        | Operator::I64Xor
        | Operator::I64Shl
        | Operator::I64ShrS
        | Operator::I64ShrU
        | Operator::I64Rotl
        | Operator::I64Rotr
        | Operator::I32WrapI64
        | Operator::I64ExtendI32S
        | Operator::I32Extend8S
        | Operator::I32Extend16S
        | Operator::I64Extend8S
        | Operator::I64Extend16S
        | Operator::I64Extend32S => true,
        _ => false,
    }
}
