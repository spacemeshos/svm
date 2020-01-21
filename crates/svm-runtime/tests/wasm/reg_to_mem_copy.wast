(module
  ;; import `SVM` vmcalls
  (func $svm_reg_to_mem_copy (import "svm" "reg_to_mem_copy") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32)
        get_local 0  ;; reg_bits
        get_local 1  ;; reg_idx
        get_local 3  ;; len
        i32.const 0  ;; mem_idx
        get_local 2  ;; mem_ptr
        call $svm_reg_to_mem_copy))
