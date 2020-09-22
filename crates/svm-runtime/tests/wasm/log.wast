(module
  (func $log (import "svm" "svm_log") (param $msg_ptr i32) (param $msg_len i32) (param $code i32))

  (import "svm" "memory" (memory $mem 0))

  (func (export "sayHello") 
  	i32.const 0   ;; msg_ptr
	i32.const 11  ;; msg_len = len('Hello World')
	i32.const 200 ;; code
	call $log))
  	