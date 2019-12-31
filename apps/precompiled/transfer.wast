(module
  (type (;0;) (func (param i32 i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32) (result i64)))
  (type (;2;) (func (param i32 i32 i64)))
  (type (;3;) (func (param i64 i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i64) (result i32)))
  (import "env" "mem_to_reg_copy" (func $mem_to_reg_copy (type 0)))
  (import "env" "get_balance_from_reg" (func $get_balance_from_reg (type 1)))
  (import "env" "set_balance_from_reg" (func $set_balance_from_reg (type 2)))
  (import "env" "storage_read_to_reg" (func $storage_read_to_reg (type 0)))
  (import "env" "reg_read_le_i64" (func $reg_read_le_i64 (type 1)))
  (import "env" "reg_write_le_i64" (func $reg_write_le_i64 (type 3)))
  (import "env" "storage_write_from_reg" (func $storage_write_from_reg (type 0)))
  (func $execute (type 4) (param i32 i32 i32 i32 i64) (result i32)
    (local i64 i64)
    local.get 0
    local.get 1
    i32.const 32
    i32.const 256
    i32.const 0
    call $mem_to_reg_copy
    i32.const -1
    local.set 0
    block  ;; label = @1
      i32.const 256
      i32.const 0
      call $get_balance_from_reg
      local.tee 5
      local.get 4
      i64.lt_s
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.const 32
      i32.const 256
      i32.const 1
      call $mem_to_reg_copy
      i32.const 256
      i32.const 1
      call $get_balance_from_reg
      local.set 6
      i32.const 256
      i32.const 0
      local.get 5
      local.get 4
      i64.sub
      call $set_balance_from_reg
      i32.const 256
      i32.const 1
      local.get 6
      local.get 4
      i64.add
      call $set_balance_from_reg
      i32.const 0
      i32.const 0
      i32.const 8
      i32.const 64
      i32.const 0
      call $storage_read_to_reg
      i32.const 64
      i32.const 0
      call $reg_read_le_i64
      i64.const 1
      i64.add
      i32.const 64
      i32.const 0
      call $reg_write_le_i64
      i32.const 64
      i32.const 0
      i32.const 8
      i32.const 0
      i32.const 0
      call $storage_write_from_reg
      i32.const 0
      local.set 0
    end
    local.get 0)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (export "execute" (func $execute)))
