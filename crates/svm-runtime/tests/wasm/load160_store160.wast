(module
  (func $store160 (import "svm" "store160") (param $mem_ptr i32) (param $var_id i32))
  (func $load160 (import "svm" "load160") (param $var_id i32) (param $mem_ptr i32))

  (import "svm" "memory" (memory 1))

  (func (export "ctor")
  	nop)

  (func (export "store") (param $var_id i32) (param $mem_ptr i32)
	get_local $mem_ptr  ;; mem_ptr
  	get_local $var_id   ;; var_id
	call $store160)

  (func (export "load") (param $var_id i32) (param $mem_ptr i32)
  	get_local $var_id   ;; var_id
  	get_local $mem_ptr  ;; mem_ptr
	call $load160))