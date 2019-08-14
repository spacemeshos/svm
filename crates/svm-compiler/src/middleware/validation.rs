use wasmer_runtime_core::{
    codegen::{Event, EventSink, FunctionMiddleware},
    module::ModuleInfo,
    wasmparser::Operator,
};

use super::error::ParseError;

/// The `ValidationMiddleware` has two main objectives:
/// * validation - make sure the wasm is valid and doesn't contain and opcodes not supported by `svm` (for example: floats)
/// * preprocessing - we want to know whether the input contains loops or not.
///   In case there no loop we can later compute ahead-of-time the gas for each function,
///   otherwise we'll have a dynamic get metering used
pub struct ValidationMiddleware;

impl ValidationMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

impl FunctionMiddleware for ValidationMiddleware {
    type Error = ParseError;

    fn feed_event<'a, 'b: 'a>(
        &mut self,
        event: Event<'a, 'b>,
        _module_info: &ModuleInfo,
        sink: &mut EventSink<'a, 'b>,
    ) -> Result<(), Self::Error> {
        match event {
            Event::Wasm(op) => parse_wasm_opcode(op)?,
            Event::WasmOwned(ref op) => parse_wasm_opcode(op)?,
            _ => (),
        };

        sink.push(event);
        Ok(())
    }
}

/// we explicitly whitelist the supported opcodes
fn parse_wasm_opcode(opcode: &Operator) -> Result<(), ParseError> {
    match opcode {
        Operator::Unreachable
        | Operator::Nop
        | Operator::Block { .. }
        | Operator::Loop { .. }
        | Operator::If { .. }
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
        | Operator::GetLocal { .. }
        | Operator::SetLocal { .. }
        | Operator::TeeLocal { .. }
        | Operator::GetGlobal { .. }
        | Operator::SetGlobal { .. }
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
        | Operator::I64ExtendSI32
        | Operator::I64ExtendUI32
        | Operator::I32Extend8S
        | Operator::I32Extend16S
        | Operator::I64Extend8S
        | Operator::I64Extend16S
        | Operator::I64Extend32S => Ok(()),
        _ => Err(ParseError::UnsupportedOpcode),
    }
}

#[cfg(test)]
mod tests {
    use crate::compile_program;
    use wasmer_runtime::error::CompileError;
    use wasmer_runtime::{imports, Func};

    #[test]
    fn valid_wasm_instance_sanity() {
        let input = r#"
            (module
                (func (export "sum") (param i32 i32) (result i32)
                    get_local 0
                    get_local 1
                    i32.add
                ))
            "#;
        let wasm = wabt::wat2wasm(input).unwrap();
        let module = compile_program(&wasm).unwrap();
        let instance = module.instantiate(&imports! {}).unwrap();

        let func: Func<(i32, i32), i32> = instance.func("sum").unwrap();
        let res = func.call(10, 20);
        assert!(res.is_ok());
        assert_eq!(30, res.unwrap());
    }

    #[test]
    fn parser_floats_are_not_supported() {
        let input = r#"
            (module
                (func $to_float (param i32) (result f32)
                    get_local 0
                    f32.convert_u/i32
                ))
            "#;

        let wasm = wabt::wat2wasm(input).unwrap();
        let res = compile_program(&wasm);

        assert!(res.is_err());

        if let Err(CompileError::InternalError { msg }) = res {
            assert_eq!("Codegen(\"UnsupportedOpcode\")", msg.as_str());
        } else {
            unreachable!()
        }
    }
}
