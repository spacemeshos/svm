use svm_gas::{FuncIndex, ProgramError};

macro_rules! validate_wasm {
    ($code:expr) => {{
        let wasm = wat::parse_str($code).unwrap();

        svm_gas::validate_wasm(&wasm)
    }};
}

#[test]
fn validate_floats_not_allowed() {
    let wasm = r#"
          (module
            (func $func0 (result f32)
                 (f32.const 0)))
        "#;

    let result = validate_wasm!(wasm);
    assert_eq!(Err(ProgramError::FloatsNotAllowed), result);
}

#[test]
fn validate_loops_not_allowed() {
    let wasm = r#"
          (module
            (func $func0
                (loop (nop))))
        "#;

    let result = validate_wasm!(wasm);
    assert_eq!(Err(ProgramError::LoopNotAllowed), result);
}

#[test]
fn validate_direct_recursive_call_not_allowed() {
    let wasm = r#"
          (module
            (func $func0
                (nop)
                (nop)
                (call $func0)))
        "#;

    let result = validate_wasm!(wasm);

    assert_eq!(
        Err(ProgramError::RecursiveCall {
            func: FuncIndex(0),
            offset: 2,
        }),
        result
    );
}

#[test]
fn validate_indirect_recursive_call_not_allowed() {
    let wasm = r#"
          (module
            (func $func0
                (call $func1))

            (func $func1
                (call $func2))

            (func $func2
                (call $func0)))
        "#;

    let result = validate_wasm!(wasm);

    assert_eq!(
        Err(ProgramError::CallCycle(vec![
            FuncIndex(0),
            FuncIndex(1),
            FuncIndex(2),
            FuncIndex(0),
        ])),
        result
    );
}

#[test]
fn validate_call_indirect_not_allowed() {
    let wasm = r#"
          (module
            (type $proc (func))

            (table funcref
                (elem
                    $func0))

            (func $func0 (type $proc)
                (nop))

            (func $func1
                (call_indirect (type $proc) (i32.const 0))))
        "#;

    let result = validate_wasm!(wasm);
    assert_eq!(Err(ProgramError::CallIndirectNotAllowed), result);
}
