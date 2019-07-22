(module
  ;; import `svm` vmcalls
  (func $storage_read_to_reg (import "svm" "storage_read_to_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with a `1 page`

  ;; exported function to be called
  (func (export "do_copy_to_reg") (param i32 i32 i32 i32 i32)
        get_local 0 ;; src_page
        get_local 1 ;; src_slice
        get_local 2 ;; offset
        get_local 3 ;; len
        get_local 4 ;; dst_reg
        call $storage_read_to_reg))
