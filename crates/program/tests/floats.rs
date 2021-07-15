use svm_program::{Program, ProgramError};

#[test]
fn validate_floats_not_allowed() {
    let wasm = r#"
          (module
            (func $func0 (result f32)
                (f32.const 0)))
        "#;

    let result = Program::new(wasm.as_bytes());

    assert!(matches!(result, Err(ProgramError::FloatsNotAllowed)));
}
