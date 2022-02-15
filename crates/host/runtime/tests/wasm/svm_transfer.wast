(module
  ;; import `SVM` vmcall `svm_transfer`
  (func $transfer (import "svm" "svm_transfer") (param $mem_ptr_src_addr i32) (param $mem_ptr_dst_addr i32) (param $amount i64))

  (func (export "ctor")
  	nop)

  (func (export "transfer") (param $mem_ptr_src_addr i32) (param $mem_ptr_dst_addr i32) (param $amount i64)
  	get_local $mem_ptr_src_addr  ;; src_addr
  	get_local $mem_ptr_dst_addr  ;; dst_addr
    get_local $amount            ;; amount
	call $transfer))