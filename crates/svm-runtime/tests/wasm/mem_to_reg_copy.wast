(module
  ;; import `SVM` vmcalls
  (func $mem_to_reg_copy (import "svm" "mem_to_reg_copy") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32)
        i32.const 0  ;; src_mem_idx
        get_local 0  ;; src_mem_ptr
        get_local 1  ;; len
        i32.const 64 ;; dst_reg_bits
        get_local 2  ;; dst_reg_idx
        call $mem_to_reg_copy))
