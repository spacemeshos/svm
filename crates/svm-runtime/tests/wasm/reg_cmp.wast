(module
  ;; import `SVM` vmcalls
  (func $reg_cmp (import "svm" "reg_cmp") (param i32 i32 i32) (result i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "reg_128_cmp") (param i32 i32) (result i32)
        i32.const 128  ;; reg_bits
        get_local 0    ;; reg_idx1
        get_local 1    ;; reg_idx2
        call $reg_cmp))
