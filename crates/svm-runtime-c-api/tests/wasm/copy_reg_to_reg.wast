(module
  ;; import `svm` vmcalls
  (func $svm_copy_reg_to_reg (import "node" "copy_reg_to_reg")  (param i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "copy_reg_to_reg_proxy") (param i32 i32)
        get_local 0 ;; `src_reg_idx`
        get_local 1 ;; `dst_reg_idx`
        call $svm_copy_reg_to_reg))
