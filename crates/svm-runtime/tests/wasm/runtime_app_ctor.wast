(module
  (func $set64 (import "svm" "set64") (param i32 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "ctor") (param i64)
    i32.const 0  ;; var_id = 0
    get_local 0  ;; var's value 
    call $set64))
