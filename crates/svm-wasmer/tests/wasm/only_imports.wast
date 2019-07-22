(module
  ;; import `svm` vmcalls
  (func $svm_mem_to_reg_copy    (import "svm" "mem_to_reg_copy")        (param i32 i32 i32 i32))
  (func $svm_reg_to_mem_copy    (import "svm" "reg_to_mem_copy")        (param i32 i32 i32 i32))
  (func $storage_read_to_reg    (import "svm" "storage_read_to_reg")    (param i32 i32 i32 i32 i32))
  (func $storage_read_to_mem    (import "svm" "storage_read_to_mem")    (param i32 i32 i32 i32 i32 i32))
  (func $storage_write_from_mem (import "svm" "storage_write_from_mem") (param i32 i32 i32 i32 i32 i32))
  (func $storage_write_from_reg (import "svm" "storage_write_from_reg") (param i32 i32 i32 i32 i32))

  (memory 100)

  ;; exported function to be called
  (func (export "do_nothing")))
