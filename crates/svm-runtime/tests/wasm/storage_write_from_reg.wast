(module
  ;; import `SVM` vmcalls
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32 i32)
        get_local 0  ;; reg_bits
        get_local 1  ;; reg_idx
        get_local 2  ;; page_idx
        get_local 3  ;; page_offset
        get_local 4  ;; count
        call $storage_write_from_reg))
