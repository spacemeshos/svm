(module
  (func $get_counter (import "node" "get_counter") (result i32))
  (func $inc_counter (import "node" "inc_counter") (param i32))

  (func (export "inc_counter_proxy") (param i32)
        get_local 0 ;; amount
        call $inc_counter)

  (func (export "get_counter_proxy")
        call $get_counter))
