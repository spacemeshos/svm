(module
  (func $get64 (import "svm" "svm_get64") (param i32) (result i64))
  (func $set64 (import "svm" "svm_set64") (param i32 i64))

  (func (export "ctor")
  	nop)

  (func (export "add") (param i64)
    ;; push var_id = 0 for later `$set64` usage
    i32.const 0  

    ;; read var #0
    i32.const 0  ;; var_id = 0
    call $get64

    ;; calculate var #0 new value
    get_local 0
    i64.add

    ;; store var #0 new value
    call $set64))