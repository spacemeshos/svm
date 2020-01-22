(module
  ;; import `SVM` vmcalls
  (func $storage_read_to_reg (import "svm" "storage_read_to_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32 i32)
        get_local 0  ;; page_idx
        get_local 1  ;; page_offset
        get_local 2  ;; reg_bits
        get_local 3  ;; reg_idx
        get_local 4  ;; count
        call $storage_read_to_reg))
