(module
  ;; import `SVM` vmcalls
  (func $host_get64 (import "svm" "svm_host_get64") (param i32) (result i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "get_host_ctx") (param i32) (result i64)
      (get_local 0) ;; field_idx
      (call $host_get64)))
