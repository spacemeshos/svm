(module
  ;; import `SVM` vmcall `svm_transfer`
  (func $transfer (import "svm" "svm_transfer") (param $src_addr i32) (param $dst_addr i32) (param $amount i64))

  (func (export "transfer") (param $src_addr i32) (param $dst_addr i32) (param $amount i64)
	get_local $src_addr  ;; src_addr
  	get_local $dst_addr  ;; dst_addr
    get_local $amount   ;; amount
	call $transfer))