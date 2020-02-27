(module
  (type (;0;) (func (param i32 i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32 i32)))
  (type (;2;) (func (param i32) (result i64)))
  (type (;3;) (func (param i32 i32 i64 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32) (result i64)))
  (type (;8;) (func (param i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;10;) (func (result i32)))
  (type (;11;) (func))
  (type (;12;) (func (param i32)))
  (type (;13;) (func (param i64)))
  (type (;14;) (func (param i32 i32) (result i32)))
  (type (;15;) (func (result i64)))
  (import "env" "buffer_copy_to_storage" (func $buffer_copy_to_storage (type 0)))
  (import "env" "storage_write_i32_be" (func $storage_write_i32_be (type 1)))
  (import "env" "host_ctx_read_i64_be" (func $host_ctx_read_i64_be (type 2)))
  (import "env" "storage_write_i64_be" (func $storage_write_i64_be (type 3)))
  (import "env" "storage_read_i32_be" (func $storage_read_i32_be (type 4)))
  (import "env" "reg_push" (func $reg_push (type 5)))
  (import "env" "buffer_copy_to_reg" (func $buffer_copy_to_reg (type 0)))
  (import "env" "add_balance_i32" (func $add_balance_i32 (type 6)))
  (import "env" "reg_pop" (func $reg_pop (type 5)))
  (import "env" "storage_read_i64_be" (func $storage_read_i64_be (type 7)))
  (import "env" "host_ctx_read_i32_be" (func $host_ctx_read_i32_be (type 8)))
  (import "env" "host_ctx_read_into_reg" (func $host_ctx_read_into_reg (type 6)))
  (import "env" "storage_write_from_reg" (func $storage_write_from_reg (type 0)))
  (import "env" "storage_read_to_reg" (func $storage_read_to_reg (type 0)))
  (import "env" "reg_cmp" (func $reg_cmp (type 4)))
  (import "env" "reg_set_i32_be" (func $reg_set_i32_be (type 6)))
  (func $init (type 9) (param i32 i32 i32 i32 i32 i32)
    (local i64)
    i32.const 0
    i32.const 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    local.get 0
    select
    call $buffer_copy_to_storage
    i32.const 0
    i32.const 96
    local.get 0
    i32.const 0
    i32.ne
    i32.const 1
    call $storage_write_i32_be
    i32.const 0
    i32.const 0
    i32.const 0
    call $host_ctx_read_i64_be
    local.tee 6
    i32.const 8
    call $storage_write_i64_be
    i32.const 0
    i32.const 0
    local.get 6
    i32.const 8
    call $storage_write_i64_be
    i32.const 0
    i32.const 0
    local.get 1
    i32.const 4
    call $storage_write_i32_be
    i32.const 0
    i32.const 0
    local.get 2
    i32.const 4
    call $storage_write_i32_be
    local.get 2
    local.get 3
    call $write_layer_liquidation)
  (func $write_layer_liquidation (type 5) (param i32 i32)
    (local i32 i32)
    i32.const 0
    call $host_ctx_read_i64_be
    drop
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        call $host_ctx_read_i32_be
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.div_u
        local.tee 3
        local.get 2
        i32.mul
        local.get 1
        i32.eq
        br_if 1 (;@1;)
        i32.const 1048863
        i32.const 49
        i32.const 1048912
        call $_ZN4core9panicking5panic17hfe328d3c36493822E
        unreachable
      end
      i32.const 1048928
      i32.const 57
      i32.const 1048988
      call $_ZN4core9panicking5panic17hfe328d3c36493822E
      unreachable
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.div_u
        local.tee 1
        local.get 3
        i32.mul
        local.get 0
        i32.eq
        br_if 1 (;@1;)
        i32.const 1049023
        i32.const 43
        i32.const 1049068
        call $_ZN4core9panicking5panic17hfe328d3c36493822E
        unreachable
      end
      i32.const 1048928
      i32.const 57
      i32.const 1049084
      call $_ZN4core9panicking5panic17hfe328d3c36493822E
      unreachable
    end
    block  ;; label = @1
      local.get 1
      i32.const 65535
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      local.get 1
      i32.const 2
      call $storage_write_i32_be
      return
    end
    i32.const 1049220
    i32.const 37
    i32.const 1049260
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $get_liquidated (type 10) (result i32)
    call $refresh_liquidation
    i32.const 0
    i32.const 0
    i32.const 4
    call $storage_read_i32_be)
  (func $refresh_liquidation (type 11)
    (local i32 i64 i64 i32 i32)
    i32.const 0
    i32.const 0
    i32.const 2
    call $storage_read_i32_be
    local.set 0
    i32.const 0
    i32.const 0
    i32.const 8
    call $storage_read_i64_be
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          call $host_ctx_read_i64_be
          local.tee 2
          local.get 1
          i64.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i64.sub
          local.get 0
          i64.extend_i32_u
          i64.mul
          local.tee 1
          i64.const 4294967295
          i64.gt_u
          br_if 1 (;@2;)
          i32.const 0
          i32.const 0
          i32.const 4
          call $storage_read_i32_be
          local.set 3
          i32.const 0
          i32.const 0
          i32.const 4
          call $storage_read_i32_be
          local.tee 4
          local.get 1
          i32.wrap_i64
          local.tee 0
          i32.ge_u
          br_if 2 (;@1;)
          i32.const 1048708
          i32.const 39
          i32.const 1048748
          call $_ZN4core9panicking5panic17hfe328d3c36493822E
          unreachable
        end
        i32.const 1049100
        i32.const 45
        i32.const 1049148
        call $_ZN4core9panicking5panic17hfe328d3c36493822E
        unreachable
      end
      i32.const 1049164
      i32.const 40
      i32.const 1049204
      call $_ZN4core9panicking5panic17hfe328d3c36493822E
      unreachable
    end
    i32.const 0
    i32.const 0
    local.get 2
    i32.const 8
    call $storage_write_i64_be
    i32.const 0
    i32.const 0
    local.get 3
    local.get 0
    i32.add
    i32.const 4
    call $storage_write_i32_be
    i32.const 0
    i32.const 0
    local.get 4
    local.get 0
    i32.sub
    i32.const 4
    call $storage_write_i32_be)
  (func $transfer (type 12) (param i32)
    call $refresh_liquidation
    block  ;; label = @1
      i32.const 0
      i32.const 0
      i32.const 4
      call $storage_read_i32_be
      local.get 0
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 1048652
      i32.const 38
      i32.const 1048692
      call $_ZN4core9panicking5panic17hfe328d3c36493822E
      unreachable
    end
    i32.const 160
    i32.const 0
    call $reg_push
    i32.const 0
    i32.const 0
    i32.const 160
    i32.const 0
    i32.const 20
    call $buffer_copy_to_reg
    local.get 0
    i32.const 160
    i32.const 0
    call $add_balance_i32
    i32.const 160
    i32.const 0
    call $reg_pop)
  (func $_ZN4core9panicking5panic17hfe328d3c36493822E (type 6) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 20
    i32.add
    i32.const 0
    i32.store
    local.get 3
    i32.const 1048824
    i32.store offset=16
    local.get 3
    i64.const 1
    i64.store offset=4 align=4
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h96963a422a9547f5E
    unreachable)
  (func $prepare (type 12) (param i32)
    i32.const 1048586
    i32.const 31
    i32.const 1048620
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $apporove (type 12) (param i32)
    i32.const 1048586
    i32.const 31
    i32.const 1048636
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $do_transfer (type 12) (param i32)
    call $refresh_liquidation
    block  ;; label = @1
      i32.const 0
      i32.const 0
      i32.const 4
      call $storage_read_i32_be
      local.get 0
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 160
      i32.const 0
      call $reg_push
      i32.const 0
      i32.const 0
      i32.const 160
      i32.const 0
      i32.const 20
      call $buffer_copy_to_reg
      local.get 0
      i32.const 160
      i32.const 0
      call $add_balance_i32
      i32.const 160
      i32.const 0
      call $reg_pop
      return
    end
    i32.const 1048652
    i32.const 38
    i32.const 1048692
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h96963a422a9547f5E (type 5) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 1048764
    i32.store offset=4
    local.get 2
    i32.const 1048824
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 12) (param i32)
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17hea79acfb6c65f1feE
    drop
    call $_ZN3std9panicking20rust_panic_with_hook17he921fcf08d5eb7d9E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h04478f7f402db464E (type 12) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hcbe1e308a536e760E (type 2) (param i32) (result i64)
    i64.const -225553400743110688)
  (func $_ZN3std9panicking20rust_panic_with_hook17he921fcf08d5eb7d9E (type 11)
    (local i32 i32)
    i32.const 1
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1049392
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i64.const 4294967297
          i64.store offset=1049392
          br 1 (;@2;)
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049396
        i32.const 1
        i32.add
        local.tee 0
        i32.store offset=1049396
        local.get 0
        i32.const 2
        i32.gt_u
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.load offset=1049400
      local.tee 1
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.store offset=1049400
      local.get 0
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 11)
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17hea79acfb6c65f1feE (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048780
      i32.const 43
      i32.const 1048824
      call $_ZN4core9panicking5panic17hfe328d3c36493822E
      unreachable
    end
    local.get 0)
  (func $write_pub_keys (type 12) (param i32)
    i32.const 0
    i32.const 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    local.get 0
    select
    call $buffer_copy_to_storage
    i32.const 0
    i32.const 96
    local.get 0
    i32.const 0
    i32.ne
    i32.const 1
    call $storage_write_i32_be)
  (func $write_first_layer (type 11)
    (local i64)
    i32.const 0
    i32.const 0
    i32.const 0
    call $host_ctx_read_i64_be
    local.tee 0
    i32.const 8
    call $storage_write_i64_be
    i32.const 0
    i32.const 0
    local.get 0
    i32.const 8
    call $storage_write_i64_be)
  (func $write_pending_pub_key (type 11)
    i32.const 256
    i32.const 0
    call $reg_push
    i32.const 0
    i32.const 256
    i32.const 0
    call $host_ctx_read_into_reg
    i32.const 256
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    call $storage_write_from_reg
    i32.const 256
    i32.const 0
    call $reg_pop)
  (func $write_liquidated (type 12) (param i32)
    i32.const 0
    i32.const 0
    local.get 0
    i32.const 4
    call $storage_write_i32_be)
  (func $write_last_run_layer (type 13) (param i64)
    i32.const 0
    i32.const 0
    local.get 0
    i32.const 8
    call $storage_write_i64_be)
  (func $is_multisig (type 10) (result i32)
    i32.const 0)
  (func $pub_key_auth (type 11)
    (local i32 i32)
    i32.const 256
    i32.const 0
    call $reg_push
    i32.const 256
    i32.const 1
    call $reg_push
    i32.const 0
    i32.const 256
    i32.const 0
    call $host_ctx_read_into_reg
    i32.const 0
    local.set 0
    block  ;; label = @1
      loop  ;; label = @2
        local.get 0
        i32.const 32
        i32.add
        local.tee 1
        i32.const 128
        i32.eq
        br_if 1 (;@1;)
        i32.const 0
        local.get 0
        i32.const 256
        i32.const 1
        i32.const 32
        call $storage_read_to_reg
        local.get 1
        local.set 0
        i32.const 256
        i32.const 0
        i32.const 1
        call $reg_cmp
        br_if 0 (;@2;)
      end
      i32.const 256
      i32.const 1
      call $reg_pop
      i32.const 256
      i32.const 0
      call $reg_pop
      return
    end
    i32.const 1049288
    i32.const 11
    i32.const 1049300
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $auth (type 10) (result i32)
    (local i32)
    call $pub_key_auth
    i32.const 256
    i32.const 0
    call $reg_push
    i32.const 256
    i32.const 1
    call $reg_push
    i32.const 0
    i32.const 96
    i32.const 256
    i32.const 0
    i32.const 32
    call $storage_read_to_reg
    i32.const 0
    i32.const 256
    i32.const 1
    call $host_ctx_read_into_reg
    block  ;; label = @1
      block  ;; label = @2
        i32.const 256
        i32.const 0
        i32.const 1
        call $reg_cmp
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        i32.const 256
        i32.const 0
        call $reg_push
        i32.const 0
        i32.const 256
        i32.const 0
        call $host_ctx_read_into_reg
        br 1 (;@1;)
      end
      i32.const 256
      i32.const 0
      call $reg_push
      i32.const 256
      i32.const 0
      i32.const 4
      call $reg_set_i32_be
    end
    i32.const 256
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    call $storage_write_from_reg
    i32.const 256
    i32.const 0
    call $reg_pop
    i32.const 256
    i32.const 0
    call $reg_pop
    i32.const 256
    i32.const 1
    call $reg_pop
    local.get 0)
  (func $pub_key_cmp (type 14) (param i32 i32) (result i32)
    i32.const 256
    local.get 0
    local.get 1
    call $reg_cmp)
  (func $read_pending_pub_key (type 5) (param i32 i32)
    i32.const 0
    i32.const 96
    local.get 0
    local.get 1
    i32.const 32
    call $storage_read_to_reg)
  (func $read_current_layer (type 15) (result i64)
    i32.const 0
    call $host_ctx_read_i64_be)
  (func $read_pub_key (type 6) (param i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.const 3
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.const 5
      i32.shl
      local.get 1
      local.get 2
      i32.const 32
      call $storage_read_to_reg
      return
    end
    i32.const 1049327
    i32.const 30
    i32.const 1049360
    call $_ZN4core9panicking5panic17hfe328d3c36493822E
    unreachable)
  (func $read_first_layer (type 15) (result i64)
    i32.const 0
    i32.const 0
    i32.const 8
    call $storage_read_i64_be)
  (func $read_liquidated (type 10) (result i32)
    i32.const 0
    i32.const 0
    i32.const 4
    call $storage_read_i32_be)
  (func $read_layer_liquidation (type 10) (result i32)
    i32.const 0
    i32.const 0
    i32.const 2
    call $storage_read_i32_be)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049404))
  (global (;2;) i32 (i32.const 1049404))
  (export "memory" (memory 0))
  (export "init" (func $init))
  (export "write_layer_liquidation" (func $write_layer_liquidation))
  (export "get_liquidated" (func $get_liquidated))
  (export "refresh_liquidation" (func $refresh_liquidation))
  (export "transfer" (func $transfer))
  (export "prepare" (func $prepare))
  (export "apporove" (func $apporove))
  (export "do_transfer" (func $do_transfer))
  (export "write_pub_keys" (func $write_pub_keys))
  (export "write_first_layer" (func $write_first_layer))
  (export "write_pending_pub_key" (func $write_pending_pub_key))
  (export "write_liquidated" (func $write_liquidated))
  (export "write_last_run_layer" (func $write_last_run_layer))
  (export "is_multisig" (func $is_multisig))
  (export "pub_key_auth" (func $pub_key_auth))
  (export "auth" (func $auth))
  (export "pub_key_cmp" (func $pub_key_cmp))
  (export "read_pending_pub_key" (func $read_pending_pub_key))
  (export "read_current_layer" (func $read_current_layer))
  (export "read_pub_key" (func $read_pub_key))
  (export "read_first_layer" (func $read_first_layer))
  (export "read_liquidated" (func $read_liquidated))
  (export "read_layer_liquidation" (func $read_layer_liquidation))
  (export "write_unliquidated" (func $write_liquidated))
  (export "read_last_run_layer" (func $read_first_layer))
  (export "read_unliquidated" (func $read_liquidated))
  (export "get_unliquidated" (func $get_liquidated))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) $_ZN4core3ptr13drop_in_place17h04478f7f402db464E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hcbe1e308a536e760E)
  (data (;0;) (i32.const 1048576) "src/lib.rsassertion failed: is_multisig()\00\00\00\00\00\10\00\0a\00\00\00\84\00\00\00\05\00\00\00\00\00\10\00\0a\00\00\00\90\00\00\00\05\00\00\00assertion failed: liquidated >= amount\00\00\00\00\10\00\0a\00\00\00\9f\00\00\00\09\00\00\00assertion failed: unliquidated >= delta\00\00\00\10\00\0a\00\00\00\b7\00\00\00\05\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00\08\01\10\00\17\00\00\00y\01\00\00\0f\00\00\00src/libstd/panicking.rsassertion failed: time_interval % layer_time == 0\ac\01\10\00\13\00\00\00!\00\00\00\05\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00\ac\01\10\00\13\00\00\00!\00\00\00\0d\00\00\00src/computations.rsassertion failed: amount % layer_count == 0\00\00\ac\01\10\00\13\00\00\00,\00\00\00\05\00\00\00\ac\01\10\00\13\00\00\00,\00\00\00\0d\00\00\00assertion failed: current_layer >= last_layer\00\00\00\ac\01\10\00\13\00\00\008\00\00\00\05\00\00\00assertion failed: delta <= 0xFF_FF_FF_FF\ac\01\10\00\13\00\00\00<\00\00\00\05\00\00\00assertion failed: layer_liq <= 0xFFFF\00\00\00\bc\02\10\00\0c\00\00\00;\00\00\00\09\00\00\00src/write.rsauth failed\00\e4\02\10\00\0b\00\00\00(\00\00\00\05\00\00\00src/auth.rsassertion failed: key_idx <= 3\00\00\00 \03\10\00\0b\00\00\00\10\00\00\00\05\00\00\00src/read.rs")
  (data (;1;) (i32.const 1049392) "\00\00\00\00\00\00\00\00\00\00\00\00"))
