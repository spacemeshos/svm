use svm_gas::FuncIndex;

fn inspect_wasm(wasm: &str) {
    let wasm = wat::parse_str(wasm).unwrap();

    let program = svm_gas::read_program(&wasm).unwrap();
    let body = program.get_func_body(FuncIndex(0));

    for op in body.instructions() {
        dbg!(op);
    }
}

#[test]
fn wasm_inspect_if() {
    let wasm = r#" 
        (module
            (func $f0
                (if (result i32) 
                    (local.get 0)
                        (then (call $dummy) (call $dummy) (i32.const 8) (call $dummy))
                        (else (call $dummy) (call $dummy) (i32.const 9) (call $dummy)))
            )

            (func $dummy)
        )
    "#;

    inspect_wasm(&wasm);
}

#[test]
fn wasm_inspect_select() {
    let wasm = r#"
        (module
            (func (export "select_trap_l") (param $cond i32) (result i32)
                (select (unreachable) (i32.const 0) (local.get $cond)))
        )
    "#;

    inspect_wasm(&wasm);
}
