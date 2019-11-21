(module
  ;; import `svm` vmcalls
  (func $storage_write_from_mem (import "svm" "storage_write_from_mem") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "do_write_from_mem") (param i32 i32 i32 i32 i32)
        i32.const 0 ;; src_mem_idx
        get_local 0 ;; src_mem_ptr
        get_local 1 ;; len
        get_local 2 ;; dst_page
        get_local 3 ;; dst_offset
        call $storage_write_from_mem))
