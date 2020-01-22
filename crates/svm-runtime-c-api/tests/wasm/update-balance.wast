(module
  ;; `SVM` vmcalls
  (func $host_ctx_read_into_reg (import "svm" "host_ctx_read_into_reg") (param i32 i32 i32))

  ;; host vmcalls
  (func $save_balance (import "env" "save_balance") (param i32 i32) (result i64))
  (func $inc_balance (import "env" "inc_balance") (param i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i64)
        ;; reset register `160:0` with `00...0`
        i64.const 0
        i32.const 160
        i32.const 0
        call $reg_write_be_i64

        ;; read current balance of address `0x000...00_10_20_30`
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $get_balance

        ;; here the top of the stack has the balance of account `0x000...00_10_20_30`

        ;; multiply account `0x000...00_10_20_30` balance by input `mul_by` param
        get_local 0
        i64.mul

        i32.const 2   ;; field #2
        i32.const 64  ;; reg 64-bits
        i32.const 3   ;; reg #3
        call $host_ctx_read_into_reg

        ;; now we'll interpret register `64:3` (contains the content of host-ctx field `2`)
        ;; as a 64-bit BigEndian number (this is the `delta`).
        i32.const 64
        i32.const 3
        call $reg_read_be_i64

        ;; here the top of the stack is:
        ;; `delta` (i64)
        ;; `account balace` (i64)

        ;; we're adding `delta` to the account balance
        i64.add

        ;; now the top of the stack holds the new balance.
        ;; we'll persist address `0x000...00_10_20_30` (the value of register `160:0`) with the new balance.
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $set_balance))
