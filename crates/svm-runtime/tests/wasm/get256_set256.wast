(module
  ;; import `SVM` vmcalls
  (func $get256 (import "svm" "get256") (param i32) (result i64 i64 i64 i64))
  (func $set256 (import "svm" "set256") (param i32 i64 i64 i64 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "get") (param i32) (result i64 i64 i64 i64)
  	i32.const 0
  	call $get256)

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
	i64.load 

  	get_local 1
	i32.const 24
	i32.add
	i64.load 

	call $set256))