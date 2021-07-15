#![allow(unused)]

use svm_gas::build_func_cfg;
use svm_gas::{BlockNum, CFG};
use svm_program::{Op, Program};

mod helpers;

macro_rules! test_cfg {
    ($cfg_path:expr, $wasm:expr) => {{
        let expected = include!($cfg_path);

        let program = Program::from_wat($wasm).unwrap();
        let func = helpers::get_func(&program, 0);

        let actual = build_func_cfg(&func);

        helpers::assert_cfg_eq(&actual, &expected);
    }};
}

#[test]
fn cfg_empty() {
    // $f0 Wasm:
    // ---------
    // 0: End

    let wasm = r#" 
        (module
            (func $f0))
    "#;

    test_cfg!("graphs/empty.rs", wasm);
}

#[test]
fn cfg_implicit_block() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(0)
    // 1: GetLocal(1)
    // 2: End

    let wasm = r#"
        (module
            (func $f0
                (local.get 0)
                (local.get 1)
            )
        )
    "#;

    test_cfg!("graphs/implicit_block.rs", wasm);
}

#[test]
fn cfg_explicit_block() {
    // $f0 Wasm:
    // ---------
    // 0: Block(NoResult)
    // 1: End
    // 2: End

    let wasm = r#"
        (module
            (func $f0
                (block)))
    "#;

    test_cfg!("graphs/explicit_block.rs", wasm);
}

#[test]
fn cfg_nested_empty_blocks() {
    // $f0 Wasm:
    // ---------
    // 0: Block(NoResult)
    // 1: Block(NoResult)
    // 2: Block(NoResult)
    // 3: Block(NoResult)
    // 4: End
    // 5: End
    // 6: End
    // 7: End
    // 8: End

    let wasm = r#"
        (module
            (func $f0
                (block
                    (block
                        (block
                            (block)
                        )
                    )
                )
            )
        )
    "#;

    test_cfg!("graphs/nested_empty_blocks.rs", wasm);
}

#[test]
fn cfg_return() {
    // $f0 Wasm:
    // ---------
    // 0: Return
    // 1: End

    let wasm = r#"
        (module
            (func $f0
                return
            )
        )
    "#;

    test_cfg!("graphs/return.rs", wasm);
}

#[test]
fn cfg_unreachable() {
    // $f0 Wasm:
    // ---------
    // 0: Unreachable
    // 1: End

    let wasm = r#"
        (module
            (func $f0
                unreachable
            )
        )
    "#;

    test_cfg!("graphs/unreachable.rs", wasm);
}

#[test]
fn cfg_br() {
    // $f0 Wasm:
    // ---------
    // 0: Block(Value(I32))
    // 1: Nop
    // 2: I32Const(10)
    // 3: Br(0)
    // 4: I32Const(20)
    // 5: End
    // 6: I32Const(30)
    // 7: End

    let wasm = r#"
        (module
            (func $f0
                (block (result i32) 
                    (nop)
                    (i32.const 10)
                    (br 0)
                    (i32.const 20)
                )
                (i32.const 30)
            )               
        )
    "#;

    test_cfg!("graphs/br.rs", wasm);
}

#[test]
fn cfg_br_if() {
    // $f0 Wasm:
    // ---------
    // 0: Block(Value(I32))
    // 1: GetLocal(0)
    // 2: BrIf(0)
    // 3: I32Const(1)
    // 4: End
    // 5: End

    let wasm = r#"
        (module
            (func $f0
                (block (result i32) 
                    (local.get 0)
                    (br_if 0) 
                    (i32.const 1)
                )
            )               
        )
    "#;

    test_cfg!("graphs/br_if.rs", wasm);
}

#[test]
fn cfg_br_table() {
    // $f0 Wasm:
    // ---------
    // 0: Block(NoResult)
    // 1: Block(NoResult)
    // 2: GetLocal(0)
    // 3: BrTable(BrTableData { table: [1], default: 0 })
    // 4: I32Const(21)
    // 5: Return
    // 6: I32Const(30)
    // 7: End
    // 8: I32Const(20)
    // 9: Return
    // 10: I32Const(40)
    // 11: End
    // 12: I32Const(22)
    // 13: End

    let wasm = r#"
        (module
            (func $f0 (param i32) (result i32)
                (block
                    (block
                        (local.get 0)
                        (br_table 1 0)
                        (return (i32.const 21))
                        (i32.const 30)
                    )
                    (return (i32.const 20))
                    (i32.const 40)
                )
                (i32.const 22)
            )               
        )
    "#;

    test_cfg!("graphs/br_table.rs", wasm);
}

#[test]
fn cfg_if_then() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(0)
    // 1: If(NoResult)
    // 2: Nop
    // 3: End
    // 4: End

    let wasm = r#"
        (module
            (func $f0
                (if (local.get 0)
                (then (nop)))
            )
        )
    "#;

    test_cfg!("graphs/if_then.rs", wasm);
}

#[test]
fn cfg_if_then_else() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(0)
    // 1: If(Value(I32))
    // 2: I32Const(7)
    // 3: Else
    // 4: I32Const(8)
    // 5: End
    // 6: End

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then (i32.const 7)) 
                (else (i32.const 8)))
            )
        )
    "#;

    test_cfg!("graphs/if_then_else.rs", wasm);
}

#[test]
fn cfg_if_then_nested() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(1)
    // 1: If(Value(I32))
    // 2: I32Const(2)
    // 3: GetLocal(3)
    // 4: If(NoResult)
    // 5: I32Const(4)
    // 6: End
    // 7: End
    // 8: End

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 1) 
                    (then 
                        (i32.const 2) 
                        (if (local.get 3) 
                        (then 
                            (i32.const 4)
                        ))
                    )
                )
            )
        )
    "#;

    test_cfg!("graphs/if_then_nested.rs", wasm);
}

#[test]
fn cfg_if_then_nested_2() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(0)
    // 1: If(Value(I32))
    // 2: GetLocal(1)
    // 3: If(Value(I32))
    // 4: I32Const(8)
    // 5: Else
    // 6: I32Const(9)
    // 7: End
    // 8: Else
    // 9: GetLocal(2)
    // 10: If(Value(I32))
    // 11: I32Const(10)
    // 12: Else
    // 13: I32Const(11)
    // 14: End
    // 15: End
    // 16: End

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then
                    (if (result i32) (local.get 1)
                    (then 
                        (i32.const 8)
                    )
                    (else
                        (i32.const 9)
                    ))
                )
                (else
                    (if (result i32) (local.get 2)
                    (then 
                        (i32.const 10)
                    )
                    (else
                        (i32.const 11))
                    ))
                )
            )
        )
    "#;

    test_cfg!("graphs/if_then_nested_2.rs", wasm);
}

#[test]
fn cfg_if_then_nested_3() {
    // $f0 Wasm:
    // ---------
    // 0: GetLocal(0)
    // 1: If(Value(I32))
    // 2: GetLocal(1)
    // 3: If(Value(I32))
    // 4: GetLocal(2)
    // 5: If(Value(I32))
    // 6: I32Const(8)
    // 7: Else
    // 8: I32Const(9)
    // 9: End
    // 10: Else
    // 11: GetLocal(3)
    // 12: If(Value(I32))
    // 13: I32Const(10)
    // 14: Else
    // 15: I32Const(11)
    // 16: End
    // 17: End
    // 18: End
    // 19: End

    let wasm = r#"
        (module
            (func $f0
                (if (result i32) (local.get 0)
                (then
                    (if (result i32) (local.get 1)
                    (then
                        (if (result i32) (local.get 2)
                        (then
                            (i32.const 8)
                        )
                        (else
                            (i32.const 9)
                        ))
                    )
                    (else
                        (if (result i32) (local.get 3)
                        (then
                            (i32.const 10)
                        )
                        (else
                            (i32.const 11))
                        )
                    ))
                ))
            )
        )
    "#;

    test_cfg!("graphs/if_then_nested_3.rs", wasm);
}
