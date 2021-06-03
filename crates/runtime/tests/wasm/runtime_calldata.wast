(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func))
  (type (;4;) (func (param i32 i32) (result i64)))
  (type (;5;) (func (param i32 i64)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32) (result i64)))
  (import "svm" "svm_static_alloc" (func (;0;) (type 0)))
  (import "svm" "svm_calldata_offset" (func (;1;) (type 1)))
  (import "svm" "svm_calldata_len" (func (;2;) (type 1)))
  (import "svm" "svm_store160" (func (;3;) (type 2)))
  (import "svm" "svm_load160" (func (;4;) (type 2)))
  (import "svm" "svm_set_returndata" (func (;5;) (type 2)))
  (func (;6;) (type 0) (param i32) (result i32)
    local.get 0
    call 0)
  (func (;7;) (type 3))
  (func (;8;) (type 3)
    (local i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 160
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1048576
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048576
    end
    call 1
    local.set 1
    local.get 0
    call 2
    local.tee 2
    i32.store offset=32
    local.get 0
    i32.const 0
    i32.store offset=28
    local.get 0
    local.get 1
    i32.store offset=24
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.const 24
                i32.add
                call 9
                local.tee 1
                i32.const 255
                i32.and
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                i32.const 1
                i32.and
                br_if 5 (;@1;)
                block  ;; label = @7
                  local.get 1
                  i32.const 65280
                  i32.and
                  i32.const 3328
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 24
                  i32.add
                  call 9
                  local.tee 1
                  i32.const 255
                  i32.and
                  i32.const 1
                  i32.eq
                  br_if 2 (;@5;)
                  local.get 1
                  i32.const 1
                  i32.and
                  br_if 6 (;@1;)
                  local.get 1
                  i32.const 65280
                  i32.and
                  i32.const 3328
                  i32.ne
                  br_if 6 (;@1;)
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=28
                    local.get 0
                    i32.load offset=32
                    i32.lt_u
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 1
                    br 6 (;@2;)
                  end
                  block  ;; label = @8
                    local.get 0
                    i32.const 24
                    i32.add
                    call 10
                    local.tee 1
                    i32.const 255
                    i32.and
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 1
                    i32.and
                    br_if 7 (;@1;)
                    local.get 1
                    i32.const 8
                    i32.shr_u
                    i32.const 255
                    i32.and
                    local.tee 1
                    i32.const 24
                    i32.mul
                    call 0
                    local.set 2
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            i32.const -6
                            i32.add
                            br_table 11 (;@1;) 1 (;@11;) 0 (;@12;)
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 1
                                        i32.const -22
                                        i32.add
                                        br_table 1 (;@17;) 8 (;@10;) 0 (;@18;)
                                      end
                                      block  ;; label = @18
                                        local.get 1
                                        i32.const -38
                                        i32.add
                                        br_table 2 (;@16;) 9 (;@9;) 0 (;@18;)
                                      end
                                      local.get 1
                                      i32.const 54
                                      i32.eq
                                      br_if 2 (;@15;)
                                      local.get 1
                                      i32.const 70
                                      i32.eq
                                      br_if 3 (;@14;)
                                      local.get 1
                                      i32.const 86
                                      i32.eq
                                      br_if 4 (;@13;)
                                      local.get 1
                                      i32.const 102
                                      i32.eq
                                      br_if 5 (;@12;)
                                      local.get 1
                                      i32.const 118
                                      i32.ne
                                      br_if 16 (;@1;)
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.eqz
                                      br_if 16 (;@1;)
                                      local.get 4
                                      i64.load
                                      local.set 6
                                      local.get 0
                                      i64.load offset=72
                                      local.set 7
                                      local.get 2
                                      i32.const 16
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 8
                                      i32.add
                                      local.get 6
                                      i64.store
                                      local.get 2
                                      local.get 7
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 1
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=24
                                      local.get 2
                                      i32.const 40
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 32
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 2
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=48
                                      local.get 2
                                      i32.const 64
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 56
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 3
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=72
                                      local.get 2
                                      i32.const 88
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 80
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 4
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=96
                                      local.get 2
                                      i32.const 112
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 104
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 5
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=120
                                      local.get 2
                                      i32.const 136
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 128
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 0
                                      i32.const 96
                                      i32.add
                                      local.get 0
                                      i32.const 24
                                      i32.add
                                      call 11
                                      local.get 0
                                      i32.load8_u offset=96
                                      i32.const 1
                                      i32.eq
                                      br_if 13 (;@4;)
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 0
                                      i32.const 113
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 128
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 0
                                      i32.const 105
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 151
                                      i32.add
                                      local.tee 3
                                      local.get 0
                                      i32.const 120
                                      i32.add
                                      i64.load align=1
                                      i64.store align=1
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 4
                                      local.get 0
                                      i32.const 143
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      i32.const 72
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 5
                                      local.get 3
                                      i64.load align=1
                                      i64.store
                                      local.get 0
                                      local.get 0
                                      i64.load offset=97 align=1
                                      i64.store offset=128
                                      local.get 0
                                      local.get 0
                                      i64.load offset=135 align=1
                                      i64.store offset=72
                                      local.get 1
                                      i32.const 6
                                      i32.le_u
                                      br_if 16 (;@1;)
                                      local.get 2
                                      local.get 0
                                      i64.load offset=72
                                      i64.store offset=144
                                      local.get 2
                                      i32.const 160
                                      i32.add
                                      local.get 5
                                      i64.load
                                      i64.store
                                      local.get 2
                                      i32.const 152
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      br 16 (;@1;)
                                    end
                                    local.get 0
                                    i32.const 96
                                    i32.add
                                    local.get 0
                                    i32.const 24
                                    i32.add
                                    call 11
                                    local.get 0
                                    i32.load8_u offset=96
                                    i32.const 1
                                    i32.eq
                                    br_if 12 (;@4;)
                                    local.get 0
                                    i32.const 128
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.get 0
                                    i32.const 113
                                    i32.add
                                    i64.load align=1
                                    i64.store
                                    local.get 0
                                    i32.const 128
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.get 0
                                    i32.const 105
                                    i32.add
                                    i64.load align=1
                                    i64.store
                                    local.get 0
                                    i32.const 151
                                    i32.add
                                    local.tee 3
                                    local.get 0
                                    i32.const 120
                                    i32.add
                                    i64.load align=1
                                    i64.store align=1
                                    local.get 0
                                    i32.const 72
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.tee 4
                                    local.get 0
                                    i32.const 143
                                    i32.add
                                    i64.load align=1
                                    i64.store
                                    local.get 0
                                    i32.const 72
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.tee 5
                                    local.get 3
                                    i64.load align=1
                                    i64.store
                                    local.get 0
                                    local.get 0
                                    i64.load offset=97 align=1
                                    i64.store offset=128
                                    local.get 0
                                    local.get 0
                                    i64.load offset=135 align=1
                                    i64.store offset=72
                                    local.get 1
                                    i32.eqz
                                    br_if 15 (;@1;)
                                    local.get 4
                                    i64.load
                                    local.set 6
                                    local.get 0
                                    i64.load offset=72
                                    local.set 7
                                    local.get 2
                                    i32.const 16
                                    i32.add
                                    local.get 5
                                    i64.load
                                    i64.store
                                    local.get 2
                                    i32.const 8
                                    i32.add
                                    local.get 6
                                    i64.store
                                    local.get 2
                                    local.get 7
                                    i64.store
                                    br 15 (;@1;)
                                  end
                                  local.get 0
                                  i32.const 96
                                  i32.add
                                  local.get 0
                                  i32.const 24
                                  i32.add
                                  call 11
                                  local.get 0
                                  i32.load8_u offset=96
                                  i32.const 1
                                  i32.eq
                                  br_if 11 (;@4;)
                                  local.get 0
                                  i32.const 128
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 0
                                  i32.const 113
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 128
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 0
                                  i32.const 105
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 151
                                  i32.add
                                  local.tee 3
                                  local.get 0
                                  i32.const 120
                                  i32.add
                                  i64.load align=1
                                  i64.store align=1
                                  local.get 0
                                  i32.const 72
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.tee 4
                                  local.get 0
                                  i32.const 143
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 72
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.tee 5
                                  local.get 3
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  local.get 0
                                  i64.load offset=97 align=1
                                  i64.store offset=128
                                  local.get 0
                                  local.get 0
                                  i64.load offset=135 align=1
                                  i64.store offset=72
                                  local.get 1
                                  i32.eqz
                                  br_if 14 (;@1;)
                                  local.get 4
                                  i64.load
                                  local.set 6
                                  local.get 0
                                  i64.load offset=72
                                  local.set 7
                                  local.get 2
                                  i32.const 16
                                  i32.add
                                  local.get 5
                                  i64.load
                                  i64.store
                                  local.get 2
                                  i32.const 8
                                  i32.add
                                  local.get 6
                                  i64.store
                                  local.get 2
                                  local.get 7
                                  i64.store
                                  local.get 0
                                  i32.const 96
                                  i32.add
                                  local.get 0
                                  i32.const 24
                                  i32.add
                                  call 11
                                  local.get 0
                                  i32.load8_u offset=96
                                  i32.const 1
                                  i32.eq
                                  br_if 11 (;@4;)
                                  local.get 0
                                  i32.const 128
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 0
                                  i32.const 113
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 128
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 0
                                  i32.const 105
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 151
                                  i32.add
                                  local.tee 3
                                  local.get 0
                                  i32.const 120
                                  i32.add
                                  i64.load align=1
                                  i64.store align=1
                                  local.get 0
                                  i32.const 72
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.tee 4
                                  local.get 0
                                  i32.const 143
                                  i32.add
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  i32.const 72
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.tee 5
                                  local.get 3
                                  i64.load align=1
                                  i64.store
                                  local.get 0
                                  local.get 0
                                  i64.load offset=97 align=1
                                  i64.store offset=128
                                  local.get 0
                                  local.get 0
                                  i64.load offset=135 align=1
                                  i64.store offset=72
                                  local.get 1
                                  i32.const 1
                                  i32.le_u
                                  br_if 14 (;@1;)
                                  local.get 2
                                  local.get 0
                                  i64.load offset=72
                                  i64.store offset=24
                                  local.get 2
                                  i32.const 40
                                  i32.add
                                  local.get 5
                                  i64.load
                                  i64.store
                                  local.get 2
                                  i32.const 32
                                  i32.add
                                  local.get 4
                                  i64.load
                                  i64.store
                                  br 14 (;@1;)
                                end
                                local.get 0
                                i32.const 96
                                i32.add
                                local.get 0
                                i32.const 24
                                i32.add
                                call 11
                                local.get 0
                                i32.load8_u offset=96
                                i32.const 1
                                i32.eq
                                br_if 10 (;@4;)
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.const 113
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 0
                                i32.const 105
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 151
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.const 120
                                i32.add
                                i64.load align=1
                                i64.store align=1
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 8
                                i32.add
                                local.tee 4
                                local.get 0
                                i32.const 143
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 16
                                i32.add
                                local.tee 5
                                local.get 3
                                i64.load align=1
                                i64.store
                                local.get 0
                                local.get 0
                                i64.load offset=97 align=1
                                i64.store offset=128
                                local.get 0
                                local.get 0
                                i64.load offset=135 align=1
                                i64.store offset=72
                                local.get 1
                                i32.eqz
                                br_if 13 (;@1;)
                                local.get 4
                                i64.load
                                local.set 6
                                local.get 0
                                i64.load offset=72
                                local.set 7
                                local.get 2
                                i32.const 16
                                i32.add
                                local.get 5
                                i64.load
                                i64.store
                                local.get 2
                                i32.const 8
                                i32.add
                                local.get 6
                                i64.store
                                local.get 2
                                local.get 7
                                i64.store
                                local.get 0
                                i32.const 96
                                i32.add
                                local.get 0
                                i32.const 24
                                i32.add
                                call 11
                                local.get 0
                                i32.load8_u offset=96
                                i32.const 1
                                i32.eq
                                br_if 10 (;@4;)
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.const 113
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 0
                                i32.const 105
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 151
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.const 120
                                i32.add
                                i64.load align=1
                                i64.store align=1
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 8
                                i32.add
                                local.tee 4
                                local.get 0
                                i32.const 143
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 16
                                i32.add
                                local.tee 5
                                local.get 3
                                i64.load align=1
                                i64.store
                                local.get 0
                                local.get 0
                                i64.load offset=97 align=1
                                i64.store offset=128
                                local.get 0
                                local.get 0
                                i64.load offset=135 align=1
                                i64.store offset=72
                                local.get 1
                                i32.const 1
                                i32.le_u
                                br_if 13 (;@1;)
                                local.get 2
                                local.get 0
                                i64.load offset=72
                                i64.store offset=24
                                local.get 2
                                i32.const 40
                                i32.add
                                local.get 5
                                i64.load
                                i64.store
                                local.get 2
                                i32.const 32
                                i32.add
                                local.get 4
                                i64.load
                                i64.store
                                local.get 0
                                i32.const 96
                                i32.add
                                local.get 0
                                i32.const 24
                                i32.add
                                call 11
                                local.get 0
                                i32.load8_u offset=96
                                i32.const 1
                                i32.eq
                                br_if 10 (;@4;)
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 0
                                i32.const 113
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 128
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 0
                                i32.const 105
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 151
                                i32.add
                                local.tee 3
                                local.get 0
                                i32.const 120
                                i32.add
                                i64.load align=1
                                i64.store align=1
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 8
                                i32.add
                                local.tee 4
                                local.get 0
                                i32.const 143
                                i32.add
                                i64.load align=1
                                i64.store
                                local.get 0
                                i32.const 72
                                i32.add
                                i32.const 16
                                i32.add
                                local.tee 5
                                local.get 3
                                i64.load align=1
                                i64.store
                                local.get 0
                                local.get 0
                                i64.load offset=97 align=1
                                i64.store offset=128
                                local.get 0
                                local.get 0
                                i64.load offset=135 align=1
                                i64.store offset=72
                                local.get 1
                                i32.const 2
                                i32.le_u
                                br_if 13 (;@1;)
                                local.get 2
                                local.get 0
                                i64.load offset=72
                                i64.store offset=48
                                local.get 2
                                i32.const 64
                                i32.add
                                local.get 5
                                i64.load
                                i64.store
                                local.get 2
                                i32.const 56
                                i32.add
                                local.get 4
                                i64.load
                                i64.store
                                br 13 (;@1;)
                              end
                              local.get 0
                              i32.const 96
                              i32.add
                              local.get 0
                              i32.const 24
                              i32.add
                              call 11
                              local.get 0
                              i32.load8_u offset=96
                              i32.const 1
                              i32.eq
                              br_if 9 (;@4;)
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 16
                              i32.add
                              local.get 0
                              i32.const 113
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 8
                              i32.add
                              local.get 0
                              i32.const 105
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 151
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.const 120
                              i32.add
                              i64.load align=1
                              i64.store align=1
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 8
                              i32.add
                              local.tee 4
                              local.get 0
                              i32.const 143
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 16
                              i32.add
                              local.tee 5
                              local.get 3
                              i64.load align=1
                              i64.store
                              local.get 0
                              local.get 0
                              i64.load offset=97 align=1
                              i64.store offset=128
                              local.get 0
                              local.get 0
                              i64.load offset=135 align=1
                              i64.store offset=72
                              local.get 1
                              i32.eqz
                              br_if 12 (;@1;)
                              local.get 4
                              i64.load
                              local.set 6
                              local.get 0
                              i64.load offset=72
                              local.set 7
                              local.get 2
                              i32.const 16
                              i32.add
                              local.get 5
                              i64.load
                              i64.store
                              local.get 2
                              i32.const 8
                              i32.add
                              local.get 6
                              i64.store
                              local.get 2
                              local.get 7
                              i64.store
                              local.get 0
                              i32.const 96
                              i32.add
                              local.get 0
                              i32.const 24
                              i32.add
                              call 11
                              local.get 0
                              i32.load8_u offset=96
                              i32.const 1
                              i32.eq
                              br_if 9 (;@4;)
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 16
                              i32.add
                              local.get 0
                              i32.const 113
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 8
                              i32.add
                              local.get 0
                              i32.const 105
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 151
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.const 120
                              i32.add
                              i64.load align=1
                              i64.store align=1
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 8
                              i32.add
                              local.tee 4
                              local.get 0
                              i32.const 143
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 16
                              i32.add
                              local.tee 5
                              local.get 3
                              i64.load align=1
                              i64.store
                              local.get 0
                              local.get 0
                              i64.load offset=97 align=1
                              i64.store offset=128
                              local.get 0
                              local.get 0
                              i64.load offset=135 align=1
                              i64.store offset=72
                              local.get 1
                              i32.const 1
                              i32.le_u
                              br_if 12 (;@1;)
                              local.get 2
                              local.get 0
                              i64.load offset=72
                              i64.store offset=24
                              local.get 2
                              i32.const 40
                              i32.add
                              local.get 5
                              i64.load
                              i64.store
                              local.get 2
                              i32.const 32
                              i32.add
                              local.get 4
                              i64.load
                              i64.store
                              local.get 0
                              i32.const 96
                              i32.add
                              local.get 0
                              i32.const 24
                              i32.add
                              call 11
                              local.get 0
                              i32.load8_u offset=96
                              i32.const 1
                              i32.eq
                              br_if 9 (;@4;)
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 16
                              i32.add
                              local.get 0
                              i32.const 113
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 8
                              i32.add
                              local.get 0
                              i32.const 105
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 151
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.const 120
                              i32.add
                              i64.load align=1
                              i64.store align=1
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 8
                              i32.add
                              local.tee 4
                              local.get 0
                              i32.const 143
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 16
                              i32.add
                              local.tee 5
                              local.get 3
                              i64.load align=1
                              i64.store
                              local.get 0
                              local.get 0
                              i64.load offset=97 align=1
                              i64.store offset=128
                              local.get 0
                              local.get 0
                              i64.load offset=135 align=1
                              i64.store offset=72
                              local.get 1
                              i32.const 2
                              i32.le_u
                              br_if 12 (;@1;)
                              local.get 2
                              local.get 0
                              i64.load offset=72
                              i64.store offset=48
                              local.get 2
                              i32.const 64
                              i32.add
                              local.get 5
                              i64.load
                              i64.store
                              local.get 2
                              i32.const 56
                              i32.add
                              local.get 4
                              i64.load
                              i64.store
                              local.get 0
                              i32.const 96
                              i32.add
                              local.get 0
                              i32.const 24
                              i32.add
                              call 11
                              local.get 0
                              i32.load8_u offset=96
                              i32.const 1
                              i32.eq
                              br_if 9 (;@4;)
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 16
                              i32.add
                              local.get 0
                              i32.const 113
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 128
                              i32.add
                              i32.const 8
                              i32.add
                              local.get 0
                              i32.const 105
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 151
                              i32.add
                              local.tee 3
                              local.get 0
                              i32.const 120
                              i32.add
                              i64.load align=1
                              i64.store align=1
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 8
                              i32.add
                              local.tee 4
                              local.get 0
                              i32.const 143
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 0
                              i32.const 72
                              i32.add
                              i32.const 16
                              i32.add
                              local.tee 5
                              local.get 3
                              i64.load align=1
                              i64.store
                              local.get 0
                              local.get 0
                              i64.load offset=97 align=1
                              i64.store offset=128
                              local.get 0
                              local.get 0
                              i64.load offset=135 align=1
                              i64.store offset=72
                              local.get 1
                              i32.const 3
                              i32.le_u
                              br_if 12 (;@1;)
                              local.get 2
                              local.get 0
                              i64.load offset=72
                              i64.store offset=72
                              local.get 2
                              i32.const 88
                              i32.add
                              local.get 5
                              i64.load
                              i64.store
                              local.get 2
                              i32.const 80
                              i32.add
                              local.get 4
                              i64.load
                              i64.store
                              br 12 (;@1;)
                            end
                            local.get 0
                            i32.const 96
                            i32.add
                            local.get 0
                            i32.const 24
                            i32.add
                            call 11
                            local.get 0
                            i32.load8_u offset=96
                            i32.const 1
                            i32.eq
                            br_if 8 (;@4;)
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.const 113
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.const 105
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 151
                            i32.add
                            local.tee 3
                            local.get 0
                            i32.const 120
                            i32.add
                            i64.load align=1
                            i64.store align=1
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 4
                            local.get 0
                            i32.const 143
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 5
                            local.get 3
                            i64.load align=1
                            i64.store
                            local.get 0
                            local.get 0
                            i64.load offset=97 align=1
                            i64.store offset=128
                            local.get 0
                            local.get 0
                            i64.load offset=135 align=1
                            i64.store offset=72
                            local.get 1
                            i32.eqz
                            br_if 11 (;@1;)
                            local.get 4
                            i64.load
                            local.set 6
                            local.get 0
                            i64.load offset=72
                            local.set 7
                            local.get 2
                            i32.const 16
                            i32.add
                            local.get 5
                            i64.load
                            i64.store
                            local.get 2
                            i32.const 8
                            i32.add
                            local.get 6
                            i64.store
                            local.get 2
                            local.get 7
                            i64.store
                            local.get 0
                            i32.const 96
                            i32.add
                            local.get 0
                            i32.const 24
                            i32.add
                            call 11
                            local.get 0
                            i32.load8_u offset=96
                            i32.const 1
                            i32.eq
                            br_if 8 (;@4;)
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.const 113
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.const 105
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 151
                            i32.add
                            local.tee 3
                            local.get 0
                            i32.const 120
                            i32.add
                            i64.load align=1
                            i64.store align=1
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 4
                            local.get 0
                            i32.const 143
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 5
                            local.get 3
                            i64.load align=1
                            i64.store
                            local.get 0
                            local.get 0
                            i64.load offset=97 align=1
                            i64.store offset=128
                            local.get 0
                            local.get 0
                            i64.load offset=135 align=1
                            i64.store offset=72
                            local.get 1
                            i32.const 1
                            i32.le_u
                            br_if 11 (;@1;)
                            local.get 2
                            local.get 0
                            i64.load offset=72
                            i64.store offset=24
                            local.get 2
                            i32.const 40
                            i32.add
                            local.get 5
                            i64.load
                            i64.store
                            local.get 2
                            i32.const 32
                            i32.add
                            local.get 4
                            i64.load
                            i64.store
                            local.get 0
                            i32.const 96
                            i32.add
                            local.get 0
                            i32.const 24
                            i32.add
                            call 11
                            local.get 0
                            i32.load8_u offset=96
                            i32.const 1
                            i32.eq
                            br_if 8 (;@4;)
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.const 113
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.const 105
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 151
                            i32.add
                            local.tee 3
                            local.get 0
                            i32.const 120
                            i32.add
                            i64.load align=1
                            i64.store align=1
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 4
                            local.get 0
                            i32.const 143
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 5
                            local.get 3
                            i64.load align=1
                            i64.store
                            local.get 0
                            local.get 0
                            i64.load offset=97 align=1
                            i64.store offset=128
                            local.get 0
                            local.get 0
                            i64.load offset=135 align=1
                            i64.store offset=72
                            local.get 1
                            i32.const 2
                            i32.le_u
                            br_if 11 (;@1;)
                            local.get 2
                            local.get 0
                            i64.load offset=72
                            i64.store offset=48
                            local.get 2
                            i32.const 64
                            i32.add
                            local.get 5
                            i64.load
                            i64.store
                            local.get 2
                            i32.const 56
                            i32.add
                            local.get 4
                            i64.load
                            i64.store
                            local.get 0
                            i32.const 96
                            i32.add
                            local.get 0
                            i32.const 24
                            i32.add
                            call 11
                            local.get 0
                            i32.load8_u offset=96
                            i32.const 1
                            i32.eq
                            br_if 8 (;@4;)
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.const 113
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.const 105
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 151
                            i32.add
                            local.tee 3
                            local.get 0
                            i32.const 120
                            i32.add
                            i64.load align=1
                            i64.store align=1
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 4
                            local.get 0
                            i32.const 143
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 5
                            local.get 3
                            i64.load align=1
                            i64.store
                            local.get 0
                            local.get 0
                            i64.load offset=97 align=1
                            i64.store offset=128
                            local.get 0
                            local.get 0
                            i64.load offset=135 align=1
                            i64.store offset=72
                            local.get 1
                            i32.const 3
                            i32.le_u
                            br_if 11 (;@1;)
                            local.get 2
                            local.get 0
                            i64.load offset=72
                            i64.store offset=72
                            local.get 2
                            i32.const 88
                            i32.add
                            local.get 5
                            i64.load
                            i64.store
                            local.get 2
                            i32.const 80
                            i32.add
                            local.get 4
                            i64.load
                            i64.store
                            local.get 0
                            i32.const 96
                            i32.add
                            local.get 0
                            i32.const 24
                            i32.add
                            call 11
                            local.get 0
                            i32.load8_u offset=96
                            i32.const 1
                            i32.eq
                            br_if 8 (;@4;)
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 16
                            i32.add
                            local.get 0
                            i32.const 113
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 128
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.const 105
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 151
                            i32.add
                            local.tee 3
                            local.get 0
                            i32.const 120
                            i32.add
                            i64.load align=1
                            i64.store align=1
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 8
                            i32.add
                            local.tee 4
                            local.get 0
                            i32.const 143
                            i32.add
                            i64.load align=1
                            i64.store
                            local.get 0
                            i32.const 72
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 5
                            local.get 3
                            i64.load align=1
                            i64.store
                            local.get 0
                            local.get 0
                            i64.load offset=97 align=1
                            i64.store offset=128
                            local.get 0
                            local.get 0
                            i64.load offset=135 align=1
                            i64.store offset=72
                            local.get 1
                            i32.const 4
                            i32.le_u
                            br_if 11 (;@1;)
                            local.get 2
                            local.get 0
                            i64.load offset=72
                            i64.store offset=96
                            local.get 2
                            i32.const 112
                            i32.add
                            local.get 5
                            i64.load
                            i64.store
                            local.get 2
                            i32.const 104
                            i32.add
                            local.get 4
                            i64.load
                            i64.store
                            br 11 (;@1;)
                          end
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.eqz
                          br_if 10 (;@1;)
                          local.get 4
                          i64.load
                          local.set 6
                          local.get 0
                          i64.load offset=72
                          local.set 7
                          local.get 2
                          i32.const 16
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 8
                          i32.add
                          local.get 6
                          i64.store
                          local.get 2
                          local.get 7
                          i64.store
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.const 1
                          i32.le_u
                          br_if 10 (;@1;)
                          local.get 2
                          local.get 0
                          i64.load offset=72
                          i64.store offset=24
                          local.get 2
                          i32.const 40
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 32
                          i32.add
                          local.get 4
                          i64.load
                          i64.store
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.const 2
                          i32.le_u
                          br_if 10 (;@1;)
                          local.get 2
                          local.get 0
                          i64.load offset=72
                          i64.store offset=48
                          local.get 2
                          i32.const 64
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 56
                          i32.add
                          local.get 4
                          i64.load
                          i64.store
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.const 3
                          i32.le_u
                          br_if 10 (;@1;)
                          local.get 2
                          local.get 0
                          i64.load offset=72
                          i64.store offset=72
                          local.get 2
                          i32.const 88
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 80
                          i32.add
                          local.get 4
                          i64.load
                          i64.store
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.const 4
                          i32.le_u
                          br_if 10 (;@1;)
                          local.get 2
                          local.get 0
                          i64.load offset=72
                          i64.store offset=96
                          local.get 2
                          i32.const 112
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 104
                          i32.add
                          local.get 4
                          i64.load
                          i64.store
                          local.get 0
                          i32.const 96
                          i32.add
                          local.get 0
                          i32.const 24
                          i32.add
                          call 11
                          local.get 0
                          i32.load8_u offset=96
                          i32.const 1
                          i32.eq
                          br_if 7 (;@4;)
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 0
                          i32.const 113
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 128
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 0
                          i32.const 105
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 151
                          i32.add
                          local.tee 3
                          local.get 0
                          i32.const 120
                          i32.add
                          i64.load align=1
                          i64.store align=1
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 8
                          i32.add
                          local.tee 4
                          local.get 0
                          i32.const 143
                          i32.add
                          i64.load align=1
                          i64.store
                          local.get 0
                          i32.const 72
                          i32.add
                          i32.const 16
                          i32.add
                          local.tee 5
                          local.get 3
                          i64.load align=1
                          i64.store
                          local.get 0
                          local.get 0
                          i64.load offset=97 align=1
                          i64.store offset=128
                          local.get 0
                          local.get 0
                          i64.load offset=135 align=1
                          i64.store offset=72
                          local.get 1
                          i32.const 5
                          i32.le_u
                          br_if 10 (;@1;)
                          local.get 2
                          local.get 0
                          i64.load offset=72
                          i64.store offset=120
                          local.get 2
                          i32.const 136
                          i32.add
                          local.get 5
                          i64.load
                          i64.store
                          local.get 2
                          i32.const 128
                          i32.add
                          local.get 4
                          i64.load
                          i64.store
                          br 10 (;@1;)
                        end
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 4
                        i64.load
                        local.set 6
                        local.get 0
                        i64.load offset=72
                        local.set 7
                        local.get 2
                        i32.const 16
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 8
                        i32.add
                        local.get 6
                        i64.store
                        local.get 2
                        local.get 7
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 1
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=24
                        local.get 2
                        i32.const 40
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 32
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 2
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=48
                        local.get 2
                        i32.const 64
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 56
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 3
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=72
                        local.get 2
                        i32.const 88
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 80
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 4
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=96
                        local.get 2
                        i32.const 112
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 104
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 5
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=120
                        local.get 2
                        i32.const 136
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 128
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 6
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=144
                        local.get 2
                        i32.const 160
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 152
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 0
                        i32.const 96
                        i32.add
                        local.get 0
                        i32.const 24
                        i32.add
                        call 11
                        local.get 0
                        i32.load8_u offset=96
                        i32.const 1
                        i32.eq
                        br_if 6 (;@4;)
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 16
                        i32.add
                        local.get 0
                        i32.const 113
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 128
                        i32.add
                        i32.const 8
                        i32.add
                        local.get 0
                        i32.const 105
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 151
                        i32.add
                        local.tee 3
                        local.get 0
                        i32.const 120
                        i32.add
                        i64.load align=1
                        i64.store align=1
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 8
                        i32.add
                        local.tee 4
                        local.get 0
                        i32.const 143
                        i32.add
                        i64.load align=1
                        i64.store
                        local.get 0
                        i32.const 72
                        i32.add
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 3
                        i64.load align=1
                        i64.store
                        local.get 0
                        local.get 0
                        i64.load offset=97 align=1
                        i64.store offset=128
                        local.get 0
                        local.get 0
                        i64.load offset=135 align=1
                        i64.store offset=72
                        local.get 1
                        i32.const 7
                        i32.le_u
                        br_if 9 (;@1;)
                        local.get 2
                        local.get 0
                        i64.load offset=72
                        i64.store offset=168
                        local.get 2
                        i32.const 184
                        i32.add
                        local.get 5
                        i64.load
                        i64.store
                        local.get 2
                        i32.const 176
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        br 9 (;@1;)
                      end
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.eqz
                      br_if 8 (;@1;)
                      local.get 4
                      i64.load
                      local.set 6
                      local.get 0
                      i64.load offset=72
                      local.set 7
                      local.get 2
                      i32.const 16
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 8
                      i32.add
                      local.get 6
                      i64.store
                      local.get 2
                      local.get 7
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 1
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=24
                      local.get 2
                      i32.const 40
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 32
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 2
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=48
                      local.get 2
                      i32.const 64
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 56
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 3
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=72
                      local.get 2
                      i32.const 88
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 80
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 4
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=96
                      local.get 2
                      i32.const 112
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 104
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 5
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=120
                      local.get 2
                      i32.const 136
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 128
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 6
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=144
                      local.get 2
                      i32.const 160
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 152
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 7
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=168
                      local.get 2
                      i32.const 184
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 176
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      local.get 0
                      i32.const 96
                      i32.add
                      local.get 0
                      i32.const 24
                      i32.add
                      call 11
                      local.get 0
                      i32.load8_u offset=96
                      i32.const 1
                      i32.eq
                      br_if 5 (;@4;)
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 0
                      i32.const 113
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 128
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.const 105
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 151
                      i32.add
                      local.tee 3
                      local.get 0
                      i32.const 120
                      i32.add
                      i64.load align=1
                      i64.store align=1
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 8
                      i32.add
                      local.tee 4
                      local.get 0
                      i32.const 143
                      i32.add
                      i64.load align=1
                      i64.store
                      local.get 0
                      i32.const 72
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 3
                      i64.load align=1
                      i64.store
                      local.get 0
                      local.get 0
                      i64.load offset=97 align=1
                      i64.store offset=128
                      local.get 0
                      local.get 0
                      i64.load offset=135 align=1
                      i64.store offset=72
                      local.get 1
                      i32.const 8
                      i32.le_u
                      br_if 8 (;@1;)
                      local.get 2
                      local.get 0
                      i64.load offset=72
                      i64.store offset=192
                      local.get 2
                      i32.const 208
                      i32.add
                      local.get 5
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 200
                      i32.add
                      local.get 4
                      i64.load
                      i64.store
                      br 8 (;@1;)
                    end
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.eqz
                    br_if 7 (;@1;)
                    local.get 4
                    i64.load
                    local.set 6
                    local.get 0
                    i64.load offset=72
                    local.set 7
                    local.get 2
                    i32.const 16
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 8
                    i32.add
                    local.get 6
                    i64.store
                    local.get 2
                    local.get 7
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 1
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=24
                    local.get 2
                    i32.const 40
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 32
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 2
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=48
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 56
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 3
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=72
                    local.get 2
                    i32.const 88
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 80
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 4
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=96
                    local.get 2
                    i32.const 112
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 104
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 5
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=120
                    local.get 2
                    i32.const 136
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 128
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 6
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=144
                    local.get 2
                    i32.const 160
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 152
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 7
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=168
                    local.get 2
                    i32.const 184
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 176
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 105
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 8
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=192
                    local.get 2
                    i32.const 208
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 200
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 0
                    i32.const 96
                    i32.add
                    local.get 0
                    i32.const 24
                    i32.add
                    call 11
                    local.get 0
                    i32.load8_u offset=96
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 0
                    i32.const 113
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 128
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 0
                    i32.const 96
                    i32.add
                    i32.const 9
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 151
                    i32.add
                    local.tee 3
                    local.get 0
                    i32.const 120
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 4
                    local.get 0
                    i32.const 143
                    i32.add
                    i64.load align=1
                    i64.store
                    local.get 0
                    i32.const 72
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 5
                    local.get 3
                    i64.load align=1
                    i64.store
                    local.get 0
                    local.get 0
                    i64.load offset=97 align=1
                    i64.store offset=128
                    local.get 0
                    local.get 0
                    i64.load offset=135 align=1
                    i64.store offset=72
                    local.get 1
                    i32.const 9
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 0
                    i64.load offset=72
                    i64.store offset=216
                    local.get 2
                    i32.const 232
                    i32.add
                    local.get 5
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 224
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    br 7 (;@1;)
                  end
                  local.get 0
                  i32.const 16
                  i32.add
                  local.get 1
                  call 12
                  local.get 0
                  i32.load8_u offset=17
                  local.set 2
                  local.get 0
                  i32.load8_u offset=16
                  local.set 1
                  br 5 (;@2;)
                end
                local.get 0
                i32.const 40
                i32.add
                local.get 0
                i32.const 24
                i32.add
                call 11
                local.get 0
                i32.load8_u offset=40
                i32.const 1
                i32.eq
                br_if 5 (;@1;)
                block  ;; label = @7
                  local.get 0
                  i32.load offset=48
                  br_table 0 (;@7;) 6 (;@1;) 6 (;@1;) 6 (;@1;)
                end
                local.get 0
                i32.const 56
                i32.add
                i32.load8_u
                i32.const 255
                i32.and
                i32.const 3
                i32.eq
                br_if 3 (;@3;)
                br 5 (;@1;)
              end
              local.get 0
              local.get 1
              call 13
              local.get 0
              i32.const 1
              i32.store8 offset=40
              local.get 0
              local.get 0
              i32.load16_u
              i32.store16 offset=41 align=1
              br 4 (;@1;)
            end
            local.get 0
            i32.const 8
            i32.add
            local.get 1
            call 13
            local.get 0
            i32.const 1
            i32.store8 offset=40
            local.get 0
            local.get 0
            i32.load16_u offset=8
            i32.store16 offset=41 align=1
            br 3 (;@1;)
          end
          local.get 0
          i32.load8_u offset=97
          local.set 1
          local.get 0
          i32.load8_u offset=98
          local.set 2
          br 1 (;@2;)
        end
        local.get 0
        i32.const 60
        i32.add
        i32.load
        i32.const 0
        call 3
        local.get 0
        i32.const 160
        i32.add
        global.set 0
        return
      end
      local.get 0
      local.get 2
      i32.store8 offset=42
      local.get 0
      local.get 1
      i32.store8 offset=41
      local.get 0
      i32.const 1
      i32.store8 offset=40
    end
    unreachable
    unreachable)
  (func (;9;) (type 0) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    call 16
    i32.const 1
    local.set 2
    local.get 1
    i32.load8_u offset=9
    i32.const 2
    local.get 1
    i32.load8_u offset=8
    i32.const 1
    i32.and
    local.tee 0
    select
    local.set 3
    local.get 0
    i32.const 1
    i32.xor
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 0
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 4
                                      br_if 16 (;@1;)
                                      i32.const 0
                                      local.set 2
                                      i32.const 0
                                      local.set 4
                                      local.get 3
                                      i32.const 255
                                      i32.and
                                      br_table 2 (;@15;) 4 (;@13;) 5 (;@12;) 9 (;@8;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 2 (;@15;) 4 (;@13;) 6 (;@11;) 9 (;@8;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 15 (;@2;) 4 (;@13;) 7 (;@10;) 9 (;@8;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 1 (;@16;) 4 (;@13;) 7 (;@10;) 9 (;@8;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 3 (;@14;) 4 (;@13;) 8 (;@9;) 10 (;@7;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 4 (;@13;) 8 (;@9;) 10 (;@7;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 4 (;@13;) 16 (;@1;) 10 (;@7;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 16 (;@1;) 4 (;@13;) 16 (;@1;) 10 (;@7;) 11 (;@6;) 12 (;@5;) 13 (;@4;) 16 (;@1;)
                                    end
                                    local.get 1
                                    local.get 3
                                    i32.const 255
                                    i32.and
                                    i32.const 8
                                    i32.shl
                                    local.get 4
                                    i32.or
                                    call 12
                                    local.get 1
                                    i32.load8_u offset=1
                                    local.set 0
                                    local.get 1
                                    i32.load8_u
                                    local.set 4
                                    br 14 (;@2;)
                                  end
                                  i32.const 1
                                  local.set 4
                                  br 12 (;@3;)
                                end
                                i32.const 2
                                local.set 4
                                br 11 (;@3;)
                              end
                              i32.const 3
                              local.set 4
                              br 10 (;@3;)
                            end
                            i32.const 4
                            local.set 4
                            br 9 (;@3;)
                          end
                          i32.const 5
                          local.set 4
                          br 8 (;@3;)
                        end
                        i32.const 6
                        local.set 4
                        br 7 (;@3;)
                      end
                      i32.const 7
                      local.set 4
                      br 6 (;@3;)
                    end
                    i32.const 8
                    local.set 4
                    br 5 (;@3;)
                  end
                  i32.const 9
                  local.set 4
                  br 4 (;@3;)
                end
                i32.const 10
                local.set 4
                br 3 (;@3;)
              end
              i32.const 11
              local.set 4
              br 2 (;@3;)
            end
            i32.const 12
            local.set 4
            br 1 (;@3;)
          end
          i32.const 13
          local.set 4
        end
      end
      local.get 1
      i32.const 16
      i32.add
      global.set 0
      local.get 4
      i32.const 255
      i32.and
      i32.const 8
      i32.shl
      local.get 0
      i32.const 16
      i32.shl
      i32.or
      local.get 2
      i32.or
      return
    end
    unreachable
    unreachable)
  (func (;10;) (type 0) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    call 16
    local.get 1
    i32.load8_u offset=9
    local.set 2
    local.get 1
    i32.load8_u offset=8
    local.set 3
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
    i32.const 8
    i32.shl
    i32.const 512
    local.get 3
    i32.const 1
    i32.and
    local.tee 1
    select
    local.get 1
    i32.const 1
    i32.xor
    i32.or)
  (func (;11;) (type 2) (param i32 i32)
    (local i32 i32 i64 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 1
                                          call 9
                                          local.tee 3
                                          i32.const 255
                                          i32.and
                                          i32.const 1
                                          i32.eq
                                          br_if 0 (;@19;)
                                          local.get 3
                                          i32.const 1
                                          i32.and
                                          br_if 14 (;@5;)
                                          local.get 3
                                          i32.const 8
                                          i32.shr_u
                                          i32.const 255
                                          i32.and
                                          br_table 1 (;@18;) 2 (;@17;) 3 (;@16;) 4 (;@15;) 5 (;@14;) 6 (;@13;) 7 (;@12;) 8 (;@11;) 9 (;@10;) 10 (;@9;) 11 (;@8;) 12 (;@7;) 13 (;@6;) 14 (;@5;)
                                        end
                                        local.get 2
                                        local.get 3
                                        call 13
                                        local.get 0
                                        local.get 2
                                        i32.load16_u
                                        i32.store16 offset=1 align=1
                                        local.get 0
                                        i32.const 1
                                        i32.store8
                                        br 17 (;@1;)
                                      end
                                      block  ;; label = @18
                                        local.get 1
                                        call 10
                                        local.tee 1
                                        i32.const 255
                                        i32.and
                                        i32.const 1
                                        i32.eq
                                        br_if 0 (;@18;)
                                        local.get 1
                                        i32.const 1
                                        i32.and
                                        br_if 13 (;@5;)
                                        i32.const 0
                                        local.set 3
                                        br 15 (;@3;)
                                      end
                                      local.get 2
                                      i32.const 8
                                      i32.add
                                      local.get 1
                                      call 12
                                      local.get 0
                                      local.get 2
                                      i32.load16_u offset=8
                                      i32.store16 offset=1 align=1
                                      local.get 0
                                      i32.const 1
                                      i32.store8
                                      br 16 (;@1;)
                                    end
                                    i32.const 1
                                    local.set 3
                                    block  ;; label = @17
                                      local.get 1
                                      call 10
                                      local.tee 1
                                      i32.const 255
                                      i32.and
                                      i32.const 1
                                      i32.eq
                                      br_if 0 (;@17;)
                                      local.get 1
                                      i32.const 1
                                      i32.and
                                      br_if 12 (;@5;)
                                      br 14 (;@3;)
                                    end
                                    local.get 2
                                    i32.const 16
                                    i32.add
                                    local.get 1
                                    call 12
                                    local.get 0
                                    local.get 2
                                    i32.load16_u offset=16
                                    i32.store16 offset=1 align=1
                                    local.get 0
                                    i32.const 1
                                    i32.store8
                                    br 15 (;@1;)
                                  end
                                  block  ;; label = @16
                                    local.get 1
                                    call 10
                                    local.tee 3
                                    i32.const 255
                                    i32.and
                                    i32.const 1
                                    i32.eq
                                    br_if 0 (;@16;)
                                    local.get 3
                                    i32.const 1
                                    i32.and
                                    br_if 11 (;@5;)
                                    i32.const 0
                                    local.set 1
                                    block  ;; label = @17
                                      local.get 3
                                      i32.const 8
                                      i32.shr_u
                                      i32.const 255
                                      i32.and
                                      local.tee 3
                                      i32.eqz
                                      br_if 0 (;@17;)
                                      local.get 3
                                      i32.const 16
                                      i32.ne
                                      br_if 12 (;@5;)
                                      i32.const 1
                                      local.set 1
                                    end
                                    i32.const 2
                                    local.set 3
                                    br 14 (;@2;)
                                  end
                                  local.get 2
                                  i32.const 24
                                  i32.add
                                  local.get 3
                                  call 12
                                  local.get 0
                                  local.get 2
                                  i32.load16_u offset=24
                                  i32.store16 offset=1 align=1
                                  local.get 0
                                  i32.const 1
                                  i32.store8
                                  br 14 (;@1;)
                                end
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 1
                                    call 10
                                    local.tee 3
                                    i32.const 255
                                    i32.and
                                    i32.const 1
                                    i32.eq
                                    br_if 0 (;@16;)
                                    local.get 3
                                    i32.const 1
                                    i32.and
                                    br_if 11 (;@5;)
                                    block  ;; label = @17
                                      local.get 1
                                      i32.const 20
                                      call 17
                                      local.tee 4
                                      i32.wrap_i64
                                      i32.const 255
                                      i32.and
                                      i32.const 1
                                      i32.ne
                                      br_if 0 (;@17;)
                                      local.get 2
                                      i32.const 40
                                      i32.add
                                      local.get 4
                                      call 18
                                      local.get 2
                                      i32.load8_u offset=41
                                      local.set 1
                                      local.get 2
                                      i32.load8_u offset=40
                                      local.set 3
                                      br 2 (;@15;)
                                    end
                                    local.get 4
                                    i64.const 1
                                    i64.and
                                    i64.eqz
                                    i32.eqz
                                    br_if 11 (;@5;)
                                    local.get 4
                                    i64.const 32
                                    i64.shr_u
                                    i32.wrap_i64
                                    local.set 5
                                    i32.const 3
                                    local.set 3
                                    br 13 (;@3;)
                                  end
                                  local.get 2
                                  i32.const 32
                                  i32.add
                                  local.get 3
                                  call 12
                                  local.get 2
                                  i32.load8_u offset=33
                                  local.set 1
                                  local.get 2
                                  i32.load8_u offset=32
                                  local.set 3
                                end
                                local.get 0
                                local.get 3
                                i32.store8 offset=1
                                local.get 0
                                i32.const 1
                                i32.store8
                                local.get 0
                                i32.const 2
                                i32.add
                                local.get 1
                                i32.store8
                                br 13 (;@1;)
                              end
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 1
                                  call 10
                                  local.tee 3
                                  i32.const 255
                                  i32.and
                                  i32.const 1
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 3
                                  i32.const 1
                                  i32.and
                                  br_if 10 (;@5;)
                                  i32.const 1
                                  local.set 5
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  local.get 3
                                                  i32.const 16776960
                                                  i32.and
                                                  i32.const 8
                                                  i32.shr_u
                                                  i32.const -1
                                                  i32.add
                                                  local.tee 3
                                                  i32.const 4
                                                  i32.shl
                                                  local.get 3
                                                  i32.const 240
                                                  i32.and
                                                  i32.const 4
                                                  i32.shr_u
                                                  i32.or
                                                  i32.const 255
                                                  i32.and
                                                  br_table 7 (;@16;) 0 (;@23;) 1 (;@22;) 2 (;@21;) 3 (;@20;) 4 (;@19;) 5 (;@18;) 6 (;@17;) 18 (;@5;)
                                                end
                                                i32.const 2
                                                local.set 5
                                                br 6 (;@16;)
                                              end
                                              i32.const 3
                                              local.set 5
                                              br 5 (;@16;)
                                            end
                                            i32.const 4
                                            local.set 5
                                            br 4 (;@16;)
                                          end
                                          i32.const 5
                                          local.set 5
                                          br 3 (;@16;)
                                        end
                                        i32.const 6
                                        local.set 5
                                        br 2 (;@16;)
                                      end
                                      i32.const 7
                                      local.set 5
                                      br 1 (;@16;)
                                    end
                                    i32.const 8
                                    local.set 5
                                  end
                                  local.get 2
                                  i32.const 112
                                  i32.add
                                  local.get 1
                                  local.get 5
                                  call 19
                                  block  ;; label = @16
                                    local.get 2
                                    i32.load8_u offset=112
                                    i32.const 1
                                    i32.ne
                                    br_if 0 (;@16;)
                                    local.get 2
                                    i32.load8_u offset=113
                                    local.set 1
                                    local.get 2
                                    i32.load8_u offset=114
                                    local.set 3
                                    br 2 (;@14;)
                                  end
                                  local.get 2
                                  i32.const 120
                                  i32.add
                                  i64.load
                                  local.set 4
                                  i32.const 4
                                  local.set 3
                                  br 12 (;@3;)
                                end
                                local.get 2
                                i32.const 48
                                i32.add
                                local.get 3
                                call 12
                                local.get 2
                                i32.load8_u offset=49
                                local.set 3
                                local.get 2
                                i32.load8_u offset=48
                                local.set 1
                              end
                              local.get 0
                              local.get 1
                              i32.store8 offset=1
                              local.get 0
                              i32.const 1
                              i32.store8
                              local.get 0
                              i32.const 2
                              i32.add
                              local.get 3
                              i32.store8
                              br 12 (;@1;)
                            end
                            block  ;; label = @13
                              local.get 1
                              call 20
                              local.tee 1
                              i32.const 255
                              i32.and
                              i32.const 1
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 1
                              i32.const 1
                              i32.and
                              br_if 8 (;@5;)
                              local.get 1
                              i32.const 16776960
                              i32.and
                              i32.const 8
                              i32.shr_u
                              local.set 1
                              i32.const 5
                              local.set 3
                              br 11 (;@2;)
                            end
                            local.get 2
                            i32.const 56
                            i32.add
                            local.get 1
                            call 21
                            local.get 0
                            local.get 2
                            i32.load16_u offset=56
                            i32.store16 offset=1 align=1
                            local.get 0
                            i32.const 1
                            i32.store8
                            br 11 (;@1;)
                          end
                          block  ;; label = @12
                            local.get 1
                            call 20
                            local.tee 1
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 1
                            i32.and
                            br_if 7 (;@5;)
                            local.get 1
                            i32.const 16776960
                            i32.and
                            i32.const 8
                            i32.shr_u
                            local.set 1
                            i32.const 6
                            local.set 3
                            br 10 (;@2;)
                          end
                          local.get 2
                          i32.const 72
                          i32.add
                          local.get 1
                          call 21
                          local.get 2
                          i32.const 64
                          i32.add
                          local.get 2
                          i32.load16_u offset=72
                          i32.const 8
                          i32.shl
                          i32.const 1
                          i32.or
                          call 12
                          local.get 0
                          local.get 2
                          i32.load16_u offset=64
                          i32.store16 offset=1 align=1
                          local.get 0
                          i32.const 1
                          i32.store8
                          br 10 (;@1;)
                        end
                        block  ;; label = @11
                          local.get 1
                          call 22
                          local.tee 1
                          i32.const 255
                          i32.and
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 1
                          i32.const 1
                          i32.and
                          br_if 6 (;@5;)
                          local.get 1
                          i32.const 16
                          i32.shr_u
                          local.set 6
                          i32.const 7
                          local.set 3
                          br 7 (;@4;)
                        end
                        local.get 2
                        i32.const 80
                        i32.add
                        local.get 1
                        call 23
                        local.get 0
                        local.get 2
                        i32.load16_u offset=80
                        i32.store16 offset=1 align=1
                        local.get 0
                        i32.const 1
                        i32.store8
                        br 9 (;@1;)
                      end
                      block  ;; label = @10
                        local.get 1
                        call 22
                        local.tee 1
                        i32.const 255
                        i32.and
                        i32.const 1
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 1
                        i32.and
                        br_if 5 (;@5;)
                        local.get 1
                        i32.const 16
                        i32.shr_u
                        local.set 6
                        i32.const 8
                        local.set 3
                        br 6 (;@4;)
                      end
                      local.get 2
                      i32.const 88
                      i32.add
                      local.get 1
                      call 23
                      local.get 0
                      local.get 2
                      i32.load16_u offset=88
                      i32.store16 offset=1 align=1
                      local.get 0
                      i32.const 1
                      i32.store8
                      br 8 (;@1;)
                    end
                    block  ;; label = @9
                      local.get 1
                      call 24
                      local.tee 4
                      i32.wrap_i64
                      i32.const 255
                      i32.and
                      i32.const 1
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 4
                      i64.const 1
                      i64.and
                      i64.eqz
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 4
                      i64.const 32
                      i64.shr_u
                      i32.wrap_i64
                      local.set 5
                      i32.const 9
                      local.set 3
                      br 6 (;@3;)
                    end
                    local.get 2
                    i32.const 96
                    i32.add
                    local.get 4
                    call 25
                    local.get 0
                    local.get 2
                    i32.load16_u offset=96
                    i32.store16 offset=1 align=1
                    local.get 0
                    i32.const 1
                    i32.store8
                    br 7 (;@1;)
                  end
                  block  ;; label = @8
                    local.get 1
                    call 24
                    local.tee 4
                    i32.wrap_i64
                    i32.const 255
                    i32.and
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 4
                    i64.const 1
                    i64.and
                    i64.eqz
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 4
                    i64.const 32
                    i64.shr_u
                    i32.wrap_i64
                    local.set 5
                    i32.const 10
                    local.set 3
                    br 5 (;@3;)
                  end
                  local.get 2
                  i32.const 104
                  i32.add
                  local.get 4
                  call 25
                  local.get 0
                  local.get 2
                  i32.load16_u offset=104
                  i32.store16 offset=1 align=1
                  local.get 0
                  i32.const 1
                  i32.store8
                  br 6 (;@1;)
                end
                local.get 2
                i32.const 112
                i32.add
                local.get 1
                call 26
                block  ;; label = @7
                  local.get 2
                  i32.load8_u offset=112
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 120
                  i32.add
                  i64.load
                  local.set 4
                  i32.const 11
                  local.set 3
                  br 4 (;@3;)
                end
                local.get 0
                local.get 2
                i32.load16_u offset=113 align=1
                i32.store16 offset=1 align=1
                local.get 0
                i32.const 1
                i32.store8
                br 5 (;@1;)
              end
              local.get 2
              i32.const 112
              i32.add
              local.get 1
              call 26
              block  ;; label = @6
                local.get 2
                i32.load8_u offset=112
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                local.get 2
                i32.const 120
                i32.add
                i64.load
                local.set 4
                i32.const 12
                local.set 3
                br 3 (;@3;)
              end
              local.get 0
              local.get 2
              i32.load16_u offset=113 align=1
              i32.store16 offset=1 align=1
              local.get 0
              i32.const 1
              i32.store8
              br 4 (;@1;)
            end
            unreachable
            unreachable
          end
        end
      end
      local.get 0
      i32.const 0
      i32.store8
      local.get 0
      i32.const 24
      i32.add
      local.get 4
      i64.store
      local.get 0
      i32.const 20
      i32.add
      local.get 5
      i32.store
      local.get 0
      i32.const 18
      i32.add
      local.get 6
      i32.store16
      local.get 0
      i32.const 17
      i32.add
      local.get 1
      i32.store8
      local.get 0
      i32.const 16
      i32.add
      local.get 3
      i32.store8
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
    end
    local.get 2
    i32.const 128
    i32.add
    global.set 0)
  (func (;12;) (type 2) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 16
    i32.shr_u
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.const 8
    i32.shr_u
    i32.store8)
  (func (;13;) (type 2) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 16
    i32.shr_u
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.const 8
    i32.shr_u
    i32.store8)
  (func (;14;) (type 3)
    (local i32 i32)
    i32.const 0
    i32.const 20
    call 0
    local.tee 0
    call 4
    i32.const 21
    call 0
    local.tee 1
    i32.const 64
    i32.store8
    local.get 1
    local.get 0
    i32.load8_u
    i32.store8 offset=1
    local.get 1
    local.get 0
    i32.load8_u offset=1
    i32.store8 offset=2
    local.get 1
    local.get 0
    i32.load8_u offset=2
    i32.store8 offset=3
    local.get 1
    local.get 0
    i32.load8_u offset=3
    i32.store8 offset=4
    local.get 1
    local.get 0
    i32.load8_u offset=4
    i32.store8 offset=5
    local.get 1
    local.get 0
    i32.load8_u offset=5
    i32.store8 offset=6
    local.get 1
    local.get 0
    i32.load8_u offset=6
    i32.store8 offset=7
    local.get 1
    local.get 0
    i32.load8_u offset=7
    i32.store8 offset=8
    local.get 1
    local.get 0
    i32.load8_u offset=8
    i32.store8 offset=9
    local.get 1
    local.get 0
    i32.load8_u offset=9
    i32.store8 offset=10
    local.get 1
    local.get 0
    i32.load8_u offset=10
    i32.store8 offset=11
    local.get 1
    local.get 0
    i32.load8_u offset=11
    i32.store8 offset=12
    local.get 1
    local.get 0
    i32.load8_u offset=12
    i32.store8 offset=13
    local.get 1
    local.get 0
    i32.load8_u offset=13
    i32.store8 offset=14
    local.get 1
    local.get 0
    i32.load8_u offset=14
    i32.store8 offset=15
    local.get 1
    local.get 0
    i32.load8_u offset=15
    i32.store8 offset=16
    local.get 1
    local.get 0
    i32.load8_u offset=16
    i32.store8 offset=17
    local.get 1
    local.get 0
    i32.load8_u offset=17
    i32.store8 offset=18
    local.get 1
    local.get 0
    i32.load8_u offset=18
    i32.store8 offset=19
    local.get 1
    local.get 0
    i32.load8_u offset=19
    i32.store8 offset=20
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1048576
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048576
    end
    local.get 1
    i32.const 21
    call 5)
  (func (;15;) (type 3))
  (func (;16;) (type 2) (param i32 i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 2
      local.get 1
      i32.load offset=8
      local.tee 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.get 2
      i32.add
      i32.load8_u
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8 offset=1
    local.get 0
    local.get 2
    local.get 3
    i32.lt_u
    i32.store8)
  (func (;17;) (type 4) (param i32 i32) (result i64)
    (local i64 i32 i32)
    i64.const 0
    local.set 2
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 3
      local.get 1
      i32.add
      local.tee 4
      i32.const -1
      i32.add
      local.get 0
      i32.load offset=8
      i32.ge_u
      local.tee 1
      br_if 0 (;@1;)
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      i32.load
      local.get 3
      i32.add
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.set 2
    end
    local.get 2
    local.get 1
    i64.extend_i32_u
    i64.or
    i64.const 512
    i64.or)
  (func (;18;) (type 5) (param i32 i64)
    block  ;; label = @1
      local.get 1
      i32.wrap_i64
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i64.const 16
    i64.shr_u
    i64.store8 offset=1
    local.get 0
    local.get 1
    i64.const 8
    i64.shr_u
    i64.store8)
  (func (;19;) (type 6) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 1
                            local.get 2
                            call 17
                            local.tee 4
                            i32.wrap_i64
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 4
                            i64.const 1
                            i64.and
                            i64.eqz
                            i32.eqz
                            br_if 11 (;@1;)
                            local.get 4
                            i64.const 32
                            i64.shr_u
                            i32.wrap_i64
                            local.set 1
                            local.get 2
                            i32.const -1
                            i32.add
                            br_table 1 (;@11;) 2 (;@10;) 3 (;@9;) 4 (;@8;) 5 (;@7;) 6 (;@6;) 7 (;@5;) 8 (;@4;) 11 (;@1;)
                          end
                          local.get 3
                          i32.const 8
                          i32.add
                          local.get 4
                          call 18
                          local.get 0
                          local.get 3
                          i32.load16_u offset=8
                          i32.store16 offset=1 align=1
                          local.get 0
                          i32.const 1
                          i32.store8
                          br 9 (;@2;)
                        end
                        local.get 1
                        i64.load8_u
                        local.set 4
                        br 7 (;@3;)
                      end
                      local.get 1
                      i64.load8_u
                      i64.const 8
                      i64.shl
                      local.get 1
                      i64.load8_u offset=1
                      i64.or
                      local.set 4
                      br 6 (;@3;)
                    end
                    local.get 1
                    i64.load8_u offset=1
                    i64.const 8
                    i64.shl
                    local.get 1
                    i64.load8_u
                    i64.const 16
                    i64.shl
                    i64.or
                    local.get 1
                    i64.load8_u offset=2
                    i64.or
                    local.set 4
                    br 5 (;@3;)
                  end
                  local.get 1
                  i64.load8_u offset=1
                  i64.const 8
                  i64.shl
                  local.get 1
                  i64.load8_u
                  i64.const 16
                  i64.shl
                  i64.or
                  local.get 1
                  i64.load8_u offset=2
                  i64.or
                  i64.const 8
                  i64.shl
                  local.get 1
                  i64.load8_u offset=3
                  i64.or
                  local.set 4
                  br 4 (;@3;)
                end
                local.get 1
                i64.load8_u offset=3
                i64.const 8
                i64.shl
                local.get 1
                i64.load8_u offset=4
                i64.or
                local.get 1
                i64.load8_u offset=1
                i64.const 8
                i64.shl
                local.get 1
                i64.load8_u
                i64.const 16
                i64.shl
                i64.or
                local.get 1
                i64.load8_u offset=2
                i64.or
                i64.const 16
                i64.shl
                i64.or
                local.set 4
                br 3 (;@3;)
              end
              local.get 1
              i64.load8_u offset=1
              i64.const 8
              i64.shl
              local.get 1
              i64.load8_u
              i64.const 16
              i64.shl
              i64.or
              local.get 1
              i64.load8_u offset=2
              i64.or
              i64.const 24
              i64.shl
              local.get 1
              i64.load8_u offset=3
              i64.const 16
              i64.shl
              local.get 1
              i64.load8_u offset=4
              i64.const 8
              i64.shl
              i64.or
              i64.or
              local.get 1
              i64.load8_u offset=5
              i64.or
              local.set 4
              br 2 (;@3;)
            end
            local.get 1
            i64.load8_u offset=5
            i64.const 8
            i64.shl
            local.get 1
            i64.load8_u offset=6
            i64.or
            local.get 1
            i64.load8_u offset=1
            i64.const 8
            i64.shl
            local.get 1
            i64.load8_u
            i64.const 16
            i64.shl
            i64.or
            local.get 1
            i64.load8_u offset=2
            i64.or
            i64.const 32
            i64.shl
            local.get 1
            i64.load8_u offset=3
            i64.const 24
            i64.shl
            local.get 1
            i64.load8_u offset=4
            i64.const 16
            i64.shl
            i64.or
            i64.or
            i64.or
            local.set 4
            br 1 (;@3;)
          end
          local.get 1
          i64.load align=1
          local.tee 4
          i64.const 56
          i64.shl
          local.get 4
          i64.const 40
          i64.shl
          i64.const 71776119061217280
          i64.and
          i64.or
          local.get 4
          i64.const 24
          i64.shl
          i64.const 280375465082880
          i64.and
          local.get 4
          i64.const 8
          i64.shl
          i64.const 1095216660480
          i64.and
          i64.or
          i64.or
          local.get 4
          i64.const 8
          i64.shr_u
          i64.const 4278190080
          i64.and
          local.get 4
          i64.const 24
          i64.shr_u
          i64.const 16711680
          i64.and
          i64.or
          local.get 4
          i64.const 40
          i64.shr_u
          i64.const 65280
          i64.and
          local.get 4
          i64.const 56
          i64.shr_u
          i64.or
          i64.or
          i64.or
          local.set 4
        end
        local.get 0
        i32.const 0
        i32.store8
        local.get 0
        i32.const 8
        i32.add
        local.get 4
        i64.store
      end
      local.get 3
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func (;20;) (type 0) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    i32.const 1
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call 10
        local.tee 3
        i32.const 255
        i32.and
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 2
          local.get 1
          i32.const 16
          i32.add
          local.get 0
          i32.const 1
          call 19
          block  ;; label = @4
            local.get 1
            i32.load8_u offset=16
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 24
            i32.add
            i32.load8_u
            local.set 0
            i32.const 0
            local.set 2
            br 3 (;@1;)
          end
          local.get 1
          i32.load8_u offset=17
          local.set 0
          local.get 1
          i32.load8_u offset=18
          local.set 3
          br 2 (;@1;)
        end
        unreachable
        unreachable
      end
      local.get 1
      i32.const 8
      i32.add
      local.get 3
      call 12
      local.get 1
      i32.load8_u offset=9
      local.set 3
      local.get 1
      i32.load8_u offset=8
      local.set 0
    end
    local.get 1
    i32.const 32
    i32.add
    global.set 0
    local.get 0
    i32.const 255
    i32.and
    i32.const 8
    i32.shl
    local.get 3
    i32.const 16
    i32.shl
    i32.or
    local.get 2
    i32.or)
  (func (;21;) (type 2) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 16
    i32.shr_u
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.const 8
    i32.shr_u
    i32.store8)
  (func (;22;) (type 0) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    i32.const 1
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              call 10
              local.tee 3
              i32.const 255
              i32.and
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              i32.and
              br_if 4 (;@1;)
              i32.const 1
              local.set 2
              local.get 3
              i32.const 16776960
              i32.and
              i32.const 8
              i32.shr_u
              i32.const -34
              i32.add
              local.tee 3
              i32.const 4
              i32.shl
              local.get 3
              i32.const 240
              i32.and
              i32.const 4
              i32.shr_u
              i32.or
              i32.const 255
              i32.and
              br_table 2 (;@3;) 1 (;@4;) 2 (;@3;) 1 (;@4;) 4 (;@1;)
            end
            local.get 1
            i32.const 8
            i32.add
            local.get 3
            call 12
            i32.const 0
            local.set 0
            local.get 1
            i32.load8_u offset=9
            local.set 3
            local.get 1
            i32.load8_u offset=8
            local.set 4
            br 2 (;@2;)
          end
          i32.const 2
          local.set 2
        end
        local.get 1
        i32.const 16
        i32.add
        local.get 0
        local.get 2
        call 19
        block  ;; label = @3
          local.get 1
          i32.load8_u offset=16
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 24
          i32.add
          i32.load
          local.tee 3
          i32.const 65280
          i32.and
          local.set 0
          i32.const 0
          local.set 2
          br 1 (;@2;)
        end
        local.get 1
        i32.load8_u offset=17
        local.set 4
        local.get 1
        i32.load8_u offset=18
        local.set 3
        i32.const 1
        local.set 2
        i32.const 0
        local.set 0
      end
      local.get 1
      i32.const 32
      i32.add
      global.set 0
      local.get 4
      i32.const 255
      i32.and
      i32.const 8
      i32.shl
      local.get 2
      i32.or
      local.get 0
      local.get 3
      i32.const 255
      i32.and
      i32.or
      i32.const 16
      i32.shl
      i32.or
      return
    end
    unreachable
    unreachable)
  (func (;23;) (type 2) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 16
    i32.shr_u
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.const 8
    i32.shr_u
    i32.store8)
  (func (;24;) (type 7) (param i32) (result i64)
    (local i32 i32 i32 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  call 10
                  local.tee 2
                  i32.const 255
                  i32.and
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 1
                  i32.and
                  br_if 6 (;@1;)
                  i32.const 1
                  local.set 3
                  local.get 2
                  i32.const 16776960
                  i32.and
                  i32.const 8
                  i32.shr_u
                  i32.const -3
                  i32.add
                  local.tee 2
                  i32.const 4
                  i32.shl
                  local.get 2
                  i32.const 240
                  i32.and
                  i32.const 4
                  i32.shr_u
                  i32.or
                  i32.const 255
                  i32.and
                  br_table 4 (;@3;) 1 (;@6;) 2 (;@5;) 3 (;@4;) 4 (;@3;) 1 (;@6;) 2 (;@5;) 3 (;@4;) 6 (;@1;)
                end
                local.get 1
                i32.const 8
                i32.add
                local.get 2
                call 12
                i64.const 1
                local.set 4
                i64.const 0
                local.set 5
                local.get 1
                i32.load8_u offset=9
                local.set 0
                local.get 1
                i32.load8_u offset=8
                local.set 2
                br 4 (;@2;)
              end
              i32.const 2
              local.set 3
              br 2 (;@3;)
            end
            i32.const 3
            local.set 3
            br 1 (;@3;)
          end
          i32.const 4
          local.set 3
        end
        local.get 1
        i32.const 16
        i32.add
        local.get 0
        local.get 3
        call 19
        block  ;; label = @3
          local.get 1
          i32.load8_u offset=16
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 24
          i32.add
          i64.load
          i64.const 32
          i64.shl
          local.set 5
          i64.const 0
          local.set 4
          br 1 (;@2;)
        end
        local.get 1
        i32.load8_u offset=17
        local.set 2
        local.get 1
        i32.load8_u offset=18
        local.set 0
        i64.const 1
        local.set 4
        i64.const 0
        local.set 5
      end
      local.get 1
      i32.const 32
      i32.add
      global.set 0
      local.get 0
      i64.extend_i32_u
      i64.const 255
      i64.and
      i64.const 16
      i64.shl
      local.get 5
      i64.or
      local.get 4
      i64.or
      local.get 2
      i64.extend_i32_u
      i64.const 255
      i64.and
      i64.const 8
      i64.shl
      i64.or
      return
    end
    unreachable
    unreachable)
  (func (;25;) (type 5) (param i32 i64)
    block  ;; label = @1
      local.get 1
      i32.wrap_i64
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i64.const 16
    i64.shr_u
    i64.store8 offset=1
    local.get 0
    local.get 1
    i64.const 8
    i64.shr_u
    i64.store8)
  (func (;26;) (type 2) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call 10
        local.tee 3
        i32.const 255
        i32.and
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 4
          block  ;; label = @4
            local.get 3
            i32.const 8
            i32.shr_u
            i32.const 255
            i32.and
            local.tee 3
            i32.const -4
            i32.add
            i32.const 2
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        i32.const -20
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const -36
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 1 (;@9;)
                        local.get 3
                        i32.const -52
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 3
                        i32.const -68
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 3 (;@7;)
                        local.get 3
                        i32.const -84
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 4 (;@6;)
                        local.get 3
                        i32.const -100
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 5 (;@5;)
                        local.get 3
                        i32.const -116
                        i32.add
                        i32.const 2
                        i32.ge_u
                        br_if 7 (;@3;)
                        i32.const 8
                        local.set 4
                        br 6 (;@4;)
                      end
                      i32.const 2
                      local.set 4
                      br 5 (;@4;)
                    end
                    i32.const 3
                    local.set 4
                    br 4 (;@4;)
                  end
                  i32.const 4
                  local.set 4
                  br 3 (;@4;)
                end
                i32.const 5
                local.set 4
                br 2 (;@4;)
              end
              i32.const 6
              local.set 4
              br 1 (;@4;)
            end
            i32.const 7
            local.set 4
          end
          local.get 2
          i32.const 16
          i32.add
          local.get 1
          local.get 4
          call 19
          block  ;; label = @4
            local.get 2
            i32.load8_u offset=16
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 2
            i32.const 16
            i32.add
            i32.const 8
            i32.add
            i64.load
            local.set 5
            local.get 0
            i32.const 0
            i32.store8
            local.get 0
            i32.const 8
            i32.add
            local.get 5
            i64.store
            br 3 (;@1;)
          end
          local.get 0
          local.get 2
          i32.load16_u offset=17 align=1
          i32.store16 offset=1 align=1
          local.get 0
          i32.const 1
          i32.store8
          br 2 (;@1;)
        end
        unreachable
        unreachable
      end
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      call 12
      local.get 0
      local.get 2
      i32.load16_u offset=8
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 1
      i32.store8
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048577))
  (global (;2;) i32 (i32.const 1048577))
  (export "memory" (memory 0))
  (export "svm_alloc" (func 6))
  (export "initialize" (func 7))
  (export "store_addr" (func 8))
  (export "load_addr" (func 14))
  (export "svm_fund" (func 15))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
