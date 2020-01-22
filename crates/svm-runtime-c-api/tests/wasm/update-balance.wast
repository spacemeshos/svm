(module
  ;; `SVM` vmcalls
  (func $host_ctx_read_into_reg (import "svm" "host_ctx_read_into_reg") (param i32 i32 i32))

  ;; host vmcalls
  (; (func $save_balance (import "env" "save_balance") (param i32 i32)) ;)
  (func $inc_balance (import "env" "inc_balance") (param i32 i32 i64))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "run") (param)
        ;; reg_push(i32, i32)
        ;; copy input `Address` (given in func-buffer) into register

        ;; increment the `Address` balance

        ;; add `delta` to balance (passed via `HostCtx` parameters)

        ;; reg_pop(i32, i32)

        (nop))
