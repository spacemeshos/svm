(module
  ;; import `SVM` vmcalls
  (func $storage_write_i32_be (import "svm" "storage_write_i32_be") (param i32 i32 i32 i32))
  (func $storage_write_i32_le (import "svm" "storage_write_i32_le") (param i32 i32 i32 i32))
  (func $storage_write_i64_be (import "svm" "storage_write_i64_be") (param i32 i32 i64 i32))
  (func $storage_write_i64_le (import "svm" "storage_write_i64_le") (param i32 i32 i64 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; Exports
  (func (export "write_i32") (param i32 i32 i32 i32 i32)
        (if (get_local 4)
          (then
            ;; Big-Endian
           (get_local 0) ;; page_idx
           (get_local 1) ;; page_offset
           (get_local 2) ;; n
           (get_local 3) ;; nbytes
           (call $storage_write_i32_be))
          (else
            ;; Little-Endian
            (get_local 0) ;; page_idx
            (get_local 1) ;; page_offset
            (get_local 2) ;; n
            (get_local 3) ;; nbytes
            (call $storage_write_i32_le))))

  (func (export "write_i64") (param i32 i32 i64 i32 i32)
        (if (get_local 4)
          (then
            ;; Big-Endian
           (get_local 0) ;; page_idx
           (get_local 1) ;; page_offset
           (get_local 2) ;; n
           (get_local 3) ;; nbytes
           (call $storage_write_i64_be))
          (else
            ;; Little-Endian
            (get_local 0) ;; page_idx
            (get_local 1) ;; page_offset
            (get_local 2) ;; n
            (get_local 3) ;; nbytes
            (call $storage_write_i64_le)))))
