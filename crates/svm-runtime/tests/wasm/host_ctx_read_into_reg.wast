(module
  ;; import `SVM` vmcalls
  (func $svm_host_ctx_read_into_reg (import "svm" "host_ctx_read_into_reg") (param i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32)
        get_local 0  ;; field index
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        call $svm_host_ctx_read_into_reg ))
