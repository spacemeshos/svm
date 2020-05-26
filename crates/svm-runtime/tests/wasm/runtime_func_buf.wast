(module
  (func $store160 (import "svm" "store160") (param $mem_idx i32) (param $mem_ptr i32) (param $var_id i32))

  (import "svm" "memory" (memory 1))

  (func (export "ctor")
  	nop)

  (func (export "store_addr") (param $var_id i32) (param $mem_ptr i32)
  	i32.const 0 ;; mem_idx
	get_local $mem_ptr 
  	get_local $var_id 
	
	call $store160))
