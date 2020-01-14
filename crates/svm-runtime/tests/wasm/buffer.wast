(module
  ;; import `SVM` vmcalls
  (func $buffer_create (import "svm" "buffer_create") (param i32 i32))
  (func $buffer_copy_to_storage (import "svm" "buffer_copy_to_storage") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "create") (param i32)
        get_local 0    ;; buffer index
        i32.const 100  ;; buffer capacity
        call $buffer_create )

  (func (export "copy") (param i32 i32 i32 i32 i32)
        get_local 0  ;; buffer index
        get_local 1  ;; buffer offset
        get_local 2  ;; page index
        get_local 3  ;; page offset
        get_local 4  ;; len
        call $buffer_copy_to_storage))
