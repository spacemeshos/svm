(module
  ;; import `SVM` vmcalls
  (func $get64 (import "svm" "svm_get64") (param i32) (result i64))
  (func $set64 (import "svm" "svm_set64") (param i32 i64))

  (func (export "get") (param $var_id i32) (result i64)
      (get_local $var_id) 
      (call $get64))

  (func (export "add") (param $var_id i32) (param $amount i64) 
      (get_local $var_id) 

      (get_local $var_id) 
      (call $get64)

      ;; here the stack contains:
      ;; 
      ;; top: 
      ;; +-----------------+
      ;; |  get64(var_id)  |
      ;; +-----------------+
      ;; |     var_id      | 
      ;; +-----------------+
      
      (get_local $amount)  
      (i64.add)
      
      ;; and now the stack contains:
      ;; 
      ;; top: 
      ;; +-----------------+
      ;; |  get64(var_id)  |
      ;; |       +         |
      ;; |     amount      |
      ;; +-----------------+
      ;; |     var_id      | 
      ;; +-----------------+

      (call $set64)))