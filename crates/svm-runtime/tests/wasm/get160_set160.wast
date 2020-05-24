(module
  ;; import `SVM` vmcalls
  (func $get160 (import "svm" "get160") (param i32) (result i64 i64 i32))
  (func $set160 (import "svm" "set160") (param i32 i64 i64 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "get") (param i32) (result i64 i64 i32)
  	i32.const 0
  	call $get160)

  (func (export "store") (param i32 i32)
  	get_local 0

  	get_local 1
	i64.load 

  	get_local 1
	i32.const 8
	i32.add
	i64.load 
	
  	get_local 1
	i32.const 16
	i32.add
	i32.load 

	call $set160))