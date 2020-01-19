(module
  (func $buffer_copy_to_storage (import "svm" "buffer_copy_to_storage") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "ctor") (param i32)
    i32.const 0  ;; buf_id
    i32.const 0  ;; buf_offset
    i32.const 0  ;; page_idx
    i32.const 0  ;; page_offset
    get_local 0  ;; len
    call $buffer_copy_to_storage))
