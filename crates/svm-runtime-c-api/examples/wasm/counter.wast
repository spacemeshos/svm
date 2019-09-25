(module
  (func $vmcall_get_counter (import "node" "vmcall_get_counter") (result i32))
  (func $vmcall_inc_counter (import "node" "vmcall_inc_counter") (param i32))

  (memory 1)

  (func (export "inc") (param i32)
        get_local 0
        call $vmcall_inc_counter)

  (func (export "get") (result i32)
        call $vmcall_get_counter))
