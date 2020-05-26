(module
  (func $get64 (import "svm" "get64") (param i32) (result i64))
  (func $set64 (import "svm" "set64") (param i32 i64))

  (import "svm" "memory" (memory 1))

  (func (export "ctor")
  	nop)

  (func (export "copy64") (param $var_id i32) (param $mem_ptr i32)
  	get_local $var_id

  	get_local $mem_ptr
	i64.load

	call $set64))
  	