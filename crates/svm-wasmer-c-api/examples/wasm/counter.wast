(module
  (func $get_counter (import "node" "get_counter") (result i32))
  (func $inc_counter (import "node" "inc_counter") (param i32))

  (memory 1)

  (func (export "inc_counter_proxy") (param i32)
        get_local 0 ;; amount
        call $inc_counter)

  (func (export "get_counter_proxy") (result i32)
        call $get_counter))
