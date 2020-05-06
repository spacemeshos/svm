(module
  ;; import `SVM` vmcalls
  (func $get64 (import "svm" "get64") (param i32) (result i64))
  (func $set64 (import "svm" "set64") (param i32 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "get") (param i32) (result i64)
      (get_local 0) ;; var_id
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
      ;;
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