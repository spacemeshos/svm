(module
  (func $get_counter (import "node" "get_counter") (result i32))
  (func $inc_counter_from_reg (import "node" "inc_counter_from_reg") (param i32))

  (memory 1)

  (func (export "inc_counter_proxy") (param i32)
        get_local 0 ;; reg_idx
        call $inc_counter_from_reg)

  (func (export "get_counter_proxy") (result i32)
        call $get_counter))
