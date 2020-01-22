(module
  ;; import `SVM` vmcalls
  (func $storage_write_from_mem (import "svm" "storage_write_from_mem") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32)
        i32.const 0 ;; mem_idx
        get_local 0 ;; mem_offset
        get_local 1 ;; page_idx
        get_local 2 ;; page_offset
        get_local 3 ;; count
        call $storage_write_from_mem))
