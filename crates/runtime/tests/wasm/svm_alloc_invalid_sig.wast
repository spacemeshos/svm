(module
  (export "svm_alloc" (func $svm_alloc))
  (export "svm_verify" (func $svm_verify))

  ;; Valid `svm_verify` signature
  (func $svm_verify (result i32)
    (i32.const 0))

  ;; Invalid `svm_alloc` signature
  (func $svm_alloc (result i32)
    (i32.const 0)))