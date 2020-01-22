(module
  ;; import `SVM` vmcalls
  (func $storage_read_to_mem (import "svm" "storage_read_to_mem") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i32 i32 i32 i32)
        get_local 0  ;; page_idx
        get_local 1  ;; page_offset
        i32.const 0  ;; mem_idx
        get_local 2  ;; mem_offset
        get_local 3  ;; count
        call $storage_read_to_mem))
