use svm_gas::{error::ProgramError, FuncIndex};

macro_rules! validate_code {
    ($code:expr) => {{
        let wasm = wabt::wat2wasm($code).unwrap();

        svm_gas::validate_code(&wasm[..])
    }};
}

#[test]
fn validate_loops_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (loop (nop))))
        "#;

    let res = validate_code!(code);
    assert_eq!(Err(ProgramError::LoopNotAllowed), res);
}

#[test]
fn validate_direct_recursive_call_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (call $func0)))
        "#;

    let res = validate_code!(code);
    assert_eq!(
        Err(ProgramError::RecursiveCall(vec![
            FuncIndex(0),
            FuncIndex(0)
        ])),
        res
    );
}

#[test]
fn validate_indirect_recursive_call_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (call $func1))

            (func $func1
                (call $func2))

            (func $func2
                (call $func0)))
        "#;

    let res = validate_code!(code);
    assert_eq!(
        Err(ProgramError::RecursiveCall(vec![
            FuncIndex(0),
            FuncIndex(1),
            FuncIndex(2),
            FuncIndex(0),
        ])),
        res
    );
}

#[test]
fn validate_call_indirect_not_allowed() {
    let code = r#"
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

    let res = validate_code!(code);
    assert_eq!(Err(ProgramError::CallIndirectNotAllowed), res);
}

#[test]
fn validate_br_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (br 0))

            (func $func1
                (block (br 0))))
        "#;

    let res = validate_code!(code);
    assert_eq!(Err(ProgramError::BrNotAllowed), res);
}

#[test]
fn validate_br_if_not_allowed() {
    let code = r#"
          (module
            (func $func0 (result i32)
                (block (result i32) (br_if 0 (i32.const 0) (i32.const 0)))))
        "#;

    let res = validate_code!(code);
    assert_eq!(Err(ProgramError::BrIfNotAllowed), res);
}

#[test]
fn validate_floats_not_allowed() {
    let code = r#"
          (module
            (func $func0 (result f32)
                 (f32.const 0)))
        "#;

    let res = validate_code!(code);
    assert_eq!(Err(ProgramError::FloatsNotAllowed), res);
}
