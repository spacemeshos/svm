(module
  (func $store256 (import "svm" "store256") (param $mem_idx i32) (param $mem_ptr i32) (param $var_id i32))
  (func $load256 (import "svm" "load256") (param $var_id i32) (param $mem_idx i32) (param $mem_ptr i32))

  (import "svm" "memory" (memory 1))

  (func (export "ctor")
  	nop)

  (func (export "store") (param $var_id i32) (param $mem_ptr i32)
  	i32.const 0         ;; mem_idx
	get_local $mem_ptr  ;; mem_ptr
  	get_local $var_id   ;; var_id
	call $store256)

  (func (export "load") (param $var_id i32) (param $mem_ptr i32)
  	get_local $var_id   ;; var_id
  	i32.const 0         ;; mem_idx
  	get_local $mem_ptr  ;; mem_ptr
	call $load256))