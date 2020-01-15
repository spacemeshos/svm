(module
  (func $reg_write_be_i64 (import "svm" "reg_write_be_i64") (param i64 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "ctor")
        (nop))

  (func (export "run") (param i64 i32 i32 i32 i32)
        ;; register set
        get_local 0  ;; value
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        call $reg_write_be_i64

        ;; persist to storage
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        i32.const 8  ;; len
        get_local 3  ;; page_idx
        get_local 4  ;; page_offset
        call $storage_write_from_reg))
