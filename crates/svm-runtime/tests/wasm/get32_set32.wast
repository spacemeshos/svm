
(module
  ;; import `SVM` vmcalls
  (func $get32 (import "svm" "get32") (param i32) (result i32))
  (func $set32 (import "svm" "set32") (param i32 i32))

  (func (export "get") (param $var_id i32) (result i32)
      get_local $var_id 
      call $get32)

  (func (export "add") (param $var_id i32) (param $amount i32) 
      (get_local $var_id) 

      (get_local $var_id) 
      (call $get32)

      ;; here the stack contains:
      ;; 
      ;; top: 
      ;; +-----------------+
      ;; |  get32(var_id)  |
      ;; +-----------------+
      ;; |     var_id      | 
      ;; +-----------------+

      (get_local $amount)  
      (i32.add)
      
      ;; and now the stack contains:
      ;; 
      ;; top: 
      ;; +-----------------+
      ;; |  get32(var_id)  |
      ;; |       +         |
      ;; |     amount      |
      ;; +-----------------+
      ;; |     var_id      | 
      ;; +-----------------+

      (call $set32)))