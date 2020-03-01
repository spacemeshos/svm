(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (result i64)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32 i32)))
  (type (;4;) (func (param i64 i32 i32)))
  (type (;5;) (func (param i32 i32 i32) (result i64)))
  (type (;6;) (func (param i32) (result i64)))
  (type (;7;) (func (param i32 i32 i64 i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (type (;9;) (func (param i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32)))
  (type (;11;) (func (result i32)))
  (type (;12;) (func))
  (type (;13;) (func (param i64)))
  (type (;14;) (func (param i32)))
  (type (;15;) (func (param i32 i32) (result i32)))
  (import "env" "storage_read_i32_be" (func $storage_read_i32_be (type 0)))
  (import "env" "host_current_balance" (func $host_current_balance (type 1)))
  (import "env" "reg_push" (func $reg_push (type 2)))
  (import "env" "buffer_copy_to_reg" (func $buffer_copy_to_reg (type 3)))
  (import "env" "host_transfer" (func $host_transfer (type 4)))
  (import "env" "reg_pop" (func $reg_pop (type 2)))
  (import "env" "storage_read_i64_be" (func $storage_read_i64_be (type 5)))
  (import "env" "host_ctx_read_i64_be" (func $host_ctx_read_i64_be (type 6)))
  (import "env" "storage_write_i64_be" (func $storage_write_i64_be (type 7)))
  (import "env" "storage_write_i32_be" (func $storage_write_i32_be (type 8)))
  (import "env" "host_ctx_read_i32_be" (func $host_ctx_read_i32_be (type 9)))
  (import "env" "host_ctx_read_into_reg" (func $host_ctx_read_into_reg (type 10)))
  (import "env" "storage_write_from_reg" (func $storage_write_from_reg (type 3)))
  (import "env" "reg_set_i32_be" (func $reg_set_i32_be (type 10)))
  (import "env" "storage_read_to_reg" (func $storage_read_to_reg (type 3)))
  (import "env" "reg_cmp" (func $reg_cmp (type 0)))
  (func $init (type 8) (param i32 i32 i32 i32)
    call $read_is_multisig
    drop
    unreachable)
  (func $read_is_multisig (type 11) (result i32)
    i32.const 1049264
    i32.const 19
    i32.const 1049284
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $get_liquidated (type 11) (result i32)
    call $refresh_liquidation
    i32.const 0
    i32.const 0
    i32.const 4
    call $storage_read_i32_be)
  (func $refresh_liquidation (type 12)
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
          i32.const 1048640
          i32.const 39
          i32.const 1048680
          call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
          unreachable
        end
        i32.const 1049036
        i32.const 45
        i32.const 1049084
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1049100
      i32.const 40
      i32.const 1049140
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
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
  (func $transfer (type 13) (param i64)
    call $read_is_multisig
    drop
    unreachable)
  (func $transfer_prepare (type 12)
    call $read_is_multisig
    drop
    unreachable)
  (func $do_transfer (type 13) (param i64)
    call $refresh_liquidation
    block  ;; label = @1
      call $host_current_balance
      local.get 0
      i64.lt_u
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
      call $host_transfer
      i32.const 160
      i32.const 0
      call $reg_pop
      return
    end
    i32.const 1048576
    i32.const 35
    i32.const 1048624
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $_ZN4core9panicking5panic17hf5b00f7a6b23c252E (type 10) (param i32 i32 i32)
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
    i32.const 1048756
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
    call $_ZN4core9panicking9panic_fmt17hdc4684f569df1bbeE
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17hdc4684f569df1bbeE (type 2) (param i32 i32)
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
    i32.const 1048696
    i32.store offset=4
    local.get 2
    i32.const 1048756
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 14) (param i32)
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h981c39a2ec950d30E
    drop
    call $_ZN3std9panicking20rust_panic_with_hook17h46f5759f06fb4468E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h048b692ed670d872E (type 14) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc507c5612d4a7c4aE (type 6) (param i32) (result i64)
    i64.const -2101423557763808909)
  (func $_ZN3std9panicking20rust_panic_with_hook17h46f5759f06fb4468E (type 12)
    (local i32 i32)
    i32.const 1
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1049360
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i64.const 4294967297
          i64.store offset=1049360
          br 1 (;@2;)
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049364
        i32.const 1
        i32.add
        local.tee 0
        i32.store offset=1049364
        local.get 0
        i32.const 2
        i32.gt_u
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.load offset=1049368
      local.tee 1
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.store offset=1049368
      local.get 0
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 12)
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h981c39a2ec950d30E (type 9) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048712
      i32.const 43
      i32.const 1048756
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    local.get 0)
  (func $write_pub_keys (type 14) (param i32)
    call $read_is_multisig
    drop
    unreachable)
  (func $write_first_layer (type 12)
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
  (func $write_layer_liquidation (type 2) (param i32 i32)
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
        i32.const 1048795
        i32.const 49
        i32.const 1048844
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1048864
      i32.const 57
      i32.const 1048924
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
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
        i32.const 1048959
        i32.const 43
        i32.const 1049004
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1048864
      i32.const 57
      i32.const 1049020
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
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
    i32.const 1049156
    i32.const 37
    i32.const 1049196
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $write_pending_pub_key (type 12)
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
    i32.const 0
    i32.const 32
    call $storage_write_from_reg
    i32.const 256
    i32.const 0
    call $reg_pop)
  (func $reset_pending_pub_key (type 12)
    i32.const 256
    i32.const 0
    call $reg_push
    i32.const 256
    i32.const 0
    i32.const 4
    call $reg_set_i32_be
    i32.const 256
    i32.const 0
    i32.const 0
    i32.const 0
    i32.const 32
    call $storage_write_from_reg
    i32.const 256
    i32.const 0
    call $reg_pop)
  (func $write_liquidated (type 14) (param i32)
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
  (func $is_multisig (type 11) (result i32)
    call $read_is_multisig
    drop
    unreachable)
  (func $multisig_any_key_auth (type 12)
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
        i32.const 0
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
    i32.const 1049235
    i32.const 11
    i32.const 1049248
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $multisig_complete (type 12)
    call $read_is_multisig
    drop
    unreachable)
  (func $pub_key_cmp (type 15) (param i32 i32) (result i32)
    i32.const 256
    local.get 0
    local.get 1
    call $reg_cmp)
  (func $copy_host_pub_key_to_reg (type 2) (param i32 i32)
    i32.const 0
    local.get 0
    local.get 1
    call $host_ctx_read_into_reg)
  (func $read_pending_pub_key (type 2) (param i32 i32)
    i32.const 0
    i32.const 96
    local.get 0
    local.get 1
    i32.const 0
    call $storage_read_to_reg)
  (func $read_current_layer (type 1) (result i64)
    i32.const 0
    call $host_ctx_read_i64_be)
  (func $read_pub_key (type 10) (param i32 i32 i32)
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
      i32.const 0
      call $storage_read_to_reg
      return
    end
    i32.const 1049311
    i32.const 30
    i32.const 1049344
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $read_first_layer (type 1) (result i64)
    i32.const 0
    i32.const 0
    i32.const 8
    call $storage_read_i64_be)
  (func $read_liquidated (type 11) (result i32)
    i32.const 0
    i32.const 0
    i32.const 4
    call $storage_read_i32_be)
  (func $read_layer_liquidation (type 11) (result i32)
    i32.const 0
    i32.const 0
    i32.const 2
    call $storage_read_i32_be)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049372))
  (global (;2;) i32 (i32.const 1049372))
  (export "memory" (memory 0))
  (export "init" (func $init))
  (export "read_is_multisig" (func $read_is_multisig))
  (export "get_liquidated" (func $get_liquidated))
  (export "refresh_liquidation" (func $refresh_liquidation))
  (export "transfer" (func $transfer))
  (export "transfer_prepare" (func $transfer_prepare))
  (export "do_transfer" (func $do_transfer))
  (export "write_pub_keys" (func $write_pub_keys))
  (export "write_first_layer" (func $write_first_layer))
  (export "write_layer_liquidation" (func $write_layer_liquidation))
  (export "write_pending_pub_key" (func $write_pending_pub_key))
  (export "reset_pending_pub_key" (func $reset_pending_pub_key))
  (export "write_liquidated" (func $write_liquidated))
  (export "write_last_run_layer" (func $write_last_run_layer))
  (export "is_multisig" (func $is_multisig))
  (export "multisig_any_key_auth" (func $multisig_any_key_auth))
  (export "multisig_complete" (func $multisig_complete))
  (export "pub_key_cmp" (func $pub_key_cmp))
  (export "copy_host_pub_key_to_reg" (func $copy_host_pub_key_to_reg))
  (export "read_pending_pub_key" (func $read_pending_pub_key))
  (export "read_current_layer" (func $read_current_layer))
  (export "read_pub_key" (func $read_pub_key))
  (export "read_first_layer" (func $read_first_layer))
  (export "read_liquidated" (func $read_liquidated))
  (export "read_layer_liquidation" (func $read_layer_liquidation))
  (export "write_unliquidated" (func $write_liquidated))
  (export "write_period_sec" (func $write_liquidated))
  (export "write_lockup_time" (func $write_liquidated))
  (export "read_last_run_layer" (func $read_first_layer))
  (export "read_unliquidated" (func $read_liquidated))
  (export "read_period_sec" (func $read_first_layer))
  (export "read_lockup_time_sec" (func $read_first_layer))
  (export "get_unliquidated" (func $get_liquidated))
  (export "pub_key_auth" (func $multisig_complete))
  (export "multisig_start" (func $multisig_complete))
  (export "transfer_apporove" (func $transfer))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) $_ZN4core3ptr13drop_in_place17h048b692ed670d872E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc507c5612d4a7c4aE)
  (data (;0;) (i32.const 1048576) "assertion failed: balance >= amountsrc/lib.rs\00\00\00#\00\10\00\0a\00\00\00j\00\00\00\09\00\00\00assertion failed: unliquidated >= delta\00#\00\10\00\0a\00\00\00\82\00\00\00\05\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00\c4\00\10\00\17\00\00\00y\01\00\00\0f\00\00\00src/libstd/panicking.rsassertion failed: time_interval % layer_time == 0l\01\10\00\13\00\00\00!\00\00\00\05\00\00\00\00\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00l\01\10\00\13\00\00\00!\00\00\00\0d\00\00\00src/computations.rsassertion failed: amount % layer_count == 0\00\00l\01\10\00\13\00\00\00,\00\00\00\05\00\00\00l\01\10\00\13\00\00\00,\00\00\00\0d\00\00\00assertion failed: current_layer >= last_layer\00\00\00l\01\10\00\13\00\00\008\00\00\00\05\00\00\00assertion failed: delta <= 0xFF_FF_FF_FFl\01\10\00\13\00\00\00<\00\00\00\05\00\00\00assertion failed: layer_liq <= 0xFFFF\00\00\00|\02\10\00\0c\00\00\00:\00\00\00\09\00\00\00src/write.rssrc/auth.rsauth failed\00\00\88\02\10\00\0b\00\00\00`\00\00\00\05\00\00\00not yet implemented\00\d4\02\10\00\0b\00\00\00\05\00\00\00\05\00\00\00src/read.rsassertion failed: key_idx <= 3\00\00\00\d4\02\10\00\0b\00\00\00\15\00\00\00\05\00\00\00")
  (data (;1;) (i32.const 1049360) "\00\00\00\00\00\00\00\00\00\00\00\00"))
