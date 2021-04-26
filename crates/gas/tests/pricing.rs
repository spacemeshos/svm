use maplit::hashmap;

use svm_gas::{price_wasm, FuncIndex, Gas, ImportPriceResolver};

struct TestImportResolver;

impl ImportPriceResolver for TestImportResolver {
    fn price_for(_func: FuncIndex) -> Gas {
        panic!()
    }
}

macro_rules! price_for {
    ($code:expr) => {{
        let wasm = wat::parse_str($code).unwrap();

        price_wasm::<TestImportResolver>(&wasm)
    }};
}

#[test]
fn price_nop_functions() {
    let wasm = r#"
          (module
            (func $func0
            	(nop))

            (func $func1
                 (nop)))

            (func $func2
                (block (block (nop))))

            (func $func3
                (block (block (block (nop))))))
        "#;

    let result = price_for!(wasm);

    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Fixed(0),
            FuncIndex(1) => Gas::Fixed(0),
            FuncIndex(2) => Gas::Fixed(0),
            FuncIndex(3) => Gas::Fixed(0)
        },
        result.unwrap()
    );
}

#[test]
fn price_imports() {
    let wasm = r#"
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

    let result = price_for!(wasm);

    assert_eq!(
        hashmap! {
            FuncIndex(3) => Gas::Fixed(8),
            FuncIndex(4) => Gas::Fixed(10),
        },
        result.unwrap()
    );
}

#[test]
fn price_constant_function() {
    let wasm = r#"
          (module
            (func $func0 (result i32)
                i32.const 10
                drop
                i32.const 20))
        "#;

    let result = price_for!(wasm);
    assert_eq!(hashmap! {FuncIndex(0) => Gas::Fixed(3)}, result.unwrap());
}

#[test]
fn price_if_stmt_without_else() {
    let wasm = r#"
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

    let result = price_for!(wasm);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 3, max: 7 }
        },
        result.unwrap()
    );
}

#[test]
fn price_if_stmt_without_else_nested() {
    let wasm = r#"
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

    let result = price_for!(wasm);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 3, max: 14 }
        },
        result.unwrap()
    );
}

#[test]
fn price_if_stmt_with_else_not_nested() {
    let wasm = r#"
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

    let result = price_for!(wasm);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 5, max: 7 }
        },
        result.unwrap()
    );
}

#[test]
fn price_if_stmt_with_else_nested() {
    let wasm = r#"
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

    let result = price_for!(wasm);
    assert_eq!(
        hashmap! {
            FuncIndex(0) => Gas::Range { min: 7, max: 12 }
        },
        result.unwrap()
    );
}
