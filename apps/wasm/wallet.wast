(module
  (type (;0;) (func (param i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (result i64)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i64)))
  (type (;5;) (func (param i32) (result i64)))
  (type (;6;) (func (param i32 i32 i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i64 i32)))
  (type (;8;) (func (param i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32)))
  (type (;10;) (func (param i32)))
  (type (;11;) (func))
  (type (;12;) (func (result i32)))
  (type (;13;) (func (param i64)))
  (type (;14;) (func (param i32 i32) (result i32)))
  (import "env" "storage_write_i32_be" (func $storage_write_i32_be (type 0)))
  (import "env" "storage_read_i32_be" (func $storage_read_i32_be (type 1)))
  (import "env" "host_current_balance" (func $host_current_balance (type 2)))
  (import "env" "reg_push" (func $reg_push (type 3)))
  (import "env" "storage_read_i64_be" (func $storage_read_i64_be (type 4)))
  (import "env" "host_ctx_read_i64_be" (func $host_ctx_read_i64_be (type 5)))
  (import "env" "buffer_copy_to_storage" (func $buffer_copy_to_storage (type 6)))
  (import "env" "storage_write_i64_be" (func $storage_write_i64_be (type 7)))
  (import "env" "host_ctx_read_i32_be" (func $host_ctx_read_i32_be (type 8)))
  (import "env" "host_ctx_read_into_reg" (func $host_ctx_read_into_reg (type 9)))
  (import "env" "storage_write_from_reg" (func $storage_write_from_reg (type 6)))
  (import "env" "reg_pop" (func $reg_pop (type 3)))
  (import "env" "reg_set_i32_be" (func $reg_set_i32_be (type 9)))
  (import "env" "reg_cmp" (func $reg_cmp (type 1)))
  (import "env" "storage_read_to_reg" (func $storage_read_to_reg (type 6)))
  (func $init (type 0) (param i32 i32 i32 i32)
    local.get 1
    call $write_pub_keys
    call $write_first_layer
    local.get 2
    call $write_period_sec
    i32.const 0
    call $write_liquidated
    i32.const 0
    i32.const 156
    local.get 1
    i32.const 4
    call $storage_write_i32_be
    local.get 1
    local.get 2
    call $write_layer_liquidation
    local.get 3
    call $write_lockup_time)
  (func $write_pub_keys (type 10) (param i32)
    (local i32 i32 i32)
    i32.const 1
    local.set 1
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call $bcmp
    select
    i32.const 1
    call $storage_read_i32_be
    local.set 2
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 0
        i32.const 96
        i32.const 60
        local.get 3
        select
        call $buffer_copy_to_storage
        i32.const 0
        i32.const 162
        i32.const 128
        i32.const 1049460
        i32.const 1049471
        i32.const 11
        call $bcmp
        select
        i32.const 1
        i32.const 1
        call $storage_write_i32_be
        br 1 (;@1;)
      end
      i32.const 0
      i32.const 0
      i32.const 0
      i32.const 0
      i32.const 32
      i32.const 20
      local.get 3
      select
      call $buffer_copy_to_storage
      i32.const 0
      local.set 1
    end
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call $bcmp
    select
    local.get 1
    i32.const 1
    call $storage_write_i32_be)
  (func $write_first_layer (type 11)
    i32.const 0
    i32.const 128
    i32.const 2
    call $host_ctx_read_i64_be
    i32.const 8
    call $storage_write_i64_be
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $write_period_sec (type 10) (param i32)
    i32.const 0
    i32.const 144
    local.get 0
    i32.const 4
    call $storage_write_i32_be)
  (func $write_liquidated (type 10) (param i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1049502
        i32.const 1049482
        i32.const 10
        call $bcmp
        br_if 0 (;@2;)
        i32.const 144
        local.set 1
        br 1 (;@1;)
      end
      i32.const 152
      i32.const 148
      i32.const 1049502
      i32.const 1049492
      i32.const 10
      call $bcmp
      select
      local.set 1
    end
    i32.const 0
    local.get 1
    local.get 0
    i32.const 4
    call $storage_write_i32_be)
  (func $write_layer_liquidation (type 3) (param i32 i32)
    (local i32 i32)
    i32.const 2
    call $host_ctx_read_i64_be
    drop
    block  ;; label = @1
      block  ;; label = @2
        i32.const 3
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
        i32.const 1048811
        i32.const 49
        i32.const 1048860
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1048880
      i32.const 57
      i32.const 1048940
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
        i32.const 1048975
        i32.const 43
        i32.const 1049020
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1048880
      i32.const 57
      i32.const 1049036
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    block  ;; label = @1
      local.get 1
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1049200
      i32.const 37
      i32.const 1049240
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    i32.const 0
    i32.const 160
    local.get 1
    i32.const 2
    call $storage_write_i32_be)
  (func $write_lockup_time (type 10) (param i32)
    i32.const 0
    i32.const 148
    i32.const 144
    i32.const 1049492
    i32.const 1049482
    i32.const 10
    call $bcmp
    select
    local.get 0
    i32.const 4
    call $storage_write_i32_be)
  (func $get_liquidated (type 12) (result i32)
    call $refresh_liquidation
    call $read_liquidated)
  (func $refresh_liquidation (type 11)
    (local i32 i64 i64)
    i32.const 0
    i32.const 160
    i32.const 2
    call $storage_read_i32_be
    local.set 0
    i32.const 0
    i32.const 136
    i32.const 8
    call $storage_read_i64_be
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 2
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
          call $read_liquidated
          drop
          i32.const 0
          i32.const 156
          i32.const 4
          call $storage_read_i32_be
          local.get 1
          i32.wrap_i64
          i32.ge_u
          br_if 2 (;@1;)
          i32.const 1048656
          i32.const 39
          i32.const 1048696
          call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
          unreachable
        end
        i32.const 1049052
        i32.const 45
        i32.const 1049100
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1049116
      i32.const 40
      i32.const 1049156
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $read_liquidated (type 12) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1049502
        i32.const 1049482
        i32.const 10
        call $bcmp
        br_if 0 (;@2;)
        i32.const 144
        local.set 0
        br 1 (;@1;)
      end
      i32.const 152
      i32.const 148
      i32.const 1049502
      i32.const 1049492
      i32.const 10
      call $bcmp
      select
      local.set 0
    end
    i32.const 0
    local.get 0
    i32.const 4
    call $storage_read_i32_be)
  (func $get_unliquidated (type 12) (result i32)
    call $refresh_liquidation
    i32.const 0
    i32.const 156
    i32.const 4
    call $storage_read_i32_be)
  (func $transfer (type 13) (param i64)
    call $pub_key_auth
    local.get 0
    call $do_transfer)
  (func $pub_key_auth (type 11)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call $bcmp
      select
      i32.const 1
      call $storage_read_i32_be
      br_if 0 (;@1;)
      i32.const 256
      i32.const 160
      i32.const 1049552
      i32.const 1049559
      i32.const 7
      call $bcmp
      select
      local.tee 0
      i32.const 0
      call $reg_push
      local.get 0
      i32.const 1
      call $reg_push
      i32.const 1
      local.get 0
      i32.const 0
      call $host_ctx_read_into_reg
      i32.const 1049512
      i32.const 40
      i32.const 1049312
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    i32.const 1049256
    i32.const 40
    i32.const 1049296
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $do_transfer (type 13) (param i64)
    call $refresh_liquidation
    block  ;; label = @1
      call $host_current_balance
      local.get 0
      i64.ge_u
      br_if 0 (;@1;)
      i32.const 1048576
      i32.const 35
      i32.const 1048624
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    i32.const 160
    i32.const 0
    call $reg_push
    i32.const 1049512
    i32.const 40
    i32.const 1048640
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $transfer_prepare (type 11)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call $bcmp
      select
      i32.const 1
      call $storage_read_i32_be
      br_if 0 (;@1;)
      i32.const 1049351
      i32.const 31
      i32.const 1049384
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    call $multisig_any_key_auth
    call $write_pending_pub_key)
  (func $_ZN4core9panicking5panic17hf5b00f7a6b23c252E (type 9) (param i32 i32 i32)
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
    i32.const 1048772
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
  (func $multisig_any_key_auth (type 11)
    (local i32 i32 i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    local.tee 0
    i32.const 0
    call $reg_push
    local.get 0
    i32.const 1
    call $reg_push
    i32.const 1
    local.get 0
    i32.const 0
    call $host_ctx_read_into_reg
    i32.const 0
    local.set 1
    block  ;; label = @1
      loop  ;; label = @2
        local.get 1
        i32.const 32
        i32.add
        local.tee 2
        i32.const 128
        i32.eq
        br_if 1 (;@1;)
        i32.const 0
        local.get 1
        local.get 0
        i32.const 1
        i32.const 32
        i32.const 20
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call $bcmp
        select
        call $storage_read_to_reg
        local.get 2
        local.set 1
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call $bcmp
        select
        i32.const 0
        i32.const 1
        call $reg_cmp
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 1
      call $reg_pop
      local.get 0
      i32.const 0
      call $reg_pop
      return
    end
    i32.const 1049400
    i32.const 11
    i32.const 1049412
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $write_pending_pub_key (type 11)
    (local i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    local.tee 0
    i32.const 0
    call $reg_push
    i32.const 1
    local.get 0
    i32.const 0
    call $host_ctx_read_into_reg
    local.get 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    call $storage_write_from_reg
    local.get 0
    i32.const 0
    call $reg_pop)
  (func $transfer_apporove (type 13) (param i64)
    call $multisig_complete
    local.get 0
    call $do_transfer)
  (func $multisig_complete (type 11)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.const 162
        i32.const 128
        i32.const 1049460
        i32.const 1049471
        i32.const 11
        call $bcmp
        select
        i32.const 1
        call $storage_read_i32_be
        i32.eqz
        br_if 0 (;@2;)
        call $multisig_any_key_auth
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call $bcmp
        select
        local.tee 0
        i32.const 0
        call $reg_push
        local.get 0
        i32.const 1
        call $reg_push
        i32.const 1
        local.get 0
        i32.const 0
        call $host_ctx_read_into_reg
        i32.const 0
        i32.const 96
        local.get 0
        i32.const 1
        i32.const 32
        i32.const 20
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call $bcmp
        select
        call $storage_read_to_reg
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call $bcmp
        select
        i32.const 0
        i32.const 1
        call $reg_cmp
        br_if 1 (;@1;)
        i32.const 1049328
        i32.const 12
        i32.const 1049444
        call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
        unreachable
      end
      i32.const 1049351
      i32.const 31
      i32.const 1049428
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    call $reset_pending_pub_key
    local.get 0
    i32.const 1
    call $reg_pop
    local.get 0
    i32.const 0
    call $reg_pop)
  (func $_ZN4core9panicking9panic_fmt17hdc4684f569df1bbeE (type 3) (param i32 i32)
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
    i32.const 1048712
    i32.store offset=4
    local.get 2
    i32.const 1048772
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 10) (param i32)
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h981c39a2ec950d30E
    drop
    call $_ZN3std9panicking20rust_panic_with_hook17h46f5759f06fb4468E
    unreachable)
  (func $_ZN4core3ptr13drop_in_place17h048b692ed670d872E (type 10) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc507c5612d4a7c4aE (type 5) (param i32) (result i64)
    i64.const -2101423557763808909)
  (func $_ZN3std9panicking20rust_panic_with_hook17h46f5759f06fb4468E (type 11)
    (local i32 i32)
    i32.const 1
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1049568
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          i64.const 4294967297
          i64.store offset=1049568
          br 1 (;@2;)
        end
        i32.const 0
        i32.const 0
        i32.load offset=1049572
        i32.const 1
        i32.add
        local.tee 0
        i32.store offset=1049572
        local.get 0
        i32.const 2
        i32.gt_u
        br_if 1 (;@1;)
      end
      i32.const 0
      i32.load offset=1049576
      local.tee 1
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.store offset=1049576
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
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h981c39a2ec950d30E (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048728
      i32.const 43
      i32.const 1048772
      call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
      unreachable
    end
    local.get 0)
  (func $write_last_run_layer (type 13) (param i64)
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $reset_pending_pub_key (type 11)
    (local i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    local.tee 0
    i32.const 0
    call $reg_push
    local.get 0
    i32.const 0
    i32.const 0
    call $reg_set_i32_be
    local.get 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    call $storage_write_from_reg
    local.get 0
    i32.const 0
    call $reg_pop)
  (func $write_unliquidated (type 10) (param i32)
    i32.const 0
    i32.const 156
    local.get 0
    i32.const 4
    call $storage_write_i32_be)
  (func $is_multisig (type 12) (result i32)
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call $bcmp
    select
    i32.const 1
    call $storage_read_i32_be
    i32.const 0
    i32.ne)
  (func $pub_key_cmp (type 14) (param i32 i32) (result i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    local.get 0
    local.get 1
    call $reg_cmp)
  (func $multisig_start (type 11)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call $bcmp
      select
      i32.const 1
      call $storage_read_i32_be
      i32.eqz
      br_if 0 (;@1;)
      call $multisig_any_key_auth
      call $write_pending_pub_key
      return
    end
    i32.const 1049351
    i32.const 31
    i32.const 1049384
    call $_ZN4core9panicking5panic17hf5b00f7a6b23c252E
    unreachable)
  (func $copy_host_pub_key_to_reg (type 3) (param i32 i32)
    i32.const 1
    local.get 0
    local.get 1
    call $host_ctx_read_into_reg)
  (func $read_is_multisig (type 12) (result i32)
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call $bcmp
    select
    i32.const 1
    call $storage_read_i32_be)
  (func $read_pending_pub_key (type 3) (param i32 i32)
    i32.const 0
    i32.const 96
    local.get 0
    local.get 1
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    call $storage_read_to_reg)
  (func $read_pub_key (type 9) (param i32 i32 i32)
    i32.const 0
    local.get 0
    local.get 1
    local.get 2
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call $bcmp
    select
    call $storage_read_to_reg)
  (func $read_current_layer (type 2) (result i64)
    i32.const 2
    call $host_ctx_read_i64_be)
  (func $read_first_layer (type 2) (result i64)
    i32.const 0
    i32.const 128
    i32.const 8
    call $storage_read_i64_be)
  (func $read_last_run_layer (type 2) (result i64)
    i32.const 0
    i32.const 136
    i32.const 8
    call $storage_read_i64_be)
  (func $read_unliquidated (type 12) (result i32)
    i32.const 0
    i32.const 156
    i32.const 4
    call $storage_read_i32_be)
  (func $read_layer_liquidation (type 12) (result i32)
    i32.const 0
    i32.const 160
    i32.const 2
    call $storage_read_i32_be)
  (func $read_period_sec (type 12) (result i32)
    i32.const 0
    i32.const 144
    i32.const 4
    call $storage_read_i32_be)
  (func $read_lockup_time_sec (type 12) (result i32)
    i32.const 0
    i32.const 148
    i32.const 144
    i32.const 1049492
    i32.const 1049482
    i32.const 10
    call $bcmp
    select
    i32.const 4
    call $storage_read_i32_be)
  (func $bcmp (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        loop  ;; label = @3
          local.get 0
          i32.load8_u
          local.tee 4
          local.get 1
          i32.load8_u
          local.tee 5
          i32.ne
          br_if 1 (;@2;)
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049580))
  (global (;2;) i32 (i32.const 1049580))
  (export "memory" (memory 0))
  (export "init" (func $init))
  (export "write_pub_keys" (func $write_pub_keys))
  (export "write_first_layer" (func $write_first_layer))
  (export "write_period_sec" (func $write_period_sec))
  (export "write_liquidated" (func $write_liquidated))
  (export "write_layer_liquidation" (func $write_layer_liquidation))
  (export "write_lockup_time" (func $write_lockup_time))
  (export "get_liquidated" (func $get_liquidated))
  (export "refresh_liquidation" (func $refresh_liquidation))
  (export "read_liquidated" (func $read_liquidated))
  (export "get_unliquidated" (func $get_unliquidated))
  (export "transfer" (func $transfer))
  (export "pub_key_auth" (func $pub_key_auth))
  (export "do_transfer" (func $do_transfer))
  (export "transfer_prepare" (func $transfer_prepare))
  (export "multisig_any_key_auth" (func $multisig_any_key_auth))
  (export "write_pending_pub_key" (func $write_pending_pub_key))
  (export "transfer_apporove" (func $transfer_apporove))
  (export "multisig_complete" (func $multisig_complete))
  (export "write_last_run_layer" (func $write_last_run_layer))
  (export "reset_pending_pub_key" (func $reset_pending_pub_key))
  (export "write_unliquidated" (func $write_unliquidated))
  (export "is_multisig" (func $is_multisig))
  (export "pub_key_cmp" (func $pub_key_cmp))
  (export "multisig_start" (func $multisig_start))
  (export "copy_host_pub_key_to_reg" (func $copy_host_pub_key_to_reg))
  (export "read_is_multisig" (func $read_is_multisig))
  (export "read_pending_pub_key" (func $read_pending_pub_key))
  (export "read_pub_key" (func $read_pub_key))
  (export "read_current_layer" (func $read_current_layer))
  (export "read_first_layer" (func $read_first_layer))
  (export "read_last_run_layer" (func $read_last_run_layer))
  (export "read_unliquidated" (func $read_unliquidated))
  (export "read_layer_liquidation" (func $read_layer_liquidation))
  (export "read_period_sec" (func $read_period_sec))
  (export "read_lockup_time_sec" (func $read_lockup_time_sec))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) $_ZN4core3ptr13drop_in_place17h048b692ed670d872E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hc507c5612d4a7c4aE)
  (data (;0;) (i32.const 1048576) "assertion failed: balance >= amountsrc/lib.rs\00\00\00#\00\10\00\0a\00\00\00m\00\00\00\09\00\00\00#\00\10\00\0a\00\00\00r\00\00\000\00\00\00assertion failed: unliquidated >= delta\00#\00\10\00\0a\00\00\00\85\00\00\00\05\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00\d4\00\10\00\17\00\00\00y\01\00\00\0f\00\00\00src/libstd/panicking.rsassertion failed: time_interval % layer_time == 0|\01\10\00\13\00\00\00!\00\00\00\05\00\00\00\00\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00|\01\10\00\13\00\00\00!\00\00\00\0d\00\00\00src/computations.rsassertion failed: amount % layer_count == 0\00\00|\01\10\00\13\00\00\00,\00\00\00\05\00\00\00|\01\10\00\13\00\00\00,\00\00\00\0d\00\00\00assertion failed: current_layer >= last_layer\00\00\00|\01\10\00\13\00\00\008\00\00\00\05\00\00\00assertion failed: delta <= 0xFF_FF_FF_FF|\01\10\00\13\00\00\00<\00\00\00\05\00\00\00src/write.rsT\02\10\00\0c\00\00\00\a2\00\00\00(\00\00\00assertion failed: layer_liq <= 0xFFFF\00\00\00T\02\10\00\0c\00\00\00N\00\00\00\09\00\00\00assertion failed: is_multisig() == false\fc\02\10\00\0b\00\00\00\11\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00\1a\00\00\00\16\00\00\00auth failed!src/auth.rsassertion failed: is_multisig()\00\00\fc\02\10\00\0b\00\00\00'\00\00\00\05\00\00\00auth failed\00\fc\02\10\00\0b\00\00\00h\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00/\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00A\00\00\00\0d\00\00\00is_multisigfirst_layerperiod_seclockup_secliquidatedinternal error: entered unreachable codepub_keyaddress")
  (data (;1;) (i32.const 1049568) "\00\00\00\00\00\00\00\00\00\00\00\00"))
