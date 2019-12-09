(module
  ;; import `SVM` vmcalls
  (func $svm_reg_to_mem_copy (import "svm" "reg_to_mem_copy") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "do_copy_to_mem") (param i32 i32 i32)
        i32.const 64 ;; src_reg_bits
        get_local 0  ;; src_reg_idx
        get_local 1  ;; len
        i32.const 0  ;; dst_mem_idx
        get_local 2  ;; dst_mem_ptr
        call $svm_reg_to_mem_copy))
