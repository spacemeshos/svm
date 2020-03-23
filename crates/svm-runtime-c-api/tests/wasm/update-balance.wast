(module
  ;; `SVM` vmcalls
  (func $reg_push (import "svm" "reg_push") (param i32 i32))
  (func $reg_pop (import "svm" "reg_pop") (param i32 i32))
  (func $host_ctx_read_i64_le (import "svm" "host_ctx_read_i64_le") (param i32) (result i64))
  (func $buffer_copy_to_reg (import "svm" "buffer_copy_to_reg") (param i32 i32 i32 i32 i32))

  ;; Host vmcalls
  (func $inc_balance (import "env" "inc_balance") (param i64 i32 i32))
  (func $mul_balance (import "env" "mul_balance") (param i64 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "ctor")
    	nop)

  (func (export "run") (param i64)
        ;; save register `160:0`
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $reg_push

        ;; copy input `Address` (given in func-buffer) into register
        i32.const 0    ;; buf_id
        i32.const 0    ;; buf_offset
        i32.const 160  ;; reg_bits
        i32.const 0    ;; reg_idx
        i32.const 20   ;; count (`Address` consumes 20 bytes)
        call $buffer_copy_to_reg

        ;; update the `Address` balance (under register `160:0`)
	;; 1) increment the balance by input parameter (`local 0`).
	;; 2) multiply-by `nonce`.

        get_local 0   ;; addition
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $inc_balance

        ;; Load `nonce` from `Host Context` as 64-bit integer (Little-Endian order).
	;; `Host Context` field `#0` holds the `nonce`.
	i32.const 0  
	call $host_ctx_read_i64_le

	;; here the top of the stack contains the `nonce` (i64)

        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $mul_balance 

        ;; restore register `160:0`
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $reg_pop))
