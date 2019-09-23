(module
  ;; import `svm` vmcalls
  (func $reg_write_be_i64 (import "svm" "reg_write_be_i64") (param i64 i32 i32))
  (func $reg_replace_byte (import "svm" "reg_replace_byte") (param i32 i32 i32 i32))
  (func $vmcall_get_balance  (import "node" "vmcall_get_balance") (param i32 i32) (result i64))
  (func $vmcall_set_balance  (import "node" "vmcall_set_balance") (param i64 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "mul_balance") (param i64)
        ;; reset register `160:0`
        i64.const 0
        i32.const 160
        i32.const 0
        call $reg_write_be_i64

        ;; copy address `0x00...10_20_30` into input register `160:0`
        ;; we'll do that by setting
        ;; byte `5` with `0x10`
        ;; byte `6` with `0x20`
        ;; byte `6` with `0x30`

        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 48  ;; byte = 0x30
        i32.const 19   ;; offset = 19
        call $reg_replace_byte

        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 32  ;; byte = 0x20
        i32.const 18   ;; offset = 18
        call $reg_replace_byte

        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        i32.const 16  ;; byte = 0x10
        i32.const 17  ;; offset = 17
        call $reg_replace_byte

        ;; read current balance of address `0x00...10_20_30` (stored at input register)
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $vmcall_get_balance

        ;; multiply balance by input `factor`
        get_local 0
        i64.mul

        ;; now the top of the stack holds the new balance.
        ;; we'll persist address `0x00...10_20_30` with the new balance.
        i32.const 160 ;; reg_bits
        i32.const 0   ;; reg_idx
        call $vmcall_set_balance))
