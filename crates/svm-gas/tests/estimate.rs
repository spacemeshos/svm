extern crate svm_gas;

use maplit::hashmap;
use svm_gas::{error::Error, FuncIndex, Gas};

use svm_gas::traits::VMCallsGasEstimator;

struct PanicVMMCallstimator;

impl VMCallsGasEstimator for PanicVMMCallstimator {
    fn estimate_gas(_func_idx: FuncIndex) -> Gas {
        panic!()
    }
}

macro_rules! estimate_gas {
    ($code:expr) => {{
        use svm_gas::code_reader::read_program;

        let wasm = wabt::wat2wasm($code).unwrap();
        let program = read_program(&wasm);

        svm_gas::estimate_program::<PanicVMMCallstimator>(&program)
    }};
}

#[test]
fn estimate_nop_functions() {
    let code = r#"
          (module
            (func $func0
            	(nop))

            (func $func1
                (block (nop)))

            (func $func2
                (block (block (nop))))

            (func $func3
                (block (block (block (nop))))))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Fixed(0),
            FuncIndex(1) => Gas::Fixed(0),
            FuncIndex(2) => Gas::Fixed(0),
            FuncIndex(3) => Gas::Fixed(0)
        },
        res.unwrap()
    );
}

#[test]
fn estimate_program_with_functions_imports() {
    let code = r#"
          (module
	     (import "env" "func0" (func $env_func0 (param i32)))
	     (import "env" "func1" (func $env_func1 (param i32)))
	     (import "env" "func2" (func $env_func2 (param i32)))

             (func $func3
                (i32.const 0)
                (i32.const 1)
                (i32.const 2)
                (i32.const 3)
                (i32.add)
                (i32.add)
                (i32.add)
                (drop))

            (func $func4
                (i32.const 4)
                (drop)
                (call $func3)))
        "#;

    let res = estimate_gas!(code);

    assert_eq!(
        hashmap! {
            FuncIndex(3) => Gas::Fixed(8),
            FuncIndex(4) => Gas::Fixed(10),
        },
        res.unwrap()
    );
}

#[test]
fn estimate_constant_function() {
    let code = r#"
          (module
            (func $func0 (result i32)
                i32.const 10
                drop
                i32.const 20))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(hashmap! {FuncIndex(0) => Gas::Fixed(3)}, res.unwrap());
}

#[test]
fn estimate_loop_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (loop (nop))))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(Err(Error::LoopNotAllowed), res);
}

#[test]
fn estimate_direct_recursive_call_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (call $func0)))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        Err(Error::RecursiveCall(vec![FuncIndex(0), FuncIndex(0)])),
        res
    );
}

#[test]
fn estimate_indirect_recursive_call_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (call $func1))

            (func $func1
                (call $func2))

            (func $func2
                (call $func0)))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        Err(Error::RecursiveCall(vec![
            FuncIndex(0),
            FuncIndex(1),
            FuncIndex(2),
            FuncIndex(0),
        ])),
        res
    );
}

#[test]
fn estimate_call_indirect_not_allowed() {
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

    let res = estimate_gas!(code);
    assert_eq!(Err(Error::CallIndirectNotAllowed), res);
}

#[test]
fn estimate_br_not_allowed() {
    let code = r#"
          (module
            (func $func0
                (br 0))

            (func $func1
                (block (br 0))))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(Err(Error::BrNotAllowed), res);
}

#[test]
fn estimate_br_if_not_allowed() {
    let code = r#"
          (module
            (func $func0 (result i32)
                (block (result i32) (br_if 0 (i32.const 0) (i32.const 0)))))
        "#;

    let res = estimate_gas!(code);
    assert_eq!(Err(Error::BrIfNotAllowed), res);
}

#[test]
fn estimate_if_stmt_without_else() {
    let code = r#"
          (module
            (func $func0
                (i32.const 0)
                (drop)

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)
                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)
                        (i32.const 3)
                        (i32.add)
                        (drop)
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * true-block: range(0, 4)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * fixed(1) * range(0, 4) = fixed(3) * range(0, 4) = range(3, 7)
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 3, max: 7 }
        },
        res.unwrap()
    );
}

#[test]
fn estimate_if_stmt_without_else_nested() {
    let code = r#"
          (module
            (func $func0
                (i32.const 0)
                (drop)

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)
                    ;; if-condition costs fixed(1) gas
                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)
                        (i32.const 3)
                        (i32.add)
                        (drop)

                        (if (i32.const 4)
                            ;; if-condition costs fixed(1) gas
                            (then
                                ;; block gas = fixed(6)
                                (i32.const 5)
                                (i32.const 6)
                                (i32.add)
                                (i32.const 7)
                                (i32.add)
                                (drop)))
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-statement:
                ;;      * preamble: fixed(4)
                ;;      * inner if-statement:
                ;;          * if-condition: fixed(1)
                ;;          * true-block: fixed(6)
                ;;      inner-if statement total: fixed(1) * range(0, 6) = range(1, 7)
                ;;
                ;;   if-statement total: fixed(1) + (fixed(4) * range(1, 7)) = fixed(1) + range(5, 11) = range(1, 12)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * range(1, 12) = range(3, 14)
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 3, max: 14 }
        },
        res.unwrap()
    );
}

#[test]
fn estimate_if_stmt_with_else_not_nested() {
    let code = r#"
          (module
            (func $func0
                (i32.const 0)
                (drop)

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)
                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)
                        (i32.const 3)
                        (i32.add)
                        (drop)
                    )
                    (else
                        ;; block gas cost = fixed(2)
                        (i32.const 0)
                        (drop)
                    ))))

                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-stmt true-block: fixed(4)
                ;; * if-stmt else-block: fixed(2)
                ;;
                ;; if-stmt total gas: range(2, 4)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * fixed(1) * range(2, 4) = fixed(3) * range(2, 4) = range(5, 7)
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 5, max: 7 }
        },
        res.unwrap()
    );
}

#[test]
fn estimate_if_stmt_with_else_nested() {
    let code = r#"
          (module
            (func $func0
                (i32.const 0)                                       ;; 0
                (drop)                                              ;; 1

                ;; here we have gas cost = fixed(2)

                (if (i32.const 1)                                   ;; 2 + 3
                    ;; if-condition costs fixed(1) gas

                    (then
                        ;; block gas cost = fixed(4)
                        (i32.const 2)                               ;; 4
                        (i32.const 3)                               ;; 5
                        (i32.add)                                   ;; 6
                        (drop)                                      ;; 7
                    )
                    (else                                           ;; 8
                        (i32.const 4)                               ;; 9
                        (drop)                                      ;; 10

                        (if (i32.const 5)                           ;; 11 + 12
                            ;; if-condition costs fixed(1) gas

                            (then
                                ;; block gas cost = fixed(2)
                                (i32.const 6)                       ;; 13
                                (drop)                              ;; 14
                            )
                            (else                                   ;; 15
                                ;; block gas cost = fixed(6)
                                (i32.const 7)                       ;; 16
                                (i32.const 8)                       ;; 17
                                (i32.const 9)                       ;; 18
                                (i32.add)                           ;; 19
                                (i32.add)                           ;; 20
                                (drop)                              ;; 21
                            ))))))                                  ;; 22 + 23 + 24


                ;; total function `func0` gas:
                ;; * before if-stmt: fixed(2)
                ;; * if-condition: fixed(1)
                ;; * if-stmt true-block: fixed(4)
                ;; * if-stmt else-block:
                ;;      * preamble: fixed(2)
                ;;      * inner if-condition: fixed(1)
                ;;      * inner if-stmt true-block: fixed(2)
                ;;      * inner if-stmt else-block: fixed(6)
                ;;          => inner-if stmt gas = fixed(1) * (fixed(2) + fixed(6)) = fixed(1) * range(2, 6) = range(3, 7)
                ;;      => if-stmt else-block total gas = fixed(2) * range(3, 7) = range(5, 9)
                ;;  => if-stmt total gas = fixed(1) * (fixed(4) + range(5, 9)) = fixed(1) * range(4, 9) = range(5, 10)
                ;;
                ;; total function `func0` gas:
                ;; fixed(2) * range(5, 10) = range(7, 12)
        "#;

    let res = estimate_gas!(code);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 7, max: 12 }
        },
        res.unwrap()
    );
}
