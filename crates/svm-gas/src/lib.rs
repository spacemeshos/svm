#![allow(missing_docs)]
#![allow(unused)]

pub(crate) mod block;
pub(crate) mod code_reader;
pub(crate) mod program;

pub mod error;
pub mod function;
pub mod function_gas;
pub mod gas;

#[test]
fn print_instructions() {
    // let wat = r#"
    // (func (export "type-i32-value") (result i32)
    //     (block (result i32) (i32.ctz (br 0 (i32.const 1))))
    // )
    // "#;

    // let wat = r#"
    //     (func $fac (export "fac") (param i64) (result i64)
    //         (if (result i64) (i64.eqz (local.get 0))
    //           (then (i64.const 1))
    //           (else
    //             (i64.mul
    //               (local.get 0)
    //               (call $fac (i64.sub (local.get 0) (i64.const 1)))
    //             )
    //           )
    //         )
    //       )
    // "#;

    // let wat = r#"
    // (func (export "as-br_if-cond")
    //     (br_if 0 (br 0))
    //   )
    // "#;

    let wat = r#"
        (func $dummy)
        (func (export "multi") (result i32)
            (block (call $dummy) (call $dummy) (call $dummy) (call $dummy))
            (block (result i32) (call $dummy) (call $dummy) (call $dummy) (i32.const 8))
        )
    "#;

    let wasm = wabt::wat2wasm(wat).unwrap();

    let program = code_reader::read_program(&wasm);

    dbg!(program.functions);
}
