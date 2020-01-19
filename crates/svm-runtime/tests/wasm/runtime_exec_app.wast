(module
  (func $buffer_copy_to_reg (import "svm" "buffer_copy_to_reg") (param i32 i32 i32 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "run") (param i32 i32 i32 i32)
        i32.const 0  ;; buf_id
        get_local 0  ;; buf_offset
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        get_local 3  ;; len
        call $buffer_copy_to_reg

        ;; persist resgiter value to storage
        get_local 1  ;; reg_bits
        get_local 2  ;; reg_idx
        get_local 3  ;; len
        i32.const 0  ;; page_idx
        i32.const 0  ;; page_offset
        call $storage_write_from_reg))
