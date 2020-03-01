(module
  ;; import `SVM` vmcalls
  ;; i32
  (func $reg_set_i32_be (import "svm" "reg_set_i32_be") (param i32 i32 i32))
  (func $reg_set_i32_le (import "svm" "reg_set_i32_le") (param i32 i32 i32))

  ;; i64
  (func $reg_set_i64_be (import "svm" "reg_set_i64_be") (param i32 i32 i64))
  (func $reg_set_i64_le (import "svm" "reg_set_i64_le") (param i32 i32 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exports
  (func (export "run_i32_be") (param i32 i32 i32)
        get_local 0 ;; reg_bits
        get_local 1 ;; reg_idx
	get_local 2 ;; n
        call $reg_set_i32_be)

  (func (export "run_i32_le") (param i32 i32 i32)
        get_local 0 ;; reg_bits
        get_local 1 ;; reg_idx
	get_local 2 ;; n
        call $reg_set_i32_le)

  (func (export "run_i64_be") (param i32 i32 i64)
        get_local 0 ;; reg_bits
        get_local 1 ;; reg_idx
	get_local 2 ;; n
        call $reg_set_i64_be)

  (func (export "run_i64_le") (param i32 i32 i64)
        get_local 0 ;; reg_bits
        get_local 1 ;; reg_idx
	get_local 2 ;; n
        call $reg_set_i64_le))