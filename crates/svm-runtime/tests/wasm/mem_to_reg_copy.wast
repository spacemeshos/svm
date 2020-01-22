(module
  ;; import `SVM` vmcalls
  (func $mem_to_reg_copy (import "svm" "mem_to_reg_copy") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32)
        i32.const 0  ;; mem_idx
        get_local 0  ;; mem_offset
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        get_local 3  ;; count
        call $mem_to_reg_copy))
