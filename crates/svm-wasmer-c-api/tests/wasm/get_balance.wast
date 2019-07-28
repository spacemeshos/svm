(module
  ;; import `svm` vmcalls
  (func $svm_get_balance  (import "node" "get_balance")  (param i32) (result i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "get_balance_proxy") (param i32) (result i64)
        get_local 0 ;; addr
        call $svm_get_balance))
