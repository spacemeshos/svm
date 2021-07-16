use svm_program::{Program, ProgramError};

#[test]
fn validate_floats_not_allowed() {
    let wat = r#"
          (module
            (func $func0 (result f32)
                (f32.const 0)))
        "#;

    let result = Program::from_wat(wat, false);

    println!("{:?}", result);
    assert!(matches!(result, Err(ProgramError::FloatsNotAllowed)));
}
