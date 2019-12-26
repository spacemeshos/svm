(module
  (func $get_counter (import "env" "get_counter") (result i32))
  (func $inc_counter (import "env" "inc_counter") (param i32))

  (memory 1)

  (func (export "inc") (param i32)
        get_local 0
        call $inc_counter)

  (func (export "get") (result i32)
        call $get_counter))
