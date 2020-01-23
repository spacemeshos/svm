(module
  ;; import `SVM` vmcalls
  (func $host_ctx_read_i32_be (import "svm" "host_ctx_read_i32_be") (param i32) (result i32))
  (func $host_ctx_read_i32_le (import "svm" "host_ctx_read_i32_le") (param i32) (result i32))
  (func $host_ctx_read_i64_be (import "svm" "host_ctx_read_i64_be") (param i32) (result i64))
  (func $host_ctx_read_i64_le (import "svm" "host_ctx_read_i64_le") (param i32) (result i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "read_i32") (param i32 i32) (result i32)
        (if (result i32) (get_local 1)
          (then
            ;; Big-Endian
           (get_local 0) ;; field_idx
           (call $host_ctx_read_i32_be))
          (else
            ;; Little-Endian
            (get_local 0) ;; field_idx
            (call $host_ctx_read_i32_le))))


  (func (export "read_i64") (param i32 i32 i32 i32) (result i64)
        (if (result i64) (get_local 1)
          (then
            ;; Big-Endian
           (get_local 0) ;; field_idx
           (call $host_ctx_read_i64_be))
          (else
            ;; Little-Endian
            (get_local 0) ;; field_idx
            (call $host_ctx_read_i64_le)))))


