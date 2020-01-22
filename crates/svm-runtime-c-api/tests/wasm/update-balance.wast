(module
  ;; `SVM` vmcalls
  (func $reg_push (import "svm" "reg_push") (param i32 i32))
  (func $reg_pop (import "svm" "reg_pop") (param i32 i32))
  (func $buffer_copy_to_reg (import "svm" "buffer_copy_to_reg") (param i32 i32 i32 i32 i32))
  (; (func $host_ctx_read_into_reg (import "svm" "host_ctx_read_into_reg") (param i32 i32 i32)) ;)

  ;; host vmcalls
  (func $inc_balance (import "env" "inc_balance") (param i32 i32 i64))
  (func $mult_balance (import "env" "mul_balance") (param i32 i32 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

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
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        get_local 0   ;; addition
        call $inc_balance

        ;; restore register `160:0`
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $reg_pop))
