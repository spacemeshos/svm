(module
  (func $buffer_copy_to_reg (import "svm" "buffer_copy_to_reg") (param i32 i32 i32 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "run") (param i32 i32 i32 i32 i32 i32 i32)
        get_local 0  ;; buf_id
        get_local 1  ;; buf_offset
        get_local 2  ;; reg_bits
        get_local 3  ;; reg_idx
        get_local 4  ;; count
        call $buffer_copy_to_reg

        ;; persist register value into storage
        get_local 2  ;; reg_bits
        get_local 3  ;; reg_idx
        get_local 5  ;; page_idx
        get_local 6  ;; page_offset
        get_local 4  ;; count
        call $storage_write_from_reg))
