(module
  ;; import `svm` vmcalls
  (func $reg_write_le_i64 (import "svm" "reg_write_le_i64") (param i64 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported functions to be called
  (func (export "do_reg_set") (param i64 i32 i32)
      get_local 0  ;; value
      get_local 1  ;; reg_bits
      get_local 2  ;; reg_idx
      call $reg_write_le_i64)

  (func (export "do_write_from_reg") (param i32 i32 i32 i32 i32)
        i32.const 64 ;; src_reg_bits
        get_local 0  ;; src_reg_idx
        get_local 1  ;; len
        get_local 2  ;; dst_page
        get_local 3  ;; dst_slice
        get_local 4  ;; dst_offset
        call $storage_write_from_reg))
