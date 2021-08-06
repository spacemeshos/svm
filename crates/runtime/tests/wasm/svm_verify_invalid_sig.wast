(module
  (export "svm_alloc" (func $svm_alloc))
  (export "svm_verify" (func $svm_verify))

  ;; Invalid `svm_verify` signature
  (func $svm_verify (param i32) (result i32)
    (i32.const 0))

  ;;  Valid `svm_alloc` signature
  (func $svm_alloc (param i32) (result i32)
    (i32.const 0)))