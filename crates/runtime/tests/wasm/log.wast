(module
  (func $log (import "svm" "svm_log") (param $offset i32) (param $length i32))

  (import "svm" "memory" (memory $mem 0))

  (func (export "sayHello") 
  	i32.const 0   ;; `data` offset
	  i32.const 11  ;; `data` length = len('Hello World')
	  call $log))