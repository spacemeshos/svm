(module
  ;; import `SVM` vmcalls
  (func $reg_pop (import "svm" "reg_pop") (param i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32)
        get_local 0 ;; reg_bits
        get_local 1 ;; reg_idx
        call $reg_pop))
