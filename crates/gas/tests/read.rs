mod helpers;

use svm_gas::FuncIndex;

use parity_wasm::elements::Instruction;

#[test]
fn read_program_multiple_imports() {
    let wasm = r#"
          (module
            (type (;0;) (func (param i32) (result i32)))
            (type (;1;) (func (result i32)))
            (type (;2;) (func (param i32 i32)))

            (import "env"  "foo" (func (;0;) (type 0)))
            (import "env"  "bar" (func (;1;) (type 1)))
            (import "env"  "baz" (func (;2;) (type 1)))
            (import "host" "foo" (func (;3;) (type 2)))
            (import "host" "bar" (func (;4;) (type 2)))
            (import "host" "baz" (func (;5;) (type 2)))

            (func (;6;) (type 2) (param i32 i32)
                (nop))

            (func (;7;) (type 0) (param i32) (result i32)
                (i32.const 10))
          )
        "#;

    let program = helpers::parse_wasm(wasm);
    let imports = program.imports();

    assert_eq!(imports.count(), 6);
    assert_eq!(imports.resolve(FuncIndex(0)), ("env", "foo"));
    assert_eq!(imports.resolve(FuncIndex(1)), ("env", "bar"));
    assert_eq!(imports.resolve(FuncIndex(2)), ("env", "baz"));
    assert_eq!(imports.resolve(FuncIndex(3)), ("host", "foo"));
    assert_eq!(imports.resolve(FuncIndex(4)), ("host", "bar"));
    assert_eq!(imports.resolve(FuncIndex(5)), ("host", "baz"));

    let func6 = program.get_func(FuncIndex(6));
    assert_eq!(&func6.code()[..], &[Instruction::Nop, Instruction::End]);

    let func7 = program.get_func(FuncIndex(7));
    assert_eq!(
        &func7.code()[..],
        &[Instruction::I32Const(10), Instruction::End]
    );
}
