(module
  ;; import `svm` vmcalls
  (func $svm_mem_to_reg_copy (import "svm" "mem_to_reg_copy") (param i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

  ;; exported function to be called
  (func (export "do_copy_to_reg") (param i32 i32 i32)
        i32.const 0 ;; $src_mem_idx
        get_local 0 ;; $src_mem_ptr
        get_local 1 ;; len
        get_local 2 ;; dst_reg
        call $svm_mem_to_reg_copy))
