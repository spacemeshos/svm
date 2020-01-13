(module
  ;; import `SVM` vmcalls
  (func $reg_write_be_i64 (import "svm" "reg_write_be_i64") (param i64 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 1)  ;; memory `0` (default) is initialized with one page

  ;; exported function to be called
  (func (export "run") (param i64)
        ;; firstly, we set the `i64` value into a register `64:0`
        get_local 0  ;; value
        i32.const 64 ;; reg_bits
        i32.const 0  ;; reg_idx
        call $reg_write_be_i64

        ;; now, we'll persist into `slice 0` (`page=0, offset=0, len=8`)
        i32.const 64 ;; src_reg_bits
        i32.const 0  ;; src_reg_idx
        i32.const 8  ;; len
        i32.const 0  ;; dst_page
        i32.const 0  ;; dst_offset
        call $storage_write_from_reg))
