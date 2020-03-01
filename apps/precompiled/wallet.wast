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
  (import "env" "storage_write_i32_be" (func (;0;) (type 0)))
  (import "env" "storage_read_i32_be" (func (;1;) (type 1)))
  (import "env" "host_current_balance" (func (;2;) (type 2)))
  (import "env" "reg_push" (func (;3;) (type 3)))
  (import "env" "storage_read_i64_be" (func (;4;) (type 4)))
  (import "env" "host_ctx_read_i64_be" (func (;5;) (type 5)))
  (import "env" "buffer_copy_to_storage" (func (;6;) (type 6)))
  (import "env" "storage_write_i64_be" (func (;7;) (type 7)))
  (import "env" "host_ctx_read_i32_be" (func (;8;) (type 8)))
  (import "env" "host_ctx_read_into_reg" (func (;9;) (type 9)))
  (import "env" "storage_write_from_reg" (func (;10;) (type 6)))
  (import "env" "reg_pop" (func (;11;) (type 3)))
  (import "env" "reg_set_i32_be" (func (;12;) (type 9)))
  (import "env" "reg_cmp" (func (;13;) (type 1)))
  (import "env" "storage_read_to_reg" (func (;14;) (type 6)))
  (func (;15;) (type 0) (param i32 i32 i32 i32)
    local.get 1
    call 16
    call 17
    local.get 2
    call 18
    i32.const 0
    call 19
    i32.const 0
    i32.const 156
    local.get 1
    i32.const 4
    call 0
    local.get 1
    local.get 2
    call 20
    local.get 3
    call 21)
  (func (;16;) (type 10) (param i32)
    (local i32 i32 i32)
    i32.const 1
    local.set 1
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call 59
    select
    i32.const 1
    call 1
    local.set 2
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
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
        call 6
        i32.const 0
        i32.const 162
        i32.const 128
        i32.const 1049460
        i32.const 1049471
        i32.const 11
        call 59
        select
        i32.const 1
        i32.const 1
        call 0
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
      call 6
      i32.const 0
      local.set 1
    end
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call 59
    select
    local.get 1
    i32.const 1
    call 0)
  (func (;17;) (type 11)
    i32.const 0
    i32.const 128
    i32.const 2
    call 5
    i32.const 8
    call 7
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call 30
    unreachable)
  (func (;18;) (type 10) (param i32)
    i32.const 0
    i32.const 144
    local.get 0
    i32.const 4
    call 0)
  (func (;19;) (type 10) (param i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1049502
        i32.const 1049482
        i32.const 10
        call 59
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
      call 59
      select
      local.set 1
    end
    i32.const 0
    local.get 1
    local.get 0
    i32.const 4
    call 0)
  (func (;20;) (type 3) (param i32 i32)
    (local i32 i32)
    i32.const 2
    call 5
    drop
    block  ;; label = @1
      block  ;; label = @2
        i32.const 3
        call 8
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
        call 30
        unreachable
      end
      i32.const 1048880
      i32.const 57
      i32.const 1048940
      call 30
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
        call 30
        unreachable
      end
      i32.const 1048880
      i32.const 57
      i32.const 1049036
      call 30
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
      call 30
      unreachable
    end
    i32.const 0
    i32.const 160
    local.get 1
    i32.const 2
    call 0)
  (func (;21;) (type 10) (param i32)
    i32.const 0
    i32.const 148
    i32.const 144
    i32.const 1049492
    i32.const 1049482
    i32.const 10
    call 59
    select
    local.get 0
    i32.const 4
    call 0)
  (func (;22;) (type 12) (result i32)
    call 23
    call 24)
  (func (;23;) (type 11)
    (local i32 i64 i64)
    i32.const 0
    i32.const 160
    i32.const 2
    call 1
    local.set 0
    i32.const 0
    i32.const 136
    i32.const 8
    call 4
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 2
          call 5
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
          call 24
          drop
          i32.const 0
          i32.const 156
          i32.const 4
          call 1
          local.get 1
          i32.wrap_i64
          i32.ge_u
          br_if 2 (;@1;)
          i32.const 1048656
          i32.const 39
          i32.const 1048696
          call 30
          unreachable
        end
        i32.const 1049052
        i32.const 45
        i32.const 1049100
        call 30
        unreachable
      end
      i32.const 1049116
      i32.const 40
      i32.const 1049156
      call 30
      unreachable
    end
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call 30
    unreachable)
  (func (;24;) (type 12) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1049502
        i32.const 1049482
        i32.const 10
        call 59
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
      call 59
      select
      local.set 0
    end
    i32.const 0
    local.get 0
    i32.const 4
    call 1)
  (func (;25;) (type 12) (result i32)
    call 23
    i32.const 0
    i32.const 156
    i32.const 4
    call 1)
  (func (;26;) (type 13) (param i64)
    call 27
    local.get 0
    call 28)
  (func (;27;) (type 11)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call 59
      select
      i32.const 1
      call 1
      br_if 0 (;@1;)
      i32.const 256
      i32.const 160
      i32.const 1049552
      i32.const 1049559
      i32.const 7
      call 59
      select
      local.tee 0
      i32.const 0
      call 3
      local.get 0
      i32.const 1
      call 3
      i32.const 1
      local.get 0
      i32.const 0
      call 9
      i32.const 1049512
      i32.const 40
      i32.const 1049312
      call 30
      unreachable
    end
    i32.const 1049256
    i32.const 40
    i32.const 1049296
    call 30
    unreachable)
  (func (;28;) (type 13) (param i64)
    call 23
    block  ;; label = @1
      call 2
      local.get 0
      i64.ge_u
      br_if 0 (;@1;)
      i32.const 1048576
      i32.const 35
      i32.const 1048624
      call 30
      unreachable
    end
    i32.const 160
    i32.const 0
    call 3
    i32.const 1049512
    i32.const 40
    i32.const 1048640
    call 30
    unreachable)
  (func (;29;) (type 11)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call 59
      select
      i32.const 1
      call 1
      br_if 0 (;@1;)
      i32.const 1049351
      i32.const 31
      i32.const 1049384
      call 30
      unreachable
    end
    call 31
    call 32)
  (func (;30;) (type 9) (param i32 i32 i32)
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
    call 35
    unreachable)
  (func (;31;) (type 11)
    (local i32 i32 i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    local.tee 0
    i32.const 0
    call 3
    local.get 0
    i32.const 1
    call 3
    i32.const 1
    local.get 0
    i32.const 0
    call 9
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
        call 59
        select
        call 14
        local.get 2
        local.set 1
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call 59
        select
        i32.const 0
        i32.const 1
        call 13
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 1
      call 11
      local.get 0
      i32.const 0
      call 11
      return
    end
    i32.const 1049400
    i32.const 11
    i32.const 1049412
    call 30
    unreachable)
  (func (;32;) (type 11)
    (local i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    local.tee 0
    i32.const 0
    call 3
    i32.const 1
    local.get 0
    i32.const 0
    call 9
    local.get 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    call 10
    local.get 0
    i32.const 0
    call 11)
  (func (;33;) (type 13) (param i64)
    call 34
    local.get 0
    call 28)
  (func (;34;) (type 11)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.const 162
        i32.const 128
        i32.const 1049460
        i32.const 1049471
        i32.const 11
        call 59
        select
        i32.const 1
        call 1
        i32.eqz
        br_if 0 (;@2;)
        call 31
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call 59
        select
        local.tee 0
        i32.const 0
        call 3
        local.get 0
        i32.const 1
        call 3
        i32.const 1
        local.get 0
        i32.const 0
        call 9
        i32.const 0
        i32.const 96
        local.get 0
        i32.const 1
        i32.const 32
        i32.const 20
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call 59
        select
        call 14
        i32.const 256
        i32.const 160
        i32.const 1049552
        i32.const 1049559
        i32.const 7
        call 59
        select
        i32.const 0
        i32.const 1
        call 13
        br_if 1 (;@1;)
        i32.const 1049328
        i32.const 12
        i32.const 1049444
        call 30
        unreachable
      end
      i32.const 1049351
      i32.const 31
      i32.const 1049428
      call 30
      unreachable
    end
    call 43
    local.get 0
    i32.const 1
    call 11
    local.get 0
    i32.const 0
    call 11)
  (func (;35;) (type 3) (param i32 i32)
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
    call 36
    unreachable)
  (func (;36;) (type 10) (param i32)
    local.get 0
    i32.load offset=8
    call 41
    drop
    call 39
    unreachable)
  (func (;37;) (type 10) (param i32))
  (func (;38;) (type 5) (param i32) (result i64)
    i64.const -2101423557763808909)
  (func (;39;) (type 11)
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
      call 40
      unreachable
    end
    unreachable
    unreachable)
  (func (;40;) (type 11)
    unreachable
    unreachable)
  (func (;41;) (type 8) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048728
      i32.const 43
      i32.const 1048772
      call 30
      unreachable
    end
    local.get 0)
  (func (;42;) (type 13) (param i64)
    i32.const 1049512
    i32.const 40
    i32.const 1049184
    call 30
    unreachable)
  (func (;43;) (type 11)
    (local i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    local.tee 0
    i32.const 0
    call 3
    local.get 0
    i32.const 0
    i32.const 0
    call 12
    local.get 0
    i32.const 0
    i32.const 0
    i32.const 96
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    call 10
    local.get 0
    i32.const 0
    call 11)
  (func (;44;) (type 10) (param i32)
    i32.const 0
    i32.const 156
    local.get 0
    i32.const 4
    call 0)
  (func (;45;) (type 12) (result i32)
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call 59
    select
    i32.const 1
    call 1
    i32.const 0
    i32.ne)
  (func (;46;) (type 14) (param i32 i32) (result i32)
    i32.const 256
    i32.const 160
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    local.get 0
    local.get 1
    call 13)
  (func (;47;) (type 11)
    block  ;; label = @1
      i32.const 0
      i32.const 162
      i32.const 128
      i32.const 1049460
      i32.const 1049471
      i32.const 11
      call 59
      select
      i32.const 1
      call 1
      i32.eqz
      br_if 0 (;@1;)
      call 31
      call 32
      return
    end
    i32.const 1049351
    i32.const 31
    i32.const 1049384
    call 30
    unreachable)
  (func (;48;) (type 3) (param i32 i32)
    i32.const 1
    local.get 0
    local.get 1
    call 9)
  (func (;49;) (type 12) (result i32)
    i32.const 0
    i32.const 162
    i32.const 128
    i32.const 1049460
    i32.const 1049471
    i32.const 11
    call 59
    select
    i32.const 1
    call 1)
  (func (;50;) (type 3) (param i32 i32)
    i32.const 0
    i32.const 96
    local.get 0
    local.get 1
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    call 14)
  (func (;51;) (type 9) (param i32 i32 i32)
    i32.const 0
    local.get 0
    local.get 1
    local.get 2
    i32.const 32
    i32.const 20
    i32.const 1049552
    i32.const 1049559
    i32.const 7
    call 59
    select
    call 14)
  (func (;52;) (type 2) (result i64)
    i32.const 2
    call 5)
  (func (;53;) (type 2) (result i64)
    i32.const 0
    i32.const 128
    i32.const 8
    call 4)
  (func (;54;) (type 2) (result i64)
    i32.const 0
    i32.const 136
    i32.const 8
    call 4)
  (func (;55;) (type 12) (result i32)
    i32.const 0
    i32.const 156
    i32.const 4
    call 1)
  (func (;56;) (type 12) (result i32)
    i32.const 0
    i32.const 160
    i32.const 2
    call 1)
  (func (;57;) (type 12) (result i32)
    i32.const 0
    i32.const 144
    i32.const 4
    call 1)
  (func (;58;) (type 12) (result i32)
    i32.const 0
    i32.const 148
    i32.const 144
    i32.const 1049492
    i32.const 1049482
    i32.const 10
    call 59
    select
    i32.const 4
    call 1)
  (func (;59;) (type 1) (param i32 i32 i32) (result i32)
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
  (export "init" (func 15))
  (export "write_pub_keys" (func 16))
  (export "write_first_layer" (func 17))
  (export "write_period_sec" (func 18))
  (export "write_liquidated" (func 19))
  (export "write_layer_liquidation" (func 20))
  (export "write_lockup_time" (func 21))
  (export "get_liquidated" (func 22))
  (export "refresh_liquidation" (func 23))
  (export "read_liquidated" (func 24))
  (export "get_unliquidated" (func 25))
  (export "transfer" (func 26))
  (export "pub_key_auth" (func 27))
  (export "do_transfer" (func 28))
  (export "transfer_prepare" (func 29))
  (export "multisig_any_key_auth" (func 31))
  (export "write_pending_pub_key" (func 32))
  (export "transfer_apporove" (func 33))
  (export "multisig_complete" (func 34))
  (export "write_last_run_layer" (func 42))
  (export "reset_pending_pub_key" (func 43))
  (export "write_unliquidated" (func 44))
  (export "is_multisig" (func 45))
  (export "pub_key_cmp" (func 46))
  (export "multisig_start" (func 47))
  (export "copy_host_pub_key_to_reg" (func 48))
  (export "read_is_multisig" (func 49))
  (export "read_pending_pub_key" (func 50))
  (export "read_pub_key" (func 51))
  (export "read_current_layer" (func 52))
  (export "read_first_layer" (func 53))
  (export "read_last_run_layer" (func 54))
  (export "read_unliquidated" (func 55))
  (export "read_layer_liquidation" (func 56))
  (export "read_period_sec" (func 57))
  (export "read_lockup_time_sec" (func 58))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) 37 38)
  (data (;0;) (i32.const 1048576) "assertion failed: balance >= amountsrc/lib.rs\00\00\00#\00\10\00\0a\00\00\00m\00\00\00\09\00\00\00#\00\10\00\0a\00\00\00r\00\00\000\00\00\00assertion failed: unliquidated >= delta\00#\00\10\00\0a\00\00\00\85\00\00\00\05\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00\d4\00\10\00\17\00\00\00y\01\00\00\0f\00\00\00src/libstd/panicking.rsassertion failed: time_interval % layer_time == 0|\01\10\00\13\00\00\00!\00\00\00\05\00\00\00\00\00\00\00attempt to calculate the remainder with a divisor of zero\00\00\00|\01\10\00\13\00\00\00!\00\00\00\0d\00\00\00src/computations.rsassertion failed: amount % layer_count == 0\00\00|\01\10\00\13\00\00\00,\00\00\00\05\00\00\00|\01\10\00\13\00\00\00,\00\00\00\0d\00\00\00assertion failed: current_layer >= last_layer\00\00\00|\01\10\00\13\00\00\008\00\00\00\05\00\00\00assertion failed: delta <= 0xFF_FF_FF_FF|\01\10\00\13\00\00\00<\00\00\00\05\00\00\00src/write.rsT\02\10\00\0c\00\00\00\a2\00\00\00(\00\00\00assertion failed: layer_liq <= 0xFFFF\00\00\00T\02\10\00\0c\00\00\00N\00\00\00\09\00\00\00assertion failed: is_multisig() == false\fc\02\10\00\0b\00\00\00\11\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00\1a\00\00\00\16\00\00\00auth failed!src/auth.rsassertion failed: is_multisig()\00\00\fc\02\10\00\0b\00\00\00'\00\00\00\05\00\00\00auth failed\00\fc\02\10\00\0b\00\00\00h\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00/\00\00\00\05\00\00\00\fc\02\10\00\0b\00\00\00A\00\00\00\0d\00\00\00is_multisigfirst_layerperiod_seclockup_secliquidatedinternal error: entered unreachable codepub_keyaddress")
  (data (;1;) (i32.const 1049568) "\00\00\00\00\00\00\00\00\00\00\00\00"))
