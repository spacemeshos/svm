(module
  ;; import `svm` vmcalls
  (func $reg_write_be_i64 (import "svm" "reg_write_be_i64") (param i64 i32 i32))
  (func $reg_replace_byte (import "svm" "reg_replace_byte") (param i32 i32 i32 i32))
  (func $get_balance (import "env" "get_balance") (param i32 i32) (result i64))
  (func $set_balance (import "env" "set_balance") (param i64 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i64)
        ;; reset register `160:0` with `00...0`
        i64.const 0
        i32.const 160
        i32.const 0
        call $reg_write_be_i64

        ;; replace register `160:0`, offset=19 with value=0x30
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 48  ;; value = 0x30
        i32.const 19   ;; offset = 19
        call $reg_replace_byte

        ;; replace register `160:0`, offset=18 with value=0x20
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 32  ;; value = 0x20
        i32.const 18   ;; offset = 18
        call $reg_replace_byte

        ;; replace register `160:0`, offset=17 with value=0x10
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 16  ;; value = 0x10
        i32.const 17  ;; offset = 17
        call $reg_replace_byte

        ;; read current balance of address `0x000...00_10_20_30`
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $get_balance

        ;; multiply balance by input `factor`
        get_local 0
        i64.mul

        ;; now the top of the stack holds the new balance.
        ;; we'll persist address `0x000...00_10_20_30` (the value of register `160:0`) with the new balance.
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $set_balance))
