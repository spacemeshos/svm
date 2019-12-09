(module
  ;; import `SVM` vmcalls
  (func $svm_reg_replace_byte (import "svm" "reg_replace_byte") (param i32 i32 i32 i32))
  (func $svm_reg_read_be_i64 (import "svm" "reg_read_be_i64") (param i32 i32) (result i64))
  (func $svm_reg_write_be_i64 (import "svm" "reg_write_be_i64") (param i64 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  (func (export "inc") (param i32)
        ;; loading the register value
        i32.const 64 ;; reg_bits
        get_local 0  ;; reg_idx
        call $svm_reg_read_be_i64

        ;; now, the top of the stack should have
        ;; the register value as LittleEndian i64

        ;; incrementing value
        i64.const 1
        i64.add

        ;; write back to register the new value
        i32.const 64 ;; reg_bits
        get_local 0  ;; reg_idx
        call $svm_reg_write_be_i64)

  (func (export "replace") (param i32 i32 i32)
        ;; write back to register the new value
        i32.const 64 ;; reg_bits
        get_local 0  ;; reg_idx
        get_local 1  ;; byte
        get_local 2  ;; offset
        call $svm_reg_replace_byte))
