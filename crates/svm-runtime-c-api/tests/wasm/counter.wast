(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (result i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func))
  (type (;7;) (func (param i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i64)))
  (type (;10;) (func (param i32) (result i64)))
  (type (;11;) (func (param i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;15;) (func (param i64 i32 i32) (result i32)))
  (import "svm" "calldata_ptr" (func $_ZN28svm_runtime_examples_counter12calldata_ptr17h864136ea57f5c214E (type 3)))
  (import "svm" "calldata_len" (func $_ZN28svm_runtime_examples_counter12calldata_len17ha313c05619693b0cE (type 3)))
  (import "svm" "set32" (func $_ZN28svm_runtime_examples_counter5set3217hb428f78cb6c121e0E (type 0)))
  (import "svm" "get32" (func $_ZN28svm_runtime_examples_counter5get3217h5f119dd0622f299bE (type 4)))
  (func $_ZN4core3ptr13drop_in_place17h12fe707ca2db124aE (type 5) (param i32))
  (func $svm_alloc (type 4) (param i32) (result i32)
    local.get 0
    call $_ZN7svm_sdk6memory5alloc17hc2483383b149bf9dE)
  (func $initialize (type 6)
    (local i32 i64 i32 i64 i32 i64 i32 i64)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $_ZN28svm_runtime_examples_counter12calldata_ptr17h864136ea57f5c214E
    call $_ZN28svm_runtime_examples_counter12calldata_len17ha313c05619693b0cE
    call $_ZN15svm_abi_decoder6cursor6Cursor3new17h0e77d169fdd97301E
    call $_ZN15svm_abi_decoder7decoder7Decoder3new17ha5a0df2a8b7fabfdE
    local.get 0
    i32.const 48
    i32.add
    local.get 0
    i32.const 88
    i32.add
    local.get 0
    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17hb34bcff2bed0990aE
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=48
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load16_u offset=49 align=1
      i32.store16 offset=16
      i32.const 1048576
      i32.const 43
      local.get 0
      i32.const 16
      i32.add
      i32.const 1048620
      i32.const 1048648
      call $_ZN4core6option18expect_none_failed17h807851172125790dE
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    i32.const 24
    i32.add
    local.get 0
    i32.const 80
    i32.add
    i64.load
    local.tee 1
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 48
    i32.add
    i32.const 24
    i32.add
    local.tee 2
    i64.load
    local.tee 3
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 48
    i32.add
    i32.const 16
    i32.add
    local.tee 4
    i64.load
    local.tee 5
    i64.store
    local.get 0
    local.get 0
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.tee 6
    i64.load
    local.tee 7
    i64.store offset=16
    local.get 2
    local.get 1
    i64.store
    local.get 4
    local.get 3
    i64.store
    local.get 6
    local.get 5
    i64.store
    local.get 0
    local.get 7
    i64.store offset=48
    i32.const 0
    local.get 0
    i32.const 48
    i32.add
    call $_ZN7svm_sdk5value82_$LT$impl$u20$core..convert..From$LT$svm_sdk..value..Value$GT$$u20$for$u20$u32$GT$4from17h78d7324904755594E
    call $_ZN28svm_runtime_examples_counter5set3217hb428f78cb6c121e0E
    local.get 0
    i32.const 96
    i32.add
    global.set 0)
  (func $add (type 6)
    (local i32 i64 i32 i64 i32 i64 i32 i64)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $_ZN28svm_runtime_examples_counter12calldata_ptr17h864136ea57f5c214E
    call $_ZN28svm_runtime_examples_counter12calldata_len17ha313c05619693b0cE
    call $_ZN15svm_abi_decoder6cursor6Cursor3new17h0e77d169fdd97301E
    call $_ZN15svm_abi_decoder7decoder7Decoder3new17ha5a0df2a8b7fabfdE
    local.get 0
    i32.const 48
    i32.add
    local.get 0
    i32.const 88
    i32.add
    local.get 0
    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17hb34bcff2bed0990aE
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=48
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load16_u offset=49 align=1
      i32.store16 offset=16
      i32.const 1048576
      i32.const 43
      local.get 0
      i32.const 16
      i32.add
      i32.const 1048620
      i32.const 1048664
      call $_ZN4core6option18expect_none_failed17h807851172125790dE
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    i32.const 24
    i32.add
    local.get 0
    i32.const 80
    i32.add
    i64.load
    local.tee 1
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 48
    i32.add
    i32.const 24
    i32.add
    local.tee 2
    i64.load
    local.tee 3
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 48
    i32.add
    i32.const 16
    i32.add
    local.tee 4
    i64.load
    local.tee 5
    i64.store
    local.get 0
    local.get 0
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.tee 6
    i64.load
    local.tee 7
    i64.store offset=16
    local.get 2
    local.get 1
    i64.store
    local.get 4
    local.get 3
    i64.store
    local.get 6
    local.get 5
    i64.store
    local.get 0
    local.get 7
    i64.store offset=48
    i32.const 0
    local.get 0
    i32.const 48
    i32.add
    call $_ZN7svm_sdk5value82_$LT$impl$u20$core..convert..From$LT$svm_sdk..value..Value$GT$$u20$for$u20$u32$GT$4from17h78d7324904755594E
    i32.const 0
    call $_ZN28svm_runtime_examples_counter5get3217h5f119dd0622f299bE
    i32.add
    call $_ZN28svm_runtime_examples_counter5set3217hb428f78cb6c121e0E
    local.get 0
    i32.const 96
    i32.add
    global.set 0)
  (func $__rust_alloc (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc
    local.set 2
    local.get 2
    return)
  (func $__rust_dealloc (type 7) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $__rdl_dealloc
    return)
  (func $__rust_realloc (type 8) (param i32 i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__rdl_realloc
    local.set 4
    local.get 4
    return)
  (func $__rust_alloc_zeroed (type 2) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    call $__rdl_alloc_zeroed
    local.set 2
    local.get 2
    return)
  (func $_ZN15svm_abi_decoder6cursor6Cursor3new17h0e77d169fdd97301E (type 7) (param i32 i32 i32)
    local.get 0
    i32.const 0
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h019fb32cc03cc4ccE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 0
        i32.load8_u
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.const 1048908
        i32.const 15
        call $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E
        local.get 2
        local.get 0
        i32.const 1
        i32.add
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 1048924
        call $_ZN4core3fmt8builders10DebugTuple5field17hc2035b36d8e0d2adE
        drop
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 1048940
      i32.const 15
      call $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E
    end
    local.get 2
    call $_ZN4core3fmt8builders10DebugTuple6finish17heeae06fe3ffe888bE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf0958354fd7e5680E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.const 1048955
    i32.const 14
    call $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E
    local.get 2
    call $_ZN4core3fmt8builders10DebugTuple6finish17heeae06fe3ffe888bE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3ptr13drop_in_place17hca03f35676b9831cE (type 5) (param i32))
  (func $_ZN15svm_abi_decoder7decoder7Decoder3new17ha5a0df2a8b7fabfdE (type 6))
  (func $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17hb34bcff2bed0990aE (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i32)
    global.get 0
    i32.const 240
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
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                local.get 2
                                                i32.const 8
                                                i32.add
                                                local.tee 4
                                                i32.load
                                                local.tee 5
                                                local.get 2
                                                i32.const 4
                                                i32.add
                                                i32.load
                                                local.tee 6
                                                i32.ge_u
                                                br_if 0 (;@22;)
                                                block  ;; label = @23
                                                  local.get 2
                                                  i32.load
                                                  local.tee 7
                                                  local.get 5
                                                  i32.add
                                                  local.tee 8
                                                  i32.load8_u
                                                  local.tee 9
                                                  i32.const 118
                                                  i32.gt_u
                                                  br_if 0 (;@23;)
                                                  local.get 9
                                                  br_table 3 (;@20;) 5 (;@18;) 6 (;@17;) 10 (;@13;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 3 (;@20;) 5 (;@18;) 7 (;@16;) 10 (;@13;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 4 (;@19;) 5 (;@18;) 8 (;@15;) 10 (;@13;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 5 (;@18;) 8 (;@15;) 10 (;@13;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 5 (;@18;) 9 (;@14;) 11 (;@12;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 5 (;@18;) 9 (;@14;) 11 (;@12;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 5 (;@18;) 0 (;@23;) 11 (;@12;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 0 (;@23;) 5 (;@18;) 0 (;@23;) 11 (;@12;) 12 (;@11;) 13 (;@10;) 2 (;@21;) 3 (;@20;)
                                                end
                                                i32.const 1048680
                                                i32.const 40
                                                i32.const 1048892
                                                call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                                                unreachable
                                              end
                                              local.get 0
                                              i32.const 1
                                              i32.store16
                                              br 20 (;@1;)
                                            end
                                            local.get 4
                                            local.get 5
                                            i32.const 1
                                            i32.add
                                            local.tee 10
                                            i32.store
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        local.get 9
                                                        i32.const -6
                                                        i32.add
                                                        local.tee 9
                                                        i32.const 240
                                                        i32.and
                                                        i32.const 4
                                                        i32.shr_u
                                                        local.get 9
                                                        i32.const 4
                                                        i32.shl
                                                        i32.or
                                                        local.tee 9
                                                        i32.const 255
                                                        i32.and
                                                        local.tee 8
                                                        i32.const -1
                                                        i32.add
                                                        i32.const 6
                                                        i32.lt_u
                                                        br_if 0 (;@26;)
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            local.get 8
                                                            i32.const 7
                                                            i32.gt_u
                                                            br_if 0 (;@28;)
                                                            local.get 8
                                                            br_table 5 (;@23;) 0 (;@28;) 0 (;@28;) 0 (;@28;) 0 (;@28;) 0 (;@28;) 0 (;@28;) 1 (;@27;) 5 (;@23;)
                                                          end
                                                          i32.const 1048680
                                                          i32.const 40
                                                          i32.const 1048876
                                                          call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                                                          unreachable
                                                        end
                                                        local.get 10
                                                        local.get 6
                                                        i32.lt_u
                                                        br_if 1 (;@25;)
                                                        i32.const 2
                                                        local.set 9
                                                        local.get 4
                                                        local.get 5
                                                        i32.const 2
                                                        i32.add
                                                        i32.store
                                                        i32.const 0
                                                        local.set 4
                                                        br 24 (;@2;)
                                                      end
                                                      local.get 3
                                                      i32.const 112
                                                      i32.add
                                                      local.set 11
                                                      br 1 (;@24;)
                                                    end
                                                    local.get 7
                                                    local.get 10
                                                    i32.add
                                                    i32.load8_u
                                                    local.set 9
                                                    local.get 4
                                                    local.get 5
                                                    i32.const 2
                                                    i32.add
                                                    i32.store
                                                    local.get 3
                                                    i32.const 112
                                                    i32.add
                                                    local.set 11
                                                    local.get 9
                                                    i32.eqz
                                                    br_if 2 (;@22;)
                                                  end
                                                  local.get 9
                                                  i32.const 255
                                                  i32.and
                                                  local.tee 12
                                                  i32.const 5
                                                  i32.shl
                                                  local.tee 4
                                                  i32.const 8
                                                  call $__rust_alloc
                                                  local.tee 5
                                                  br_if 2 (;@21;)
                                                  local.get 4
                                                  i32.const 8
                                                  call $_ZN5alloc5alloc18handle_alloc_error17he62175e02bb3fceeE
                                                  unreachable
                                                end
                                                local.get 3
                                                i32.const 112
                                                i32.add
                                                local.set 11
                                              end
                                              local.get 3
                                              i32.const 0
                                              i32.store offset=120
                                              local.get 3
                                              i64.const 8
                                              i64.store offset=112
                                              br 17 (;@4;)
                                            end
                                            i32.const 0
                                            local.set 8
                                            local.get 3
                                            i32.const 0
                                            i32.store offset=120
                                            local.get 3
                                            local.get 12
                                            i32.store offset=116
                                            local.get 3
                                            local.get 5
                                            i32.store offset=112
                                            local.get 3
                                            i32.const 160
                                            i32.add
                                            i32.const 5
                                            i32.add
                                            local.set 4
                                            local.get 3
                                            i32.const 200
                                            i32.add
                                            i32.const 3
                                            i32.or
                                            local.set 5
                                            block  ;; label = @21
                                              loop  ;; label = @22
                                                local.get 3
                                                i32.const 200
                                                i32.add
                                                local.get 1
                                                local.get 2
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17hb34bcff2bed0990aE
                                                local.get 3
                                                i32.const 160
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 5
                                                i32.const 8
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 3
                                                i32.const 160
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 5
                                                i32.const 16
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 3
                                                i32.const 160
                                                i32.add
                                                i32.const 24
                                                i32.add
                                                local.get 5
                                                i32.const 24
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 3
                                                i32.const 160
                                                i32.add
                                                i32.const 29
                                                i32.add
                                                local.get 5
                                                i32.const 29
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 3
                                                local.get 5
                                                i64.load align=1
                                                i64.store offset=160
                                                local.get 3
                                                i32.load8_u offset=200
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 8
                                                i32.const 1
                                                i32.add
                                                local.set 8
                                                local.get 3
                                                i32.const 128
                                                i32.add
                                                i32.const 24
                                                i32.add
                                                local.get 4
                                                i32.const 24
                                                i32.add
                                                i64.load align=1
                                                local.tee 13
                                                i64.store
                                                local.get 3
                                                i32.const 128
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 4
                                                i32.const 16
                                                i32.add
                                                i64.load align=1
                                                local.tee 14
                                                i64.store
                                                local.get 3
                                                i32.const 128
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 4
                                                i32.const 8
                                                i32.add
                                                i64.load align=1
                                                local.tee 15
                                                i64.store
                                                local.get 3
                                                local.get 4
                                                i64.load align=1
                                                local.tee 16
                                                i64.store offset=128
                                                local.get 3
                                                i32.const 200
                                                i32.add
                                                i32.const 24
                                                i32.add
                                                local.tee 7
                                                local.get 13
                                                i64.store
                                                local.get 3
                                                i32.const 200
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 10
                                                local.get 14
                                                i64.store
                                                local.get 3
                                                i32.const 200
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 17
                                                local.get 15
                                                i64.store
                                                local.get 3
                                                local.get 16
                                                i64.store offset=200
                                                block  ;; label = @23
                                                  local.get 3
                                                  i32.load offset=120
                                                  local.tee 9
                                                  local.get 3
                                                  i32.load offset=116
                                                  i32.ne
                                                  br_if 0 (;@23;)
                                                  local.get 3
                                                  i32.const 112
                                                  i32.add
                                                  local.get 9
                                                  i32.const 1
                                                  call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h9af60457896686dcE
                                                  local.get 3
                                                  i32.load offset=120
                                                  local.set 9
                                                end
                                                local.get 3
                                                i32.load offset=112
                                                local.get 9
                                                i32.const 5
                                                i32.shl
                                                i32.add
                                                local.tee 6
                                                local.get 3
                                                i64.load offset=200
                                                i64.store
                                                local.get 6
                                                i32.const 8
                                                i32.add
                                                local.get 17
                                                i64.load
                                                i64.store
                                                local.get 6
                                                i32.const 16
                                                i32.add
                                                local.get 10
                                                i64.load
                                                i64.store
                                                local.get 6
                                                i32.const 24
                                                i32.add
                                                local.get 7
                                                i64.load
                                                i64.store
                                                local.get 3
                                                local.get 9
                                                i32.const 1
                                                i32.add
                                                i32.store offset=120
                                                local.get 8
                                                i32.const 255
                                                i32.and
                                                local.get 12
                                                i32.ge_u
                                                br_if 18 (;@4;)
                                                br 0 (;@22;)
                                              end
                                            end
                                            local.get 3
                                            i32.load8_u offset=202
                                            local.set 4
                                            local.get 3
                                            i32.load8_u offset=201
                                            local.set 9
                                            block  ;; label = @21
                                              local.get 3
                                              i32.load offset=120
                                              local.tee 2
                                              i32.eqz
                                              br_if 0 (;@21;)
                                              local.get 3
                                              i32.load offset=112
                                              local.set 5
                                              local.get 2
                                              i32.const 5
                                              i32.shl
                                              local.set 2
                                              loop  ;; label = @22
                                                block  ;; label = @23
                                                  local.get 5
                                                  i32.load
                                                  i32.eqz
                                                  br_if 0 (;@23;)
                                                  local.get 5
                                                  i32.const 4
                                                  i32.add
                                                  call $_ZN4core3ptr13drop_in_place17hb3c7307c35e6ee0bE.llvm.6848653632164620884
                                                end
                                                local.get 5
                                                i32.const 32
                                                i32.add
                                                local.set 5
                                                local.get 2
                                                i32.const -32
                                                i32.add
                                                local.tee 2
                                                br_if 0 (;@22;)
                                              end
                                            end
                                            local.get 3
                                            i32.load offset=116
                                            local.tee 5
                                            i32.eqz
                                            br_if 18 (;@2;)
                                            local.get 5
                                            i32.const 5
                                            i32.shl
                                            local.tee 5
                                            i32.eqz
                                            br_if 18 (;@2;)
                                            local.get 3
                                            i32.load offset=112
                                            local.get 5
                                            i32.const 8
                                            call $__rust_dealloc
                                            br 18 (;@2;)
                                          end
                                          local.get 4
                                          local.get 5
                                          i32.const 1
                                          i32.add
                                          i32.store
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                local.get 9
                                                i32.const 16
                                                i32.gt_u
                                                br_if 0 (;@22;)
                                                i32.const 0
                                                local.set 5
                                                local.get 9
                                                br_table 2 (;@20;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 0 (;@22;) 1 (;@21;) 2 (;@20;)
                                              end
                                              i32.const 1048680
                                              i32.const 40
                                              i32.const 1048796
                                              call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                                              unreachable
                                            end
                                            i32.const 1
                                            local.set 5
                                          end
                                          local.get 3
                                          i32.const 72
                                          i32.add
                                          local.get 5
                                          call $_ZN73_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$bool$GT$$GT$4from17h9b9a526a4e733139E
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 13
                                          i32.add
                                          local.get 3
                                          i32.const 72
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          i64.load
                                          i64.store align=1
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 21
                                          i32.add
                                          local.get 3
                                          i32.const 72
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          i64.load
                                          i64.store align=1
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 29
                                          i32.add
                                          local.get 3
                                          i32.const 72
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          i64.load
                                          local.tee 13
                                          i64.store align=1
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 29
                                          i32.add
                                          local.tee 5
                                          local.get 13
                                          i64.store align=1
                                          local.get 3
                                          local.get 3
                                          i64.load offset=72
                                          i64.store offset=205 align=1
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          local.get 3
                                          i32.const 200
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          local.get 3
                                          i64.load offset=200 align=1
                                          i64.store offset=160
                                          local.get 3
                                          i32.const 24
                                          i32.add
                                          local.get 5
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          i32.const 16
                                          i32.add
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 21
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          i32.const 8
                                          i32.add
                                          local.get 3
                                          i32.const 160
                                          i32.add
                                          i32.const 13
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 3
                                          local.get 3
                                          i64.load offset=165 align=1
                                          i64.store
                                          br 16 (;@3;)
                                        end
                                        i32.const 1
                                        local.set 2
                                        local.get 4
                                        local.get 5
                                        i32.const 1
                                        i32.add
                                        local.tee 8
                                        i32.store
                                        block  ;; label = @19
                                          local.get 5
                                          i32.const 20
                                          i32.add
                                          local.get 6
                                          i32.ge_u
                                          br_if 0 (;@19;)
                                          local.get 4
                                          local.get 5
                                          i32.const 21
                                          i32.add
                                          i32.store
                                          local.get 7
                                          local.get 8
                                          i32.add
                                          local.set 9
                                          i32.const 0
                                          local.set 2
                                        end
                                        local.get 3
                                        i32.const 32
                                        i32.add
                                        i32.const 2
                                        i32.add
                                        local.tee 5
                                        local.get 3
                                        i32.const 72
                                        i32.add
                                        i32.const 2
                                        i32.add
                                        i32.load8_u
                                        i32.store8
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.tee 4
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load
                                        i64.store
                                        local.get 3
                                        local.get 3
                                        i32.load16_u offset=72 align=1
                                        i32.store16 offset=32
                                        local.get 3
                                        local.get 3
                                        i64.load offset=200
                                        i64.store offset=160
                                        block  ;; label = @19
                                          local.get 2
                                          br_if 0 (;@19;)
                                          local.get 3
                                          i32.const 8
                                          i32.add
                                          i32.const 1
                                          i32.store8
                                          local.get 3
                                          i32.const 12
                                          i32.add
                                          local.get 9
                                          i32.store
                                          local.get 3
                                          i32.const 16
                                          i32.add
                                          local.get 3
                                          i64.load offset=160
                                          i64.store
                                          local.get 3
                                          i32.const 11
                                          i32.add
                                          local.get 5
                                          i32.load8_u
                                          i32.store8
                                          local.get 3
                                          i32.const 24
                                          i32.add
                                          local.get 4
                                          i64.load
                                          i64.store
                                          local.get 3
                                          i32.const 0
                                          i32.store
                                          local.get 3
                                          local.get 3
                                          i32.load16_u offset=32
                                          i32.store16 offset=9 align=1
                                          br 16 (;@3;)
                                        end
                                        local.get 0
                                        i32.const 513
                                        i32.store16
                                        local.get 0
                                        i32.const 2
                                        i32.add
                                        i32.const 0
                                        i32.store8
                                        br 17 (;@1;)
                                      end
                                      local.get 4
                                      local.get 5
                                      i32.const 1
                                      i32.add
                                      local.tee 1
                                      i32.store
                                      local.get 9
                                      i32.const -1
                                      i32.add
                                      local.tee 2
                                      i32.const 240
                                      i32.and
                                      i32.const 4
                                      i32.shr_u
                                      local.get 2
                                      i32.const 4
                                      i32.shl
                                      i32.or
                                      i32.const 255
                                      i32.and
                                      local.tee 2
                                      i32.const 8
                                      i32.ge_u
                                      br_if 8 (;@9;)
                                      block  ;; label = @18
                                        local.get 2
                                        i32.const 1
                                        i32.add
                                        local.tee 9
                                        local.get 5
                                        i32.add
                                        local.get 6
                                        i32.ge_u
                                        br_if 0 (;@18;)
                                        local.get 4
                                        local.get 9
                                        local.get 1
                                        i32.add
                                        i32.store
                                        local.get 2
                                        i32.const 1
                                        i32.add
                                        local.set 2
                                        local.get 8
                                        i32.const 1
                                        i32.add
                                        local.set 5
                                        i64.const 0
                                        local.set 13
                                        loop  ;; label = @19
                                          local.get 13
                                          i64.const 8
                                          i64.shl
                                          local.get 5
                                          i64.load8_u
                                          i64.or
                                          local.set 13
                                          local.get 5
                                          i32.const 1
                                          i32.add
                                          local.set 5
                                          local.get 2
                                          i32.const -1
                                          i32.add
                                          local.tee 2
                                          br_if 0 (;@19;)
                                        end
                                        local.get 3
                                        i32.const 72
                                        i32.add
                                        local.get 13
                                        call $_ZN92_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$svm_sdk..amount..Amount$GT$$GT$4from17h8d6665680edc889bE
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 13
                                        i32.add
                                        local.get 3
                                        i32.const 72
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load
                                        i64.store align=1
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 21
                                        i32.add
                                        local.get 3
                                        i32.const 72
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.load
                                        i64.store align=1
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 29
                                        i32.add
                                        local.get 3
                                        i32.const 72
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        i64.load
                                        local.tee 13
                                        i64.store align=1
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 29
                                        i32.add
                                        local.tee 5
                                        local.get 13
                                        i64.store align=1
                                        local.get 3
                                        local.get 3
                                        i64.load offset=72
                                        i64.store offset=205 align=1
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.get 3
                                        i32.const 200
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        local.get 3
                                        i64.load offset=200 align=1
                                        i64.store offset=160
                                        local.get 3
                                        i32.const 24
                                        i32.add
                                        local.get 5
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        i32.const 16
                                        i32.add
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 21
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        i32.const 8
                                        i32.add
                                        local.get 3
                                        i32.const 160
                                        i32.add
                                        i32.const 13
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 3
                                        local.get 3
                                        i64.load offset=165 align=1
                                        i64.store
                                        br 15 (;@3;)
                                      end
                                      local.get 0
                                      i32.const 513
                                      i32.store16
                                      local.get 0
                                      i32.const 2
                                      i32.add
                                      i32.const 0
                                      i32.store8
                                      br 16 (;@1;)
                                    end
                                    local.get 4
                                    local.get 5
                                    i32.const 1
                                    i32.add
                                    local.tee 2
                                    i32.store
                                    block  ;; label = @17
                                      local.get 2
                                      local.get 6
                                      i32.ge_u
                                      br_if 0 (;@17;)
                                      local.get 4
                                      local.get 5
                                      i32.const 2
                                      i32.add
                                      i32.store
                                      local.get 3
                                      local.get 7
                                      local.get 2
                                      i32.add
                                      i32.load8_u
                                      call $_ZN71_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i8$GT$$GT$4from17h164afcb92ea98a32E
                                      br 14 (;@3;)
                                    end
                                    local.get 0
                                    i32.const 513
                                    i32.store16
                                    local.get 0
                                    i32.const 2
                                    i32.add
                                    i32.const 0
                                    i32.store8
                                    br 15 (;@1;)
                                  end
                                  local.get 4
                                  local.get 5
                                  i32.const 1
                                  i32.add
                                  local.tee 2
                                  i32.store
                                  block  ;; label = @16
                                    local.get 2
                                    local.get 6
                                    i32.ge_u
                                    br_if 0 (;@16;)
                                    local.get 4
                                    local.get 5
                                    i32.const 2
                                    i32.add
                                    i32.store
                                    local.get 3
                                    local.get 7
                                    local.get 2
                                    i32.add
                                    i32.load8_u
                                    call $_ZN71_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u8$GT$$GT$4from17h5ee2529d5a7a0eddE
                                    br 13 (;@3;)
                                  end
                                  local.get 0
                                  i32.const 513
                                  i32.store16
                                  local.get 0
                                  i32.const 2
                                  i32.add
                                  i32.const 0
                                  i32.store8
                                  br 14 (;@1;)
                                end
                                local.get 4
                                local.get 5
                                i32.const 1
                                i32.add
                                local.tee 1
                                i32.store
                                local.get 9
                                i32.const -34
                                i32.add
                                local.tee 2
                                i32.const 240
                                i32.and
                                i32.const 4
                                i32.shr_u
                                local.get 2
                                i32.const 4
                                i32.shl
                                i32.or
                                local.tee 2
                                i32.const 255
                                i32.and
                                i32.const 4
                                i32.ge_u
                                br_if 6 (;@8;)
                                block  ;; label = @15
                                  local.get 2
                                  i32.const 24
                                  i32.shl
                                  i32.const 24
                                  i32.shr_s
                                  i32.const 2
                                  i32.shl
                                  i32.const 1049012
                                  i32.add
                                  i32.load
                                  local.tee 2
                                  local.get 5
                                  i32.add
                                  local.get 6
                                  i32.ge_u
                                  br_if 0 (;@15;)
                                  local.get 4
                                  local.get 2
                                  local.get 1
                                  i32.add
                                  i32.store
                                  local.get 8
                                  i32.const 1
                                  i32.add
                                  local.set 5
                                  i64.const 0
                                  local.set 13
                                  loop  ;; label = @16
                                    local.get 13
                                    i64.const 8
                                    i64.shl
                                    local.get 5
                                    i64.load8_u
                                    i64.or
                                    local.set 13
                                    local.get 5
                                    i32.const 1
                                    i32.add
                                    local.set 5
                                    local.get 2
                                    i32.const -1
                                    i32.add
                                    local.tee 2
                                    br_if 0 (;@16;)
                                  end
                                  local.get 3
                                  local.get 13
                                  i32.wrap_i64
                                  call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i16$GT$$GT$4from17hd1e6afd39b18022dE
                                  br 12 (;@3;)
                                end
                                local.get 0
                                i32.const 513
                                i32.store16
                                local.get 0
                                i32.const 2
                                i32.add
                                i32.const 0
                                i32.store8
                                br 13 (;@1;)
                              end
                              local.get 4
                              local.get 5
                              i32.const 1
                              i32.add
                              local.tee 1
                              i32.store
                              local.get 9
                              i32.const -34
                              i32.add
                              local.tee 2
                              i32.const 240
                              i32.and
                              i32.const 4
                              i32.shr_u
                              local.get 2
                              i32.const 4
                              i32.shl
                              i32.or
                              local.tee 2
                              i32.const 255
                              i32.and
                              i32.const 4
                              i32.ge_u
                              br_if 6 (;@7;)
                              block  ;; label = @14
                                local.get 2
                                i32.const 24
                                i32.shl
                                i32.const 24
                                i32.shr_s
                                i32.const 2
                                i32.shl
                                i32.const 1049012
                                i32.add
                                i32.load
                                local.tee 2
                                local.get 5
                                i32.add
                                local.get 6
                                i32.ge_u
                                br_if 0 (;@14;)
                                local.get 4
                                local.get 2
                                local.get 1
                                i32.add
                                i32.store
                                local.get 8
                                i32.const 1
                                i32.add
                                local.set 5
                                i64.const 0
                                local.set 13
                                loop  ;; label = @15
                                  local.get 13
                                  i64.const 8
                                  i64.shl
                                  local.get 5
                                  i64.load8_u
                                  i64.or
                                  local.set 13
                                  local.get 5
                                  i32.const 1
                                  i32.add
                                  local.set 5
                                  local.get 2
                                  i32.const -1
                                  i32.add
                                  local.tee 2
                                  br_if 0 (;@15;)
                                end
                                local.get 3
                                local.get 13
                                i32.wrap_i64
                                call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u16$GT$$GT$4from17h75b0139ac3f91186E
                                br 11 (;@3;)
                              end
                              local.get 0
                              i32.const 513
                              i32.store16
                              local.get 0
                              i32.const 2
                              i32.add
                              i32.const 0
                              i32.store8
                              br 12 (;@1;)
                            end
                            local.get 4
                            local.get 5
                            i32.const 1
                            i32.add
                            local.tee 1
                            i32.store
                            local.get 9
                            i32.const -3
                            i32.add
                            local.tee 2
                            i32.const 240
                            i32.and
                            i32.const 4
                            i32.shr_u
                            local.get 2
                            i32.const 4
                            i32.shl
                            i32.or
                            local.tee 2
                            i32.const 255
                            i32.and
                            i32.const 8
                            i32.ge_u
                            br_if 6 (;@6;)
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 2
                                i32.const 24
                                i32.shl
                                i32.const 24
                                i32.shr_s
                                i32.const 2
                                i32.shl
                                i32.const 1049028
                                i32.add
                                i32.load
                                local.tee 2
                                local.get 5
                                i32.add
                                local.get 6
                                i32.lt_u
                                br_if 0 (;@14;)
                                i64.const 1
                                local.set 14
                                i64.const 512
                                local.set 13
                                br 1 (;@13;)
                              end
                              local.get 4
                              local.get 2
                              local.get 1
                              i32.add
                              i32.store
                              local.get 8
                              i32.const 1
                              i32.add
                              local.set 5
                              i64.const 0
                              local.set 13
                              loop  ;; label = @14
                                local.get 13
                                i64.const 8
                                i64.shl
                                local.get 5
                                i64.load8_u
                                i64.or
                                local.set 13
                                local.get 5
                                i32.const 1
                                i32.add
                                local.set 5
                                local.get 2
                                i32.const -1
                                i32.add
                                local.tee 2
                                br_if 0 (;@14;)
                              end
                              local.get 13
                              i64.const 32
                              i64.shl
                              i64.const 512
                              i64.or
                              local.set 13
                              i64.const 0
                              local.set 14
                            end
                            block  ;; label = @13
                              local.get 14
                              local.get 13
                              i64.or
                              i32.wrap_i64
                              i32.const 255
                              i32.and
                              i32.const 1
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 13
                              i64.const 32
                              i64.shr_u
                              i32.wrap_i64
                              call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i32$GT$$GT$4from17hb24425081e86a431E
                              br 10 (;@3;)
                            end
                            local.get 0
                            i32.const 513
                            i32.store16
                            local.get 0
                            i32.const 2
                            i32.add
                            i32.const 0
                            i32.store8
                            br 11 (;@1;)
                          end
                          local.get 4
                          local.get 5
                          i32.const 1
                          i32.add
                          local.tee 1
                          i32.store
                          local.get 9
                          i32.const -3
                          i32.add
                          local.tee 2
                          i32.const 240
                          i32.and
                          i32.const 4
                          i32.shr_u
                          local.get 2
                          i32.const 4
                          i32.shl
                          i32.or
                          local.tee 2
                          i32.const 255
                          i32.and
                          i32.const 8
                          i32.ge_u
                          br_if 6 (;@5;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 2
                              i32.const 24
                              i32.shl
                              i32.const 24
                              i32.shr_s
                              i32.const 2
                              i32.shl
                              i32.const 1049028
                              i32.add
                              i32.load
                              local.tee 2
                              local.get 5
                              i32.add
                              local.get 6
                              i32.lt_u
                              br_if 0 (;@13;)
                              i64.const 512
                              local.set 13
                              i64.const 1
                              local.set 14
                              br 1 (;@12;)
                            end
                            local.get 4
                            local.get 2
                            local.get 1
                            i32.add
                            i32.store
                            local.get 8
                            i32.const 1
                            i32.add
                            local.set 5
                            i64.const 0
                            local.set 13
                            loop  ;; label = @13
                              local.get 13
                              i64.const 8
                              i64.shl
                              local.get 5
                              i64.load8_u
                              i64.or
                              local.set 13
                              local.get 5
                              i32.const 1
                              i32.add
                              local.set 5
                              local.get 2
                              i32.const -1
                              i32.add
                              local.tee 2
                              br_if 0 (;@13;)
                            end
                            local.get 13
                            i64.const 32
                            i64.shl
                            i64.const 512
                            i64.or
                            local.set 13
                            i64.const 0
                            local.set 14
                          end
                          block  ;; label = @12
                            local.get 13
                            local.get 14
                            i64.or
                            i32.wrap_i64
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 3
                            local.get 13
                            i64.const 32
                            i64.shr_u
                            i32.wrap_i64
                            call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u32$GT$$GT$4from17h20124cb092f508c4E
                            br 9 (;@3;)
                          end
                          local.get 0
                          i32.const 513
                          i32.store16
                          local.get 0
                          i32.const 2
                          i32.add
                          i32.const 0
                          i32.store8
                          br 10 (;@1;)
                        end
                        local.get 3
                        i32.const 200
                        i32.add
                        local.get 2
                        call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h3cbc5d22a1b960d9E
                        block  ;; label = @11
                          local.get 3
                          i32.load8_u offset=200
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 3
                          local.get 3
                          i32.const 208
                          i32.add
                          i64.load
                          call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i64$GT$$GT$4from17h2e190c8cbb969be7E
                          br 8 (;@3;)
                        end
                        local.get 3
                        i32.load8_u offset=202
                        local.set 5
                        local.get 0
                        local.get 3
                        i32.load8_u offset=201
                        i32.store8 offset=1
                        local.get 0
                        i32.const 1
                        i32.store8
                        local.get 0
                        i32.const 2
                        i32.add
                        local.get 5
                        i32.store8
                        br 9 (;@1;)
                      end
                      local.get 3
                      i32.const 200
                      i32.add
                      local.get 2
                      call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h3cbc5d22a1b960d9E
                      block  ;; label = @10
                        local.get 3
                        i32.load8_u offset=200
                        i32.const 1
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 3
                        local.get 3
                        i32.const 208
                        i32.add
                        i64.load
                        call $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u64$GT$$GT$4from17h2a26227943144d2fE
                        br 7 (;@3;)
                      end
                      local.get 3
                      i32.load8_u offset=202
                      local.set 5
                      local.get 0
                      local.get 3
                      i32.load8_u offset=201
                      i32.store8 offset=1
                      local.get 0
                      i32.const 1
                      i32.store8
                      local.get 0
                      i32.const 2
                      i32.add
                      local.get 5
                      i32.store8
                      br 8 (;@1;)
                    end
                    i32.const 1048680
                    i32.const 40
                    i32.const 1048812
                    call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                    unreachable
                  end
                  i32.const 1048680
                  i32.const 40
                  i32.const 1048828
                  call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                  unreachable
                end
                i32.const 1048680
                i32.const 40
                i32.const 1048828
                call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                unreachable
              end
              i32.const 1048680
              i32.const 40
              i32.const 1048844
              call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
              unreachable
            end
            i32.const 1048680
            i32.const 40
            i32.const 1048844
            call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
            unreachable
          end
          local.get 3
          i32.const 160
          i32.add
          i32.const 8
          i32.add
          local.get 11
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 3
          local.get 11
          i64.load align=4
          i64.store offset=160
          local.get 3
          i32.const 200
          i32.add
          local.get 3
          i32.const 160
          i32.add
          call $_ZN113_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$svm_sdk..value..Value$GT$$GT$$GT$4from17h92fee2de9c646ff8E
          local.get 3
          i32.const 72
          i32.add
          i32.const 13
          i32.add
          local.get 3
          i32.const 200
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store align=1
          local.get 3
          i32.const 72
          i32.add
          i32.const 21
          i32.add
          local.get 3
          i32.const 200
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store align=1
          local.get 3
          i32.const 72
          i32.add
          i32.const 29
          i32.add
          local.get 3
          i32.const 200
          i32.add
          i32.const 24
          i32.add
          i64.load
          local.tee 13
          i64.store align=1
          local.get 3
          i32.const 32
          i32.add
          i32.const 29
          i32.add
          local.tee 5
          local.get 13
          i64.store align=1
          local.get 3
          local.get 3
          i64.load offset=200
          i64.store offset=77 align=1
          local.get 3
          i32.const 32
          i32.add
          i32.const 8
          i32.add
          local.get 3
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          i64.load align=1
          i64.store
          local.get 3
          i32.const 32
          i32.add
          i32.const 16
          i32.add
          local.get 3
          i32.const 72
          i32.add
          i32.const 16
          i32.add
          i64.load align=1
          i64.store
          local.get 3
          i32.const 32
          i32.add
          i32.const 24
          i32.add
          local.get 3
          i32.const 72
          i32.add
          i32.const 24
          i32.add
          i64.load align=1
          i64.store
          local.get 3
          local.get 3
          i64.load offset=72 align=1
          i64.store offset=32
          local.get 3
          i32.const 24
          i32.add
          local.get 5
          i64.load align=1
          i64.store
          local.get 3
          i32.const 16
          i32.add
          local.get 3
          i32.const 32
          i32.add
          i32.const 21
          i32.add
          i64.load align=1
          i64.store
          local.get 3
          i32.const 8
          i32.add
          local.get 3
          i32.const 32
          i32.add
          i32.const 13
          i32.add
          i64.load align=1
          i64.store
          local.get 3
          local.get 3
          i64.load offset=37 align=1
          i64.store
        end
        local.get 0
        i32.const 0
        i32.store8
        local.get 0
        i32.const 8
        i32.add
        local.get 3
        local.tee 5
        i64.load
        i64.store
        local.get 0
        i32.const 32
        i32.add
        local.get 5
        i32.const 24
        i32.add
        i64.load
        i64.store
        local.get 0
        i32.const 24
        i32.add
        local.get 5
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 0
        i32.const 16
        i32.add
        local.get 5
        i32.const 8
        i32.add
        i64.load
        i64.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 9
      i32.store8 offset=1
      local.get 0
      i32.const 1
      i32.store8
      local.get 0
      i32.const 2
      i32.add
      local.get 4
      i32.store8
    end
    local.get 3
    i32.const 240
    i32.add
    global.set 0)
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h3cbc5d22a1b960d9E (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64)
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
                        local.get 1
                        i32.const 8
                        i32.add
                        local.tee 2
                        i32.load
                        local.tee 3
                        local.get 1
                        i32.const 4
                        i32.add
                        i32.load
                        local.tee 4
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        i32.load
                        local.get 3
                        i32.add
                        local.tee 5
                        i32.load8_u
                        local.set 6
                        i32.const 1
                        local.set 7
                        local.get 2
                        local.get 3
                        i32.const 1
                        i32.add
                        local.tee 8
                        i32.store
                        local.get 6
                        i32.const -4
                        i32.add
                        local.tee 2
                        i32.const 113
                        i32.gt_u
                        br_if 1 (;@9;)
                        local.get 2
                        br_table 9 (;@1;) 9 (;@1;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 2 (;@8;) 2 (;@8;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 3 (;@7;) 3 (;@7;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 4 (;@6;) 4 (;@6;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 5 (;@5;) 5 (;@5;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 6 (;@4;) 6 (;@4;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 7 (;@3;) 7 (;@3;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 1 (;@9;) 8 (;@2;) 8 (;@2;) 9 (;@1;)
                      end
                      local.get 0
                      i32.const 2
                      i32.store16 offset=1 align=1
                      local.get 2
                      local.get 3
                      i32.const 1
                      i32.add
                      i32.store
                      local.get 0
                      i32.const 1
                      i32.store8
                      return
                    end
                    i32.const 1048680
                    i32.const 40
                    i32.const 1048860
                    call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
                    unreachable
                  end
                  i32.const 2
                  local.set 7
                  br 6 (;@1;)
                end
                i32.const 3
                local.set 7
                br 5 (;@1;)
              end
              i32.const 4
              local.set 7
              br 4 (;@1;)
            end
            i32.const 5
            local.set 7
            br 3 (;@1;)
          end
          i32.const 6
          local.set 7
          br 2 (;@1;)
        end
        i32.const 7
        local.set 7
        br 1 (;@1;)
      end
      i32.const 8
      local.set 7
    end
    block  ;; label = @1
      local.get 7
      local.get 3
      i32.add
      local.get 4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 8
      i32.add
      local.get 7
      local.get 8
      i32.add
      i32.store
      local.get 5
      i32.const 1
      i32.add
      local.set 1
      i64.const 0
      local.set 9
      loop  ;; label = @2
        local.get 9
        i64.const 8
        i64.shl
        local.get 1
        i64.load8_u
        i64.or
        local.set 9
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 7
        i32.const -1
        i32.add
        local.tee 7
        br_if 0 (;@2;)
      end
      local.get 0
      i32.const 8
      i32.add
      local.get 9
      i64.store
      local.get 0
      i32.const 0
      i32.store8
      return
    end
    local.get 0
    i32.const 2
    i32.store16 offset=1 align=1
    local.get 0
    i32.const 1
    i32.store8)
  (func $_ZN74_$LT$svm_abi_decoder..decoder..DecodeError$u20$as$u20$core..fmt..Debug$GT$3fmt17h9388a80901b01355E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 2
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.const 1048969
        i32.const 5
        call $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 1048976
        call $_ZN4core3fmt8builders10DebugTuple5field17hc2035b36d8e0d2adE
        drop
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 1048992
      i32.const 4
      call $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.const 1048996
      call $_ZN4core3fmt8builders10DebugTuple5field17hc2035b36d8e0d2adE
      drop
    end
    local.get 2
    call $_ZN4core3fmt8builders10DebugTuple6finish17heeae06fe3ffe888bE
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3ptr13drop_in_place17hb3c7307c35e6ee0bE.llvm.6848653632164620884 (type 5) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.set 2
        local.get 1
        i32.const 5
        i32.shl
        local.set 1
        loop  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 4
            i32.add
            call $_ZN4core3ptr13drop_in_place17hb3c7307c35e6ee0bE.llvm.6848653632164620884
          end
          local.get 2
          i32.const 32
          i32.add
          local.set 2
          local.get 1
          i32.const -32
          i32.add
          local.tee 1
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const 5
      i32.shl
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 2
      i32.const 8
      call $__rust_dealloc
    end)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h71c1f598a49bac44E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      local.get 1
      call $_ZN4core3fmt9Formatter15debug_lower_hex17h849604c366b47f68E
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        call $_ZN4core3fmt9Formatter15debug_upper_hex17ha3884c60210b1dfeE
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h59d71fee3c1b5474E
        return
      end
      local.get 0
      local.get 1
      call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17he0978d4c9e80e139E
      return
    end
    local.get 0
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hf65aa2a60facf14eE)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve17h9af60457896686dcE (type 7) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 3
      local.get 1
      i32.sub
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              i32.shl
              local.tee 1
              local.get 2
              local.get 1
              local.get 2
              i32.gt_u
              select
              local.tee 1
              i32.const 4
              local.get 1
              i32.const 4
              i32.gt_u
              select
              local.tee 1
              i32.const 134217727
              i32.and
              local.tee 4
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 5
              i32.shl
              local.tee 2
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 4
              local.get 1
              i32.eq
              i32.const 3
              i32.shl
              local.set 4
              block  ;; label = @6
                local.get 0
                i32.load
                i32.const 0
                local.get 3
                select
                local.tee 1
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 4
                  call $__rust_alloc
                  local.tee 1
                  br_if 5 (;@2;)
                  br 4 (;@3;)
                end
                local.get 4
                local.set 1
                br 4 (;@2;)
              end
              local.get 3
              i32.const 5
              i32.shl
              local.tee 3
              local.get 2
              i32.eq
              br_if 3 (;@2;)
              block  ;; label = @6
                local.get 3
                br_if 0 (;@6;)
                local.get 2
                br_if 2 (;@4;)
                i32.const 8
                local.set 1
                br 4 (;@2;)
              end
              local.get 1
              local.get 3
              i32.const 8
              local.get 2
              call $__rust_realloc
              local.tee 1
              i32.eqz
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            call $_ZN5alloc7raw_vec17capacity_overflow17ha369776b076180eeE
            unreachable
          end
          local.get 2
          i32.const 8
          call $__rust_alloc
          local.tee 1
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 4
        call $_ZN5alloc5alloc18handle_alloc_error17he62175e02bb3fceeE
        unreachable
      end
      local.get 0
      local.get 1
      i32.store
      local.get 0
      i32.const 4
      i32.add
      local.get 2
      i32.const 5
      i32.shr_u
      i32.store
    end)
  (func $_ZN113_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$svm_sdk..value..Value$GT$$GT$$GT$4from17h92fee2de9c646ff8E (type 0) (param i32 i32)
    local.get 0
    i64.const 4294967297
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i32.load
    i32.store)
  (func $_ZN73_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$bool$GT$$GT$4from17h9b9a526a4e733139E (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 9
    i32.add
    local.get 1
    i32.store8
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    i32.store8)
  (func $_ZN92_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$svm_sdk..amount..Amount$GT$$GT$4from17h8d6665680edc889bE (type 9) (param i32 i64)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 3
    i32.store8)
  (func $_ZN71_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i8$GT$$GT$4from17h164afcb92ea98a32E (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 9
    i32.add
    local.get 1
    i32.store8
    local.get 0
    i32.const 8
    i32.add
    i32.const 4
    i32.store8)
  (func $_ZN71_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u8$GT$$GT$4from17h5ee2529d5a7a0eddE (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 9
    i32.add
    local.get 1
    i32.store8
    local.get 0
    i32.const 8
    i32.add
    i32.const 5
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i16$GT$$GT$4from17hd1e6afd39b18022dE (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 10
    i32.add
    local.get 1
    i32.store16
    local.get 0
    i32.const 8
    i32.add
    i32.const 6
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u16$GT$$GT$4from17h75b0139ac3f91186E (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 10
    i32.add
    local.get 1
    i32.store16
    local.get 0
    i32.const 8
    i32.add
    i32.const 7
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i32$GT$$GT$4from17hb24425081e86a431E (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 8
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u32$GT$$GT$4from17h20124cb092f508c4E (type 0) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 9
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$i64$GT$$GT$4from17h2e190c8cbb969be7E (type 9) (param i32 i64)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 10
    i32.store8)
  (func $_ZN72_$LT$svm_sdk..value..Value$u20$as$u20$core..convert..From$LT$u64$GT$$GT$4from17h2a26227943144d2fE (type 9) (param i32 i64)
    local.get 0
    i32.const 0
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i32.const 11
    i32.store8)
  (func $_ZN7svm_sdk5value82_$LT$impl$u20$core..convert..From$LT$svm_sdk..value..Value$GT$$u20$for$u20$u32$GT$4from17h78d7324904755594E (type 4) (param i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        local.get 0
        i32.const 8
        i32.add
        i32.load8_u
        i32.const 9
        i32.eq
        br_if 1 (;@1;)
      end
      i32.const 1049123
      i32.const 40
      i32.const 1049164
      call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
      unreachable
    end
    local.get 0
    i32.const 12
    i32.add
    i32.load)
  (func $_ZN7svm_sdk6memory5alloc17hc2483383b149bf9dE (type 4) (param i32) (result i32)
    local.get 0
    i32.const 1
    call $__rust_alloc_zeroed)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h08d06f832c14283aE (type 10) (param i32) (result i64)
    i64.const -2214446546759255314)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h911021edd741cc49E (type 10) (param i32) (result i64)
    i64.const -3994463020707609660)
  (func $_ZN4core3ptr13drop_in_place17h020c7e7e67a167e6E (type 5) (param i32))
  (func $_ZN4core3ptr13drop_in_place17h2af87d0bdfae1a93E (type 5) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core3ptr13drop_in_place17hfbc67c1b337a35b6E (type 5) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.const 1
      call $__rust_dealloc
    end)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h6a9f553991623917E (type 4) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1049220
      i32.const 43
      i32.const 1049304
      call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
      unreachable
    end
    local.get 0)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17hd86c7e0fbe96e5aaE (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1049220
      i32.const 43
      local.get 1
      call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
      unreachable
    end
    local.get 0)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hc6e921a65ea9542bE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 0
            i32.store offset=12
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 2
            i32.const 12
            i32.add
            local.set 3
            block  ;; label = @5
              local.get 1
              i32.const 65536
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=14
              local.get 2
              local.get 1
              i32.const 12
              i32.shr_u
              i32.const 224
              i32.or
              i32.store8 offset=12
              local.get 2
              local.get 1
              i32.const 6
              i32.shr_u
              i32.const 63
              i32.and
              i32.const 128
              i32.or
              i32.store8 offset=13
              i32.const 3
              local.set 1
              br 3 (;@2;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=15
            local.get 2
            local.get 1
            i32.const 18
            i32.shr_u
            i32.const 240
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 4
            local.set 1
            br 2 (;@2;)
          end
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 3
            local.get 0
            i32.const 4
            i32.add
            i32.load
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h295a4e0018e2d199E
            local.get 0
            i32.load offset=8
            local.set 3
          end
          local.get 0
          i32.load
          local.get 3
          i32.add
          local.get 1
          i32.store8
          local.get 0
          local.get 0
          i32.load offset=8
          i32.const 1
          i32.add
          i32.store offset=8
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        local.get 2
        i32.const 12
        i32.add
        local.set 3
        i32.const 2
        local.set 1
      end
      local.get 0
      local.get 1
      call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h295a4e0018e2d199E
      local.get 0
      i32.load
      local.get 0
      i32.const 8
      i32.add
      local.tee 0
      i32.load
      local.tee 4
      i32.add
      local.get 3
      local.get 1
      call $memcpy
      drop
      local.get 0
      local.get 4
      local.get 1
      i32.add
      i32.store
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0)
  (func $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h295a4e0018e2d199E (type 0) (param i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 2
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.add
          local.tee 1
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
          local.get 2
          i32.const 1
          i32.shl
          local.tee 3
          local.get 1
          local.get 3
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.tee 3
          i32.const 0
          i32.lt_s
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i32.load
              i32.const 0
              local.get 2
              select
              local.tee 1
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              call $__rust_alloc
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            local.get 3
            i32.eq
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              local.get 3
              i32.const 1
              call $__rust_alloc
              local.set 1
              br 1 (;@4;)
            end
            local.get 1
            local.get 2
            i32.const 1
            local.get 3
            call $__rust_realloc
            local.set 1
          end
          local.get 1
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          i32.store
          local.get 0
          i32.const 4
          i32.add
          local.get 3
          i32.store
        end
        return
      end
      call $_ZN5alloc7raw_vec17capacity_overflow17ha369776b076180eeE
      unreachable
    end
    local.get 3
    i32.const 1
    call $_ZN5alloc5alloc18handle_alloc_error17he62175e02bb3fceeE
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h633f9c548303c0d8E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1049180
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8b996d8af01475c9E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h196f664680526f53E (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 0
    local.get 2
    call $_ZN5alloc3vec12Vec$LT$T$GT$7reserve17h295a4e0018e2d199E
    local.get 0
    i32.load
    local.get 0
    i32.const 8
    i32.add
    local.tee 0
    i32.load
    local.tee 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store
    i32.const 0)
  (func $_ZN3std5alloc24default_alloc_error_hook17hd9d9385c91b96c15E (type 0) (param i32 i32))
  (func $rust_oom (type 0) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.const 0
    i32.load offset=1052728
    local.tee 2
    i32.const 7
    local.get 2
    select
    call_indirect (type 0)
    unreachable
    unreachable)
  (func $__rdl_alloc (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      i32.const 1052748
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h34fd3793387a0586E
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 1052748
      local.get 1
      local.get 0
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17he5847471231aa01aE
      return
    end
    i32.const 1052748
    local.get 0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE)
  (func $__rdl_dealloc (type 7) (param i32 i32 i32)
    i32.const 1052748
    local.get 0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h25e0c0cdb1422e7eE)
  (func $__rdl_realloc (type 8) (param i32 i32 i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1052748
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h34fd3793387a0586E
        local.get 2
        i32.ge_u
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            i32.const 1052748
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h34fd3793387a0586E
            local.get 2
            i32.ge_u
            br_if 0 (;@4;)
            i32.const 1052748
            local.get 2
            local.get 3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17he5847471231aa01aE
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1052748
          local.get 3
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE
          local.set 2
        end
        local.get 2
        br_if 1 (;@1;)
        i32.const 0
        return
      end
      i32.const 1052748
      local.get 0
      local.get 3
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h430fa370263f868fE
      return
    end
    local.get 2
    local.get 0
    local.get 3
    local.get 1
    local.get 1
    local.get 3
    i32.gt_u
    select
    call $memcpy
    local.set 2
    i32.const 1052748
    local.get 0
    call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h25e0c0cdb1422e7eE
    local.get 2)
  (func $__rdl_alloc_zeroed (type 2) (param i32 i32) (result i32)
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1052748
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h34fd3793387a0586E
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 1052748
        local.get 1
        local.get 0
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17he5847471231aa01aE
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1052748
      local.get 0
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE
      local.set 1
    end
    block  ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1052748
      local.get 1
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc17calloc_must_clear17ha816ec7e279978dbE
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      local.get 0
      call $memset
      drop
    end
    local.get 1)
  (func $rust_begin_unwind (type 5) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    call $_ZN4core5panic9PanicInfo8location17h24feadd41cc759ecE
    i32.const 1049288
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17hd86c7e0fbe96e5aaE
    local.set 2
    local.get 0
    call $_ZN4core5panic9PanicInfo7message17ha07d8699a1298dc8E
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h6a9f553991623917E
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 1
    i32.const 1049320
    local.get 0
    call $_ZN4core5panic9PanicInfo7message17ha07d8699a1298dc8E
    local.get 2
    call $_ZN3std9panicking20rust_panic_with_hook17he9cf70f096e3ae49E
    unreachable)
  (func $_ZN3std9panicking20rust_panic_with_hook17he9cf70f096e3ae49E (type 11) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    i32.const 1
    local.set 5
    i32.const 0
    i32.const 0
    i32.load offset=1052744
    i32.const 1
    i32.add
    i32.store offset=1052744
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1053200
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            i32.const 0
            i64.const 4294967297
            i64.store offset=1053200
            br 1 (;@3;)
          end
          i32.const 0
          i32.const 0
          i32.load offset=1053204
          i32.const 1
          i32.add
          local.tee 5
          i32.store offset=1053204
          local.get 5
          i32.const 2
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 4
        local.get 3
        i32.store offset=28
        local.get 4
        local.get 2
        i32.store offset=24
        local.get 4
        i32.const 1049204
        i32.store offset=20
        local.get 4
        i32.const 1049204
        i32.store offset=16
        i32.const 0
        i32.load offset=1052732
        local.tee 2
        i32.const -1
        i32.le_s
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.store offset=1052732
        block  ;; label = @3
          i32.const 0
          i32.load offset=1052740
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1052736
          local.set 2
          local.get 4
          i32.const 8
          i32.add
          local.get 0
          local.get 1
          i32.load offset=16
          call_indirect (type 0)
          local.get 4
          local.get 4
          i64.load offset=8
          i64.store offset=16
          local.get 2
          local.get 4
          i32.const 16
          i32.add
          local.get 3
          i32.load offset=12
          call_indirect (type 0)
          i32.const 0
          i32.load offset=1052732
          local.set 2
        end
        i32.const 0
        local.get 2
        i32.const -1
        i32.add
        i32.store offset=1052732
        local.get 5
        i32.const 1
        i32.le_u
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    call $rust_panic
    unreachable)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17hfc3314771b196f49E (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 3
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      local.set 3
      local.get 1
      i32.load
      local.set 4
      local.get 2
      i32.const 0
      i32.store offset=32
      local.get 2
      i64.const 1
      i64.store offset=24
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 4
      i64.load align=4
      i64.store offset=40
      local.get 2
      i32.const 36
      i32.add
      i32.const 1049180
      local.get 2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h8b996d8af01475c9E
      drop
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      local.get 2
      i32.load offset=32
      i32.store
      local.get 2
      local.get 2
      i64.load offset=24
      i64.store offset=8
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        i32.load
        local.tee 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 6
        i32.const 1
        call $__rust_dealloc
      end
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
      local.get 3
      i32.load
      local.set 3
    end
    local.get 1
    i32.const 1
    i32.store offset=4
    local.get 1
    i32.const 12
    i32.add
    i32.load
    local.set 4
    local.get 1
    i32.const 8
    i32.add
    local.tee 1
    i32.load
    local.set 5
    local.get 1
    i64.const 0
    i64.store align=4
    block  ;; label = @1
      i32.const 12
      i32.const 4
      call $__rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 12
      i32.const 4
      call $_ZN5alloc5alloc18handle_alloc_error17he62175e02bb3fceeE
      unreachable
    end
    local.get 1
    local.get 4
    i32.store offset=8
    local.get 1
    local.get 5
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1049340
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 64
    i32.add
    global.set 0)
  (func $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17ha36eed8111393a47E (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.load offset=4
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.set 4
      local.get 2
      i32.const 0
      i32.store offset=32
      local.get 2
      i64.const 1
      i64.store offset=24
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=36
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 4
      i32.const 16
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 4
      i32.const 8
      i32.add
      i64.load align=4
      i64.store
      local.get 2
      local.get 4
      i64.load align=4
      i64.store offset=40
      local.get 2
      i32.const 36
      i32.add
      i32.const 1049180
      local.get 2
      i32.const 40
      i32.add
      call $_ZN4core3fmt5write17h8b996d8af01475c9E
      drop
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      local.tee 4
      local.get 2
      i32.load offset=32
      i32.store
      local.get 2
      local.get 2
      i64.load offset=24
      i64.store offset=8
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        i32.const 1
        call $__rust_dealloc
      end
      local.get 3
      local.get 2
      i64.load offset=8
      i64.store align=4
      local.get 3
      i32.const 8
      i32.add
      local.get 4
      i32.load
      i32.store
    end
    local.get 0
    i32.const 1049340
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 64
    i32.add
    global.set 0)
  (func $rust_panic (type 0) (param i32 i32)
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
    i32.const 8
    i32.add
    call $__rust_start_panic
    drop
    unreachable
    unreachable)
  (func $__rust_start_panic (type 4) (param i32) (result i32)
    unreachable
    unreachable)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc16malloc_alignment17h34fd3793387a0586E (type 4) (param i32) (result i32)
    i32.const 8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc17calloc_must_clear17ha816ec7e279978dbE (type 2) (param i32 i32) (result i32)
    local.get 1
    i32.const -4
    i32.add
    i32.load8_u
    i32.const 3
    i32.and
    i32.const 0
    i32.ne)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 245
              i32.lt_u
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              local.get 1
              i32.const -65587
              i32.ge_u
              br_if 4 (;@1;)
              local.get 1
              i32.const 11
              i32.add
              local.tee 1
              i32.const -8
              i32.and
              local.set 3
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 4
              i32.eqz
              br_if 1 (;@4;)
              i32.const 0
              local.set 5
              block  ;; label = @6
                local.get 1
                i32.const 8
                i32.shr_u
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                i32.const 31
                local.set 5
                local.get 3
                i32.const 16777215
                i32.gt_u
                br_if 0 (;@6;)
                local.get 3
                i32.const 6
                local.get 1
                i32.clz
                local.tee 1
                i32.sub
                i32.const 31
                i32.and
                i32.shr_u
                i32.const 1
                i32.and
                local.get 1
                i32.const 1
                i32.shl
                i32.sub
                i32.const 62
                i32.add
                local.set 5
              end
              i32.const 0
              local.get 3
              i32.sub
              local.set 2
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    local.get 5
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    i32.load
                    local.tee 1
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 6
                    local.get 3
                    i32.const 0
                    i32.const 25
                    local.get 5
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    i32.const 31
                    i32.and
                    local.get 5
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 7
                    i32.const 0
                    local.set 8
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.const 4
                        i32.add
                        i32.load
                        i32.const -8
                        i32.and
                        local.tee 9
                        local.get 3
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.get 3
                        i32.sub
                        local.tee 9
                        local.get 2
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.set 2
                        local.get 1
                        local.set 8
                        local.get 9
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 2
                        local.get 1
                        local.set 8
                        br 3 (;@7;)
                      end
                      local.get 1
                      i32.const 20
                      i32.add
                      i32.load
                      local.tee 9
                      local.get 6
                      local.get 9
                      local.get 1
                      local.get 7
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      i32.load
                      local.tee 1
                      i32.ne
                      select
                      local.get 6
                      local.get 9
                      select
                      local.set 6
                      local.get 7
                      i32.const 1
                      i32.shl
                      local.set 7
                      local.get 1
                      br_if 0 (;@9;)
                    end
                    block  ;; label = @9
                      local.get 6
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 6
                      local.set 1
                      br 2 (;@7;)
                    end
                    local.get 8
                    br_if 2 (;@6;)
                  end
                  i32.const 0
                  local.set 8
                  i32.const 2
                  local.get 5
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee 1
                  i32.const 0
                  local.get 1
                  i32.sub
                  i32.or
                  local.get 4
                  i32.and
                  local.tee 1
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  local.get 1
                  i32.const 0
                  local.get 1
                  i32.sub
                  i32.and
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 272
                  i32.add
                  i32.load
                  local.tee 1
                  i32.eqz
                  br_if 3 (;@4;)
                end
                loop  ;; label = @7
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.tee 6
                  local.get 3
                  i32.ge_u
                  local.get 6
                  local.get 3
                  i32.sub
                  local.tee 9
                  local.get 2
                  i32.lt_u
                  i32.and
                  local.set 7
                  block  ;; label = @8
                    local.get 1
                    i32.load offset=16
                    local.tee 6
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 20
                    i32.add
                    i32.load
                    local.set 6
                  end
                  local.get 1
                  local.get 8
                  local.get 7
                  select
                  local.set 8
                  local.get 9
                  local.get 2
                  local.get 7
                  select
                  local.set 2
                  local.get 6
                  local.set 1
                  local.get 6
                  br_if 0 (;@7;)
                end
                local.get 8
                i32.eqz
                br_if 2 (;@4;)
              end
              block  ;; label = @6
                local.get 0
                i32.load offset=400
                local.tee 1
                local.get 3
                i32.lt_u
                br_if 0 (;@6;)
                local.get 2
                local.get 1
                local.get 3
                i32.sub
                i32.ge_u
                br_if 2 (;@4;)
              end
              local.get 8
              i32.load offset=24
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 8
                    i32.load offset=12
                    local.tee 6
                    local.get 8
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 20
                    i32.const 16
                    local.get 8
                    i32.const 20
                    i32.add
                    local.tee 6
                    i32.load
                    local.tee 7
                    select
                    i32.add
                    i32.load
                    local.tee 1
                    br_if 1 (;@7;)
                    i32.const 0
                    local.set 6
                    br 2 (;@6;)
                  end
                  local.get 8
                  i32.load offset=8
                  local.tee 1
                  local.get 6
                  i32.store offset=12
                  local.get 6
                  local.get 1
                  i32.store offset=8
                  br 1 (;@6;)
                end
                local.get 6
                local.get 8
                i32.const 16
                i32.add
                local.get 7
                select
                local.set 7
                loop  ;; label = @7
                  local.get 7
                  local.set 9
                  block  ;; label = @8
                    local.get 1
                    local.tee 6
                    i32.const 20
                    i32.add
                    local.tee 7
                    i32.load
                    local.tee 1
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 16
                    i32.add
                    local.set 7
                    local.get 6
                    i32.load offset=16
                    local.set 1
                  end
                  local.get 1
                  br_if 0 (;@7;)
                end
                local.get 9
                i32.const 0
                i32.store
              end
              block  ;; label = @6
                local.get 5
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    local.get 8
                    i32.load offset=28
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    local.tee 1
                    i32.load
                    local.get 8
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 16
                    i32.const 20
                    local.get 5
                    i32.load offset=16
                    local.get 8
                    i32.eq
                    select
                    i32.add
                    local.get 6
                    i32.store
                    local.get 6
                    i32.eqz
                    br_if 2 (;@6;)
                    br 1 (;@7;)
                  end
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 6
                  br_if 0 (;@7;)
                  local.get 0
                  i32.const 4
                  i32.add
                  local.tee 1
                  local.get 1
                  i32.load
                  i32.const -2
                  local.get 8
                  i32.load offset=28
                  i32.rotl
                  i32.and
                  i32.store
                  br 1 (;@6;)
                end
                local.get 6
                local.get 5
                i32.store offset=24
                block  ;; label = @7
                  local.get 8
                  i32.load offset=16
                  local.tee 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 6
                  local.get 1
                  i32.store offset=16
                  local.get 1
                  local.get 6
                  i32.store offset=24
                end
                local.get 8
                i32.const 20
                i32.add
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.const 20
                i32.add
                local.get 1
                i32.store
                local.get 1
                local.get 6
                i32.store offset=24
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.const 16
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 8
                  local.get 3
                  i32.const 3
                  i32.or
                  i32.store offset=4
                  local.get 8
                  local.get 3
                  i32.add
                  local.tee 3
                  local.get 2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 3
                  local.get 2
                  i32.add
                  local.get 2
                  i32.store
                  block  ;; label = @8
                    local.get 2
                    i32.const 256
                    i32.lt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.const 8
                        i32.shr_u
                        local.tee 6
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 1
                        br 1 (;@9;)
                      end
                      i32.const 31
                      local.set 1
                      local.get 2
                      i32.const 16777215
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 6
                      local.get 6
                      i32.clz
                      local.tee 1
                      i32.sub
                      i32.const 31
                      i32.and
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 1
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 62
                      i32.add
                      local.set 1
                    end
                    local.get 3
                    i64.const 0
                    i64.store offset=16 align=4
                    local.get 3
                    local.get 1
                    i32.store offset=28
                    local.get 0
                    local.get 1
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 272
                    i32.add
                    local.set 6
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 0
                              i32.const 4
                              i32.add
                              local.tee 7
                              i32.load
                              local.tee 9
                              i32.const 1
                              local.get 1
                              i32.const 31
                              i32.and
                              i32.shl
                              local.tee 0
                              i32.and
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 6
                              i32.load
                              local.tee 7
                              i32.const 4
                              i32.add
                              i32.load
                              i32.const -8
                              i32.and
                              local.get 2
                              i32.ne
                              br_if 1 (;@12;)
                              local.get 7
                              local.set 1
                              br 2 (;@11;)
                            end
                            local.get 7
                            local.get 9
                            local.get 0
                            i32.or
                            i32.store
                            local.get 6
                            local.get 3
                            i32.store
                            local.get 3
                            local.get 6
                            i32.store offset=24
                            br 3 (;@9;)
                          end
                          local.get 2
                          i32.const 0
                          i32.const 25
                          local.get 1
                          i32.const 1
                          i32.shr_u
                          i32.sub
                          i32.const 31
                          i32.and
                          local.get 1
                          i32.const 31
                          i32.eq
                          select
                          i32.shl
                          local.set 6
                          loop  ;; label = @12
                            local.get 7
                            local.get 6
                            i32.const 29
                            i32.shr_u
                            i32.const 4
                            i32.and
                            i32.add
                            i32.const 16
                            i32.add
                            local.tee 9
                            i32.load
                            local.tee 1
                            i32.eqz
                            br_if 2 (;@10;)
                            local.get 6
                            i32.const 1
                            i32.shl
                            local.set 6
                            local.get 1
                            local.set 7
                            local.get 1
                            i32.const 4
                            i32.add
                            i32.load
                            i32.const -8
                            i32.and
                            local.get 2
                            i32.ne
                            br_if 0 (;@12;)
                          end
                        end
                        local.get 1
                        i32.load offset=8
                        local.tee 2
                        local.get 3
                        i32.store offset=12
                        local.get 1
                        local.get 3
                        i32.store offset=8
                        local.get 3
                        i32.const 0
                        i32.store offset=24
                        local.get 3
                        local.get 1
                        i32.store offset=12
                        local.get 3
                        local.get 2
                        i32.store offset=8
                        br 4 (;@6;)
                      end
                      local.get 9
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 7
                      i32.store offset=24
                    end
                    local.get 3
                    local.get 3
                    i32.store offset=12
                    local.get 3
                    local.get 3
                    i32.store offset=8
                    br 2 (;@6;)
                  end
                  local.get 0
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  local.tee 2
                  i32.const 3
                  i32.shl
                  i32.add
                  i32.const 8
                  i32.add
                  local.set 1
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load
                      local.tee 6
                      i32.const 1
                      local.get 2
                      i32.shl
                      local.tee 2
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load offset=8
                      local.set 2
                      br 1 (;@8;)
                    end
                    local.get 0
                    local.get 6
                    local.get 2
                    i32.or
                    i32.store
                    local.get 1
                    local.set 2
                  end
                  local.get 1
                  local.get 3
                  i32.store offset=8
                  local.get 2
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 1
                  i32.store offset=12
                  local.get 3
                  local.get 2
                  i32.store offset=8
                  br 1 (;@6;)
                end
                local.get 8
                local.get 2
                local.get 3
                i32.add
                local.tee 1
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 8
                local.get 1
                i32.add
                local.tee 1
                local.get 1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
              end
              local.get 8
              i32.const 8
              i32.add
              return
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load
                  local.tee 8
                  i32.const 16
                  local.get 1
                  i32.const 11
                  i32.add
                  i32.const -8
                  i32.and
                  local.get 1
                  i32.const 11
                  i32.lt_u
                  select
                  local.tee 3
                  i32.const 3
                  i32.shr_u
                  local.tee 2
                  i32.shr_u
                  local.tee 1
                  i32.const 3
                  i32.and
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 0
                  i32.load offset=400
                  i32.le_u
                  br_if 3 (;@4;)
                  local.get 1
                  br_if 1 (;@6;)
                  local.get 0
                  i32.load offset=4
                  local.tee 1
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  local.get 1
                  i32.const 0
                  local.get 1
                  i32.sub
                  i32.and
                  i32.ctz
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 272
                  i32.add
                  i32.load
                  local.tee 6
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.get 3
                  i32.sub
                  local.set 2
                  local.get 6
                  local.set 7
                  loop  ;; label = @8
                    block  ;; label = @9
                      local.get 6
                      i32.load offset=16
                      local.tee 1
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const 20
                      i32.add
                      i32.load
                      local.tee 1
                      i32.eqz
                      br_if 4 (;@5;)
                    end
                    local.get 1
                    i32.const 4
                    i32.add
                    i32.load
                    i32.const -8
                    i32.and
                    local.get 3
                    i32.sub
                    local.tee 6
                    local.get 2
                    local.get 6
                    local.get 2
                    i32.lt_u
                    local.tee 6
                    select
                    local.set 2
                    local.get 1
                    local.get 7
                    local.get 6
                    select
                    local.set 7
                    local.get 1
                    local.set 6
                    br 0 (;@8;)
                  end
                end
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    local.get 1
                    i32.const -1
                    i32.xor
                    i32.const 1
                    i32.and
                    local.get 2
                    i32.add
                    local.tee 3
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.const 16
                    i32.add
                    i32.load
                    local.tee 1
                    i32.const 8
                    i32.add
                    local.tee 2
                    i32.load
                    local.tee 6
                    local.get 7
                    i32.const 8
                    i32.add
                    local.tee 7
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 7
                    i32.store offset=12
                    local.get 7
                    local.get 6
                    i32.store offset=8
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 8
                  i32.const -2
                  local.get 3
                  i32.rotl
                  i32.and
                  i32.store
                end
                local.get 1
                local.get 3
                i32.const 3
                i32.shl
                local.tee 3
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 1
                local.get 3
                i32.add
                local.tee 1
                local.get 1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                br 5 (;@1;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  local.get 1
                  local.get 2
                  i32.shl
                  i32.const 2
                  local.get 2
                  i32.shl
                  local.tee 1
                  i32.const 0
                  local.get 1
                  i32.sub
                  i32.or
                  i32.and
                  local.tee 1
                  i32.const 0
                  local.get 1
                  i32.sub
                  i32.and
                  i32.ctz
                  local.tee 2
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 7
                  i32.const 16
                  i32.add
                  i32.load
                  local.tee 1
                  i32.const 8
                  i32.add
                  local.tee 9
                  i32.load
                  local.tee 6
                  local.get 7
                  i32.const 8
                  i32.add
                  local.tee 7
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 6
                  local.get 7
                  i32.store offset=12
                  local.get 7
                  local.get 6
                  i32.store offset=8
                  br 1 (;@6;)
                end
                local.get 0
                local.get 8
                i32.const -2
                local.get 2
                i32.rotl
                i32.and
                i32.store
              end
              local.get 1
              local.get 3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 1
              local.get 3
              i32.add
              local.tee 6
              local.get 2
              i32.const 3
              i32.shl
              local.tee 2
              local.get 3
              i32.sub
              local.tee 3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 2
              i32.add
              local.get 3
              i32.store
              block  ;; label = @6
                local.get 0
                i32.load offset=400
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                local.get 1
                i32.const 3
                i32.shr_u
                local.tee 7
                i32.const 3
                i32.shl
                i32.add
                i32.const 8
                i32.add
                local.set 2
                local.get 0
                i32.load offset=408
                local.set 1
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load
                    local.tee 8
                    i32.const 1
                    local.get 7
                    i32.const 31
                    i32.and
                    i32.shl
                    local.tee 7
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    i32.load offset=8
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 8
                  local.get 7
                  i32.or
                  i32.store
                  local.get 2
                  local.set 7
                end
                local.get 2
                local.get 1
                i32.store offset=8
                local.get 7
                local.get 1
                i32.store offset=12
                local.get 1
                local.get 2
                i32.store offset=12
                local.get 1
                local.get 7
                i32.store offset=8
              end
              local.get 0
              local.get 6
              i32.store offset=408
              local.get 0
              local.get 3
              i32.store offset=400
              local.get 9
              return
            end
            local.get 7
            i32.load offset=24
            local.set 5
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 7
                  i32.load offset=12
                  local.tee 6
                  local.get 7
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 20
                  i32.const 16
                  local.get 7
                  i32.const 20
                  i32.add
                  local.tee 6
                  i32.load
                  local.tee 8
                  select
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 6
                  br 2 (;@5;)
                end
                local.get 7
                i32.load offset=8
                local.tee 1
                local.get 6
                i32.store offset=12
                local.get 6
                local.get 1
                i32.store offset=8
                br 1 (;@5;)
              end
              local.get 6
              local.get 7
              i32.const 16
              i32.add
              local.get 8
              select
              local.set 8
              loop  ;; label = @6
                local.get 8
                local.set 9
                block  ;; label = @7
                  local.get 1
                  local.tee 6
                  i32.const 20
                  i32.add
                  local.tee 8
                  i32.load
                  local.tee 1
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 16
                  i32.add
                  local.set 8
                  local.get 6
                  i32.load offset=16
                  local.set 1
                end
                local.get 1
                br_if 0 (;@6;)
              end
              local.get 9
              i32.const 0
              i32.store
            end
            local.get 5
            i32.eqz
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 0
              local.get 7
              i32.load offset=28
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              local.tee 1
              i32.load
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 5
              i32.const 16
              i32.const 20
              local.get 5
              i32.load offset=16
              local.get 7
              i32.eq
              select
              i32.add
              local.get 6
              i32.store
              local.get 6
              i32.eqz
              br_if 3 (;@2;)
              br 2 (;@3;)
            end
            local.get 1
            local.get 6
            i32.store
            local.get 6
            br_if 1 (;@3;)
            local.get 0
            local.get 0
            i32.load offset=4
            i32.const -2
            local.get 7
            i32.load offset=28
            i32.rotl
            i32.and
            i32.store offset=4
            br 2 (;@2;)
          end
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=400
                      local.tee 1
                      local.get 3
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.load offset=404
                      local.tee 1
                      local.get 3
                      i32.gt_u
                      br_if 3 (;@6;)
                      i32.const 0
                      local.set 2
                      local.get 3
                      i32.const 65583
                      i32.add
                      local.tee 6
                      i32.const 16
                      i32.shr_u
                      memory.grow
                      local.tee 1
                      i32.const -1
                      i32.eq
                      br_if 8 (;@1;)
                      local.get 1
                      i32.const 16
                      i32.shl
                      local.tee 8
                      i32.eqz
                      br_if 8 (;@1;)
                      local.get 0
                      local.get 0
                      i32.load offset=416
                      local.get 6
                      i32.const -65536
                      i32.and
                      local.tee 5
                      i32.add
                      local.tee 1
                      i32.store offset=416
                      local.get 0
                      local.get 0
                      i32.load offset=420
                      local.tee 6
                      local.get 1
                      local.get 6
                      local.get 1
                      i32.gt_u
                      select
                      i32.store offset=420
                      local.get 0
                      i32.load offset=412
                      local.tee 6
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 0
                      i32.const 424
                      i32.add
                      local.tee 4
                      local.set 1
                      loop  ;; label = @10
                        local.get 1
                        i32.load
                        local.tee 7
                        local.get 1
                        i32.load offset=4
                        local.tee 9
                        i32.add
                        local.get 8
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 1
                        i32.load offset=8
                        local.tee 1
                        br_if 0 (;@10;)
                        br 5 (;@5;)
                      end
                    end
                    local.get 0
                    i32.load offset=408
                    local.set 2
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        local.get 3
                        i32.sub
                        local.tee 6
                        i32.const 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 0
                        i32.store offset=408
                        local.get 0
                        i32.const 0
                        i32.store offset=400
                        local.get 2
                        local.get 1
                        i32.const 3
                        i32.or
                        i32.store offset=4
                        local.get 2
                        local.get 1
                        i32.add
                        local.tee 3
                        i32.const 4
                        i32.add
                        local.set 1
                        local.get 3
                        i32.load offset=4
                        i32.const 1
                        i32.or
                        local.set 3
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 6
                      i32.store offset=400
                      local.get 0
                      local.get 2
                      local.get 3
                      i32.add
                      local.tee 7
                      i32.store offset=408
                      local.get 7
                      local.get 6
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      local.get 2
                      local.get 1
                      i32.add
                      local.get 6
                      i32.store
                      local.get 3
                      i32.const 3
                      i32.or
                      local.set 3
                      local.get 2
                      i32.const 4
                      i32.add
                      local.set 1
                    end
                    local.get 1
                    local.get 3
                    i32.store
                    local.get 2
                    i32.const 8
                    i32.add
                    return
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.load offset=444
                      local.tee 1
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 8
                      i32.le_u
                      br_if 1 (;@8;)
                    end
                    local.get 0
                    local.get 8
                    i32.store offset=444
                  end
                  local.get 0
                  i32.const 4095
                  i32.store offset=448
                  local.get 0
                  local.get 8
                  i32.store offset=424
                  local.get 0
                  i32.const 436
                  i32.add
                  i32.const 0
                  i32.store
                  local.get 0
                  i32.const 428
                  i32.add
                  local.get 5
                  i32.store
                  local.get 0
                  i32.const 20
                  i32.add
                  local.get 0
                  i32.const 8
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 0
                  i32.const 28
                  i32.add
                  local.get 0
                  i32.const 16
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 36
                  i32.add
                  local.get 0
                  i32.const 24
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 44
                  i32.add
                  local.get 0
                  i32.const 32
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 52
                  i32.add
                  local.get 0
                  i32.const 40
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 60
                  i32.add
                  local.get 0
                  i32.const 48
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 68
                  i32.add
                  local.get 0
                  i32.const 56
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 76
                  i32.add
                  local.get 0
                  i32.const 64
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 84
                  i32.add
                  local.get 0
                  i32.const 72
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 80
                  i32.add
                  local.tee 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 92
                  i32.add
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 88
                  i32.add
                  local.tee 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 100
                  i32.add
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 96
                  i32.add
                  local.tee 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 108
                  i32.add
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 104
                  i32.add
                  local.tee 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 116
                  i32.add
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 112
                  i32.add
                  local.tee 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 124
                  i32.add
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 120
                  i32.add
                  local.tee 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 132
                  i32.add
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 128
                  i32.add
                  local.tee 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 140
                  i32.add
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 136
                  i32.add
                  local.tee 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 148
                  i32.add
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 156
                  i32.add
                  local.get 0
                  i32.const 144
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 164
                  i32.add
                  local.get 0
                  i32.const 152
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 172
                  i32.add
                  local.get 0
                  i32.const 160
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 180
                  i32.add
                  local.get 0
                  i32.const 168
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 188
                  i32.add
                  local.get 0
                  i32.const 176
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 196
                  i32.add
                  local.get 0
                  i32.const 184
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 204
                  i32.add
                  local.get 0
                  i32.const 192
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 212
                  i32.add
                  local.get 0
                  i32.const 200
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 220
                  i32.add
                  local.get 0
                  i32.const 208
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 228
                  i32.add
                  local.get 0
                  i32.const 216
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 236
                  i32.add
                  local.get 0
                  i32.const 224
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 244
                  i32.add
                  local.get 0
                  i32.const 232
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 252
                  i32.add
                  local.get 0
                  i32.const 240
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  i32.const 260
                  i32.add
                  local.get 0
                  i32.const 248
                  i32.add
                  local.tee 6
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store
                  local.get 0
                  i32.const 268
                  i32.add
                  local.get 0
                  i32.const 256
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 1
                  local.get 6
                  i32.store
                  local.get 0
                  local.get 8
                  i32.store offset=412
                  local.get 0
                  i32.const 264
                  i32.add
                  local.get 1
                  i32.store
                  local.get 0
                  local.get 5
                  i32.const -40
                  i32.add
                  local.tee 1
                  i32.store offset=404
                  local.get 8
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 8
                  local.get 1
                  i32.add
                  i32.const 40
                  i32.store offset=4
                  local.get 0
                  i32.const 2097152
                  i32.store offset=440
                  br 3 (;@4;)
                end
                local.get 1
                i32.const 12
                i32.add
                i32.load
                br_if 1 (;@5;)
                local.get 8
                local.get 6
                i32.le_u
                br_if 1 (;@5;)
                local.get 7
                local.get 6
                i32.gt_u
                br_if 1 (;@5;)
                local.get 1
                local.get 9
                local.get 5
                i32.add
                i32.store offset=4
                local.get 0
                local.get 0
                i32.load offset=412
                local.tee 1
                i32.const 15
                i32.add
                i32.const -8
                i32.and
                local.tee 6
                i32.const -8
                i32.add
                i32.store offset=412
                local.get 0
                local.get 1
                local.get 6
                i32.sub
                local.get 0
                i32.load offset=404
                local.get 5
                i32.add
                local.tee 7
                i32.add
                i32.const 8
                i32.add
                local.tee 8
                i32.store offset=404
                local.get 6
                i32.const -4
                i32.add
                local.get 8
                i32.const 1
                i32.or
                i32.store
                local.get 1
                local.get 7
                i32.add
                i32.const 40
                i32.store offset=4
                local.get 0
                i32.const 2097152
                i32.store offset=440
                br 2 (;@4;)
              end
              local.get 0
              local.get 1
              local.get 3
              i32.sub
              local.tee 2
              i32.store offset=404
              local.get 0
              local.get 0
              i32.load offset=412
              local.tee 1
              local.get 3
              i32.add
              local.tee 6
              i32.store offset=412
              local.get 6
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 1
              i32.const 8
              i32.add
              return
            end
            local.get 0
            local.get 0
            i32.load offset=444
            local.tee 1
            local.get 8
            local.get 1
            local.get 8
            i32.lt_u
            select
            i32.store offset=444
            local.get 8
            local.get 5
            i32.add
            local.set 7
            local.get 4
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 1
                  i32.load
                  local.get 7
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 1
                  i32.load offset=8
                  local.tee 1
                  br_if 0 (;@7;)
                  br 2 (;@5;)
                end
              end
              local.get 1
              i32.const 12
              i32.add
              i32.load
              br_if 0 (;@5;)
              local.get 1
              local.get 8
              i32.store
              local.get 1
              local.get 1
              i32.load offset=4
              local.get 5
              i32.add
              i32.store offset=4
              local.get 8
              local.get 3
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 8
              local.get 3
              i32.add
              local.set 1
              local.get 7
              local.get 8
              i32.sub
              local.get 3
              i32.sub
              local.set 3
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=412
                    local.get 7
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 0
                    i32.load offset=408
                    local.get 7
                    i32.eq
                    br_if 1 (;@7;)
                    block  ;; label = @9
                      local.get 7
                      i32.const 4
                      i32.add
                      i32.load
                      local.tee 2
                      i32.const 3
                      i32.and
                      i32.const 1
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 7
                      local.get 2
                      i32.const -8
                      i32.and
                      local.tee 2
                      call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
                      local.get 2
                      local.get 3
                      i32.add
                      local.set 3
                      local.get 7
                      local.get 2
                      i32.add
                      local.set 7
                    end
                    local.get 7
                    local.get 7
                    i32.load offset=4
                    i32.const -2
                    i32.and
                    i32.store offset=4
                    local.get 1
                    local.get 3
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    local.get 1
                    local.get 3
                    i32.add
                    local.get 3
                    i32.store
                    block  ;; label = @9
                      local.get 3
                      i32.const 256
                      i32.lt_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 3
                          i32.const 8
                          i32.shr_u
                          local.tee 6
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 2
                          br 1 (;@10;)
                        end
                        i32.const 31
                        local.set 2
                        local.get 3
                        i32.const 16777215
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 6
                        local.get 6
                        i32.clz
                        local.tee 2
                        i32.sub
                        i32.const 31
                        i32.and
                        i32.shr_u
                        i32.const 1
                        i32.and
                        local.get 2
                        i32.const 1
                        i32.shl
                        i32.sub
                        i32.const 62
                        i32.add
                        local.set 2
                      end
                      local.get 1
                      i64.const 0
                      i64.store offset=16 align=4
                      local.get 1
                      local.get 2
                      i32.store offset=28
                      local.get 0
                      local.get 2
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.const 272
                      i32.add
                      local.set 6
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.const 4
                                i32.add
                                local.tee 7
                                i32.load
                                local.tee 9
                                i32.const 1
                                local.get 2
                                i32.const 31
                                i32.and
                                i32.shl
                                local.tee 0
                                i32.and
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 6
                                i32.load
                                local.tee 7
                                i32.const 4
                                i32.add
                                i32.load
                                i32.const -8
                                i32.and
                                local.get 3
                                i32.ne
                                br_if 1 (;@13;)
                                local.get 7
                                local.set 2
                                br 2 (;@12;)
                              end
                              local.get 7
                              local.get 9
                              local.get 0
                              i32.or
                              i32.store
                              local.get 6
                              local.get 1
                              i32.store
                              local.get 1
                              local.get 6
                              i32.store offset=24
                              br 3 (;@10;)
                            end
                            local.get 3
                            i32.const 0
                            i32.const 25
                            local.get 2
                            i32.const 1
                            i32.shr_u
                            i32.sub
                            i32.const 31
                            i32.and
                            local.get 2
                            i32.const 31
                            i32.eq
                            select
                            i32.shl
                            local.set 6
                            loop  ;; label = @13
                              local.get 7
                              local.get 6
                              i32.const 29
                              i32.shr_u
                              i32.const 4
                              i32.and
                              i32.add
                              i32.const 16
                              i32.add
                              local.tee 9
                              i32.load
                              local.tee 2
                              i32.eqz
                              br_if 2 (;@11;)
                              local.get 6
                              i32.const 1
                              i32.shl
                              local.set 6
                              local.get 2
                              local.set 7
                              local.get 2
                              i32.const 4
                              i32.add
                              i32.load
                              i32.const -8
                              i32.and
                              local.get 3
                              i32.ne
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 2
                          i32.load offset=8
                          local.tee 3
                          local.get 1
                          i32.store offset=12
                          local.get 2
                          local.get 1
                          i32.store offset=8
                          local.get 1
                          i32.const 0
                          i32.store offset=24
                          local.get 1
                          local.get 2
                          i32.store offset=12
                          local.get 1
                          local.get 3
                          i32.store offset=8
                          br 5 (;@6;)
                        end
                        local.get 9
                        local.get 1
                        i32.store
                        local.get 1
                        local.get 7
                        i32.store offset=24
                      end
                      local.get 1
                      local.get 1
                      i32.store offset=12
                      local.get 1
                      local.get 1
                      i32.store offset=8
                      br 3 (;@6;)
                    end
                    local.get 0
                    local.get 3
                    i32.const 3
                    i32.shr_u
                    local.tee 2
                    i32.const 3
                    i32.shl
                    i32.add
                    i32.const 8
                    i32.add
                    local.set 3
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load
                        local.tee 6
                        i32.const 1
                        local.get 2
                        i32.shl
                        local.tee 2
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 3
                        i32.load offset=8
                        local.set 2
                        br 1 (;@9;)
                      end
                      local.get 0
                      local.get 6
                      local.get 2
                      i32.or
                      i32.store
                      local.get 3
                      local.set 2
                    end
                    local.get 3
                    local.get 1
                    i32.store offset=8
                    local.get 2
                    local.get 1
                    i32.store offset=12
                    local.get 1
                    local.get 3
                    i32.store offset=12
                    local.get 1
                    local.get 2
                    i32.store offset=8
                    br 2 (;@6;)
                  end
                  local.get 0
                  local.get 1
                  i32.store offset=412
                  local.get 0
                  local.get 0
                  i32.load offset=404
                  local.get 3
                  i32.add
                  local.tee 3
                  i32.store offset=404
                  local.get 1
                  local.get 3
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  br 1 (;@6;)
                end
                local.get 0
                local.get 1
                i32.store offset=408
                local.get 0
                local.get 0
                i32.load offset=400
                local.get 3
                i32.add
                local.tee 3
                i32.store offset=400
                local.get 1
                local.get 3
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 3
                i32.add
                local.get 3
                i32.store
              end
              local.get 8
              i32.const 8
              i32.add
              return
            end
            local.get 4
            local.set 1
            block  ;; label = @5
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.load
                  local.tee 7
                  local.get 6
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 7
                  local.get 1
                  i32.load offset=4
                  i32.add
                  local.tee 7
                  local.get 6
                  i32.gt_u
                  br_if 2 (;@5;)
                end
                local.get 1
                i32.load offset=8
                local.set 1
                br 0 (;@6;)
              end
            end
            local.get 0
            local.get 8
            i32.store offset=412
            local.get 0
            local.get 5
            i32.const -40
            i32.add
            local.tee 1
            i32.store offset=404
            local.get 8
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 8
            local.get 1
            i32.add
            i32.const 40
            i32.store offset=4
            local.get 0
            i32.const 2097152
            i32.store offset=440
            local.get 6
            local.get 7
            i32.const -32
            i32.add
            i32.const -8
            i32.and
            i32.const -8
            i32.add
            local.tee 1
            local.get 1
            local.get 6
            i32.const 16
            i32.add
            i32.lt_u
            select
            local.tee 9
            i32.const 27
            i32.store offset=4
            local.get 4
            i64.load align=4
            local.set 10
            local.get 9
            i32.const 16
            i32.add
            local.get 4
            i32.const 8
            i32.add
            i64.load align=4
            i64.store align=4
            local.get 9
            local.get 10
            i64.store offset=8 align=4
            local.get 0
            i32.const 436
            i32.add
            i32.const 0
            i32.store
            local.get 0
            i32.const 428
            i32.add
            local.get 5
            i32.store
            local.get 0
            local.get 8
            i32.store offset=424
            local.get 0
            i32.const 432
            i32.add
            local.get 9
            i32.const 8
            i32.add
            i32.store
            local.get 9
            i32.const 28
            i32.add
            local.set 1
            loop  ;; label = @5
              local.get 1
              i32.const 7
              i32.store
              local.get 7
              local.get 1
              i32.const 4
              i32.add
              local.tee 1
              i32.gt_u
              br_if 0 (;@5;)
            end
            local.get 9
            local.get 6
            i32.eq
            br_if 0 (;@4;)
            local.get 9
            local.get 9
            i32.load offset=4
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 6
            local.get 9
            local.get 6
            i32.sub
            local.tee 8
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 9
            local.get 8
            i32.store
            block  ;; label = @5
              local.get 8
              i32.const 256
              i32.lt_u
              br_if 0 (;@5;)
              block  ;; label = @6
                block  ;; label = @7
                  local.get 8
                  i32.const 8
                  i32.shr_u
                  local.tee 7
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 1
                  br 1 (;@6;)
                end
                i32.const 31
                local.set 1
                local.get 8
                i32.const 16777215
                i32.gt_u
                br_if 0 (;@6;)
                local.get 8
                i32.const 6
                local.get 7
                i32.clz
                local.tee 1
                i32.sub
                i32.const 31
                i32.and
                i32.shr_u
                i32.const 1
                i32.and
                local.get 1
                i32.const 1
                i32.shl
                i32.sub
                i32.const 62
                i32.add
                local.set 1
              end
              local.get 6
              i64.const 0
              i64.store offset=16 align=4
              local.get 6
              i32.const 28
              i32.add
              local.get 1
              i32.store
              local.get 0
              local.get 1
              i32.const 2
              i32.shl
              i32.add
              i32.const 272
              i32.add
              local.set 7
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.const 4
                        i32.add
                        local.tee 9
                        i32.load
                        local.tee 5
                        i32.const 1
                        local.get 1
                        i32.const 31
                        i32.and
                        i32.shl
                        local.tee 4
                        i32.and
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 7
                        i32.load
                        local.tee 9
                        i32.const 4
                        i32.add
                        i32.load
                        i32.const -8
                        i32.and
                        local.get 8
                        i32.ne
                        br_if 1 (;@9;)
                        local.get 9
                        local.set 1
                        br 2 (;@8;)
                      end
                      local.get 9
                      local.get 5
                      local.get 4
                      i32.or
                      i32.store
                      local.get 7
                      local.get 6
                      i32.store
                      local.get 6
                      i32.const 24
                      i32.add
                      local.get 7
                      i32.store
                      br 3 (;@6;)
                    end
                    local.get 8
                    i32.const 0
                    i32.const 25
                    local.get 1
                    i32.const 1
                    i32.shr_u
                    i32.sub
                    i32.const 31
                    i32.and
                    local.get 1
                    i32.const 31
                    i32.eq
                    select
                    i32.shl
                    local.set 7
                    loop  ;; label = @9
                      local.get 9
                      local.get 7
                      i32.const 29
                      i32.shr_u
                      i32.const 4
                      i32.and
                      i32.add
                      i32.const 16
                      i32.add
                      local.tee 5
                      i32.load
                      local.tee 1
                      i32.eqz
                      br_if 2 (;@7;)
                      local.get 7
                      i32.const 1
                      i32.shl
                      local.set 7
                      local.get 1
                      local.set 9
                      local.get 1
                      i32.const 4
                      i32.add
                      i32.load
                      i32.const -8
                      i32.and
                      local.get 8
                      i32.ne
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 1
                  i32.load offset=8
                  local.tee 7
                  local.get 6
                  i32.store offset=12
                  local.get 1
                  local.get 6
                  i32.store offset=8
                  local.get 6
                  i32.const 24
                  i32.add
                  i32.const 0
                  i32.store
                  local.get 6
                  local.get 1
                  i32.store offset=12
                  local.get 6
                  local.get 7
                  i32.store offset=8
                  br 3 (;@4;)
                end
                local.get 5
                local.get 6
                i32.store
                local.get 6
                i32.const 24
                i32.add
                local.get 9
                i32.store
              end
              local.get 6
              local.get 6
              i32.store offset=12
              local.get 6
              local.get 6
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 0
            local.get 8
            i32.const 3
            i32.shr_u
            local.tee 7
            i32.const 3
            i32.shl
            i32.add
            i32.const 8
            i32.add
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 8
                i32.const 1
                local.get 7
                i32.shl
                local.tee 7
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=8
                local.set 7
                br 1 (;@5;)
              end
              local.get 0
              local.get 8
              local.get 7
              i32.or
              i32.store
              local.get 1
              local.set 7
            end
            local.get 1
            local.get 6
            i32.store offset=8
            local.get 7
            local.get 6
            i32.store offset=12
            local.get 6
            local.get 1
            i32.store offset=12
            local.get 6
            local.get 7
            i32.store offset=8
          end
          local.get 0
          i32.load offset=404
          local.tee 1
          local.get 3
          i32.le_u
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          local.get 3
          i32.sub
          local.tee 2
          i32.store offset=404
          local.get 0
          local.get 0
          i32.load offset=412
          local.tee 1
          local.get 3
          i32.add
          local.tee 6
          i32.store offset=412
          local.get 6
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 1
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 1
          i32.const 8
          i32.add
          return
        end
        local.get 6
        local.get 5
        i32.store offset=24
        block  ;; label = @3
          local.get 7
          i32.load offset=16
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 6
          i32.store offset=24
        end
        local.get 7
        i32.const 20
        i32.add
        i32.load
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 20
        i32.add
        local.get 1
        i32.store
        local.get 1
        local.get 6
        i32.store offset=24
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 7
          local.get 3
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 3
          i32.add
          local.tee 3
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 3
          local.get 2
          i32.add
          local.get 2
          i32.store
          block  ;; label = @4
            local.get 0
            i32.load offset=400
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 1
            i32.const 3
            i32.shr_u
            local.tee 8
            i32.const 3
            i32.shl
            i32.add
            i32.const 8
            i32.add
            local.set 6
            local.get 0
            i32.load offset=408
            local.set 1
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 9
                i32.const 1
                local.get 8
                i32.const 31
                i32.and
                i32.shl
                local.tee 8
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.load offset=8
                local.set 8
                br 1 (;@5;)
              end
              local.get 0
              local.get 9
              local.get 8
              i32.or
              i32.store
              local.get 6
              local.set 8
            end
            local.get 6
            local.get 1
            i32.store offset=8
            local.get 8
            local.get 1
            i32.store offset=12
            local.get 1
            local.get 6
            i32.store offset=12
            local.get 1
            local.get 8
            i32.store offset=8
          end
          local.get 0
          local.get 3
          i32.store offset=408
          local.get 0
          local.get 2
          i32.store offset=400
          br 1 (;@2;)
        end
        local.get 7
        local.get 2
        local.get 3
        i32.add
        local.tee 1
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 7
        local.get 1
        i32.add
        local.tee 1
        local.get 1
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
      end
      local.get 7
      i32.const 8
      i32.add
      return
    end
    local.get 2)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 256
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 24
          i32.add
          i32.load
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                i32.load offset=12
                local.tee 4
                local.get 1
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                i32.const 20
                i32.const 16
                local.get 1
                i32.const 20
                i32.add
                local.tee 4
                i32.load
                local.tee 5
                select
                i32.add
                i32.load
                local.tee 2
                br_if 1 (;@5;)
                i32.const 0
                local.set 4
                br 2 (;@4;)
              end
              local.get 1
              i32.load offset=8
              local.tee 2
              local.get 4
              i32.store offset=12
              local.get 4
              local.get 2
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 4
            local.get 1
            i32.const 16
            i32.add
            local.get 5
            select
            local.set 5
            loop  ;; label = @5
              local.get 5
              local.set 6
              block  ;; label = @6
                local.get 2
                local.tee 4
                i32.const 20
                i32.add
                local.tee 5
                i32.load
                local.tee 2
                br_if 0 (;@6;)
                local.get 4
                i32.const 16
                i32.add
                local.set 5
                local.get 4
                i32.load offset=16
                local.set 2
              end
              local.get 2
              br_if 0 (;@5;)
            end
            local.get 6
            i32.const 0
            i32.store
          end
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            local.get 0
            local.get 1
            i32.const 28
            i32.add
            i32.load
            i32.const 2
            i32.shl
            i32.add
            i32.const 272
            i32.add
            local.tee 2
            i32.load
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 16
            i32.const 20
            local.get 3
            i32.load offset=16
            local.get 1
            i32.eq
            select
            i32.add
            local.get 4
            i32.store
            local.get 4
            i32.eqz
            br_if 3 (;@1;)
            br 2 (;@2;)
          end
          local.get 2
          local.get 4
          i32.store
          local.get 4
          br_if 1 (;@2;)
          local.get 0
          local.get 0
          i32.load offset=4
          i32.const -2
          local.get 1
          i32.load offset=28
          i32.rotl
          i32.and
          i32.store offset=4
          return
        end
        block  ;; label = @3
          local.get 1
          i32.const 12
          i32.add
          i32.load
          local.tee 4
          local.get 1
          i32.const 8
          i32.add
          i32.load
          local.tee 5
          i32.eq
          br_if 0 (;@3;)
          local.get 5
          local.get 4
          i32.store offset=12
          local.get 4
          local.get 5
          i32.store offset=8
          return
        end
        local.get 0
        local.get 0
        i32.load
        i32.const -2
        local.get 2
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store
        br 1 (;@1;)
      end
      local.get 4
      local.get 3
      i32.store offset=24
      block  ;; label = @2
        local.get 1
        i32.load offset=16
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 2
        i32.store offset=16
        local.get 2
        local.get 4
        i32.store offset=24
      end
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 20
      i32.add
      local.get 2
      i32.store
      local.get 2
      local.get 4
      i32.store offset=24
      return
    end)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc7realloc17h430fa370263f868fE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.const -65588
      i32.gt_u
      br_if 0 (;@1;)
      i32.const 16
      local.get 2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 2
      i32.const 11
      i32.lt_u
      select
      local.set 4
      local.get 1
      i32.const -4
      i32.add
      local.tee 5
      i32.load
      local.tee 6
      i32.const -8
      i32.and
      local.set 7
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 6
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const -8
                    i32.add
                    local.tee 8
                    local.get 7
                    i32.add
                    local.set 9
                    local.get 7
                    local.get 4
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 0
                    i32.load offset=412
                    local.get 9
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 0
                    i32.load offset=408
                    local.get 9
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 9
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 6
                    i32.const 2
                    i32.and
                    br_if 6 (;@2;)
                    local.get 6
                    i32.const -8
                    i32.and
                    local.tee 6
                    local.get 7
                    i32.add
                    local.tee 7
                    local.get 4
                    i32.ge_u
                    br_if 4 (;@4;)
                    br 6 (;@2;)
                  end
                  local.get 4
                  i32.const 256
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 7
                  local.get 4
                  i32.const 4
                  i32.or
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 7
                  local.get 4
                  i32.sub
                  i32.const 131073
                  i32.ge_u
                  br_if 5 (;@2;)
                  br 4 (;@3;)
                end
                local.get 7
                local.get 4
                i32.sub
                local.tee 2
                i32.const 16
                i32.lt_u
                br_if 3 (;@3;)
                local.get 5
                local.get 4
                local.get 6
                i32.const 1
                i32.and
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 8
                local.get 4
                i32.add
                local.tee 3
                local.get 2
                i32.const 3
                i32.or
                i32.store offset=4
                local.get 9
                local.get 9
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 0
                local.get 3
                local.get 2
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17h02226e42769db0d1E
                br 3 (;@3;)
              end
              local.get 0
              i32.load offset=404
              local.get 7
              i32.add
              local.tee 7
              local.get 4
              i32.le_u
              br_if 3 (;@2;)
              local.get 5
              local.get 4
              local.get 6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 8
              local.get 4
              i32.add
              local.tee 2
              local.get 7
              local.get 4
              i32.sub
              local.tee 3
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 3
              i32.store offset=404
              local.get 0
              local.get 2
              i32.store offset=412
              br 2 (;@3;)
            end
            local.get 0
            i32.load offset=400
            local.get 7
            i32.add
            local.tee 7
            local.get 4
            i32.lt_u
            br_if 2 (;@2;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                local.get 4
                i32.sub
                local.tee 2
                i32.const 15
                i32.gt_u
                br_if 0 (;@6;)
                local.get 5
                local.get 6
                i32.const 1
                i32.and
                local.get 7
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 8
                local.get 7
                i32.add
                local.tee 2
                local.get 2
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set 2
                i32.const 0
                local.set 3
                br 1 (;@5;)
              end
              local.get 5
              local.get 4
              local.get 6
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 8
              local.get 4
              i32.add
              local.tee 3
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 8
              local.get 7
              i32.add
              local.tee 4
              local.get 2
              i32.store
              local.get 4
              local.get 4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
            end
            local.get 0
            local.get 3
            i32.store offset=408
            local.get 0
            local.get 2
            i32.store offset=400
            br 1 (;@3;)
          end
          local.get 0
          local.get 9
          local.get 6
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
          block  ;; label = @4
            local.get 7
            local.get 4
            i32.sub
            local.tee 2
            i32.const 16
            i32.lt_u
            br_if 0 (;@4;)
            local.get 5
            local.get 4
            local.get 5
            i32.load
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 8
            local.get 4
            i32.add
            local.tee 3
            local.get 2
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 8
            local.get 7
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 0
            local.get 3
            local.get 2
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17h02226e42769db0d1E
            br 1 (;@3;)
          end
          local.get 5
          local.get 7
          local.get 5
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 8
          local.get 7
          i32.add
          local.tee 2
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
        end
        local.get 1
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      local.get 1
      local.get 2
      local.get 5
      i32.load
      local.tee 3
      i32.const -8
      i32.and
      i32.const 4
      i32.const 8
      local.get 3
      i32.const 3
      i32.and
      select
      i32.sub
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      call $memcpy
      local.set 2
      local.get 0
      local.get 1
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h25e0c0cdb1422e7eE
      local.get 2
      return
    end
    local.get 3)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17h02226e42769db0d1E (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    i32.add
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 4
              i32.const 1
              i32.and
              br_if 0 (;@5;)
              local.get 4
              i32.const 3
              i32.and
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              i32.load
              local.tee 4
              local.get 2
              i32.add
              local.set 2
              block  ;; label = @6
                local.get 0
                i32.load offset=408
                local.get 1
                local.get 4
                i32.sub
                local.tee 1
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=4
                i32.const 3
                i32.and
                i32.const 3
                i32.ne
                br_if 1 (;@5;)
                local.get 0
                local.get 2
                i32.store offset=400
                local.get 3
                local.get 3
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 1
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 3
                local.get 2
                i32.store
                return
              end
              local.get 0
              local.get 1
              local.get 4
              call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 3
                i32.const 4
                i32.add
                i32.load
                local.tee 4
                i32.const 2
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 4
                i32.add
                local.get 4
                i32.const -2
                i32.and
                i32.store
                local.get 1
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 2
                i32.add
                local.get 2
                i32.store
                br 1 (;@5;)
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=412
                  local.get 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=408
                  local.get 3
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 0
                  local.get 3
                  local.get 4
                  i32.const -8
                  i32.and
                  local.tee 4
                  call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
                  local.get 1
                  local.get 4
                  local.get 2
                  i32.add
                  local.tee 2
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 2
                  i32.add
                  local.get 2
                  i32.store
                  local.get 1
                  local.get 0
                  i32.load offset=408
                  i32.ne
                  br_if 2 (;@5;)
                  local.get 0
                  local.get 2
                  i32.store offset=400
                  return
                end
                local.get 0
                local.get 1
                i32.store offset=412
                local.get 0
                local.get 0
                i32.load offset=404
                local.get 2
                i32.add
                local.tee 2
                i32.store offset=404
                local.get 1
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 1
                local.get 0
                i32.load offset=408
                i32.ne
                br_if 2 (;@4;)
                local.get 0
                i32.const 0
                i32.store offset=400
                local.get 0
                i32.const 0
                i32.store offset=408
                return
              end
              local.get 0
              local.get 1
              i32.store offset=408
              local.get 0
              local.get 0
              i32.load offset=400
              local.get 2
              i32.add
              local.tee 2
              i32.store offset=400
              local.get 1
              local.get 2
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 1
              local.get 2
              i32.add
              local.get 2
              i32.store
              return
            end
            local.get 2
            i32.const 256
            i32.lt_u
            br_if 3 (;@1;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 8
                i32.shr_u
                local.tee 4
                br_if 0 (;@6;)
                i32.const 0
                local.set 3
                br 1 (;@5;)
              end
              i32.const 31
              local.set 3
              local.get 2
              i32.const 16777215
              i32.gt_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 6
              local.get 4
              i32.clz
              local.tee 3
              i32.sub
              i32.const 31
              i32.and
              i32.shr_u
              i32.const 1
              i32.and
              local.get 3
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 3
            end
            local.get 1
            i64.const 0
            i64.store offset=16 align=4
            local.get 1
            i32.const 28
            i32.add
            local.get 3
            i32.store
            local.get 0
            local.get 3
            i32.const 2
            i32.shl
            i32.add
            i32.const 272
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 4
                  i32.add
                  local.tee 0
                  i32.load
                  local.tee 5
                  i32.const 1
                  local.get 3
                  i32.const 31
                  i32.and
                  i32.shl
                  local.tee 6
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load
                  local.tee 4
                  i32.const 4
                  i32.add
                  i32.load
                  i32.const -8
                  i32.and
                  local.get 2
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 4
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 0
                local.get 5
                local.get 6
                i32.or
                i32.store
                local.get 4
                local.get 1
                i32.store
                local.get 1
                i32.const 24
                i32.add
                local.get 4
                i32.store
                br 4 (;@2;)
              end
              local.get 2
              i32.const 0
              i32.const 25
              local.get 3
              i32.const 1
              i32.shr_u
              i32.sub
              i32.const 31
              i32.and
              local.get 3
              i32.const 31
              i32.eq
              select
              i32.shl
              local.set 3
              loop  ;; label = @6
                local.get 4
                local.get 3
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                local.tee 5
                i32.load
                local.tee 0
                i32.eqz
                br_if 3 (;@3;)
                local.get 3
                i32.const 1
                i32.shl
                local.set 3
                local.get 0
                local.set 4
                local.get 0
                i32.const 4
                i32.add
                i32.load
                i32.const -8
                i32.and
                local.get 2
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 0
            i32.load offset=8
            local.tee 2
            local.get 1
            i32.store offset=12
            local.get 0
            local.get 1
            i32.store offset=8
            local.get 1
            i32.const 24
            i32.add
            i32.const 0
            i32.store
            local.get 1
            local.get 0
            i32.store offset=12
            local.get 1
            local.get 2
            i32.store offset=8
          end
          return
        end
        local.get 5
        local.get 1
        i32.store
        local.get 1
        i32.const 24
        i32.add
        local.get 4
        i32.store
      end
      local.get 1
      local.get 1
      i32.store offset=12
      local.get 1
      local.get 1
      i32.store offset=8
      return
    end
    local.get 0
    local.get 2
    i32.const 3
    i32.shr_u
    local.tee 3
    i32.const 3
    i32.shl
    i32.add
    i32.const 8
    i32.add
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 4
        i32.const 1
        local.get 3
        i32.shl
        local.tee 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=8
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      local.get 3
      i32.or
      i32.store
      local.get 2
      local.set 0
    end
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 1
    local.get 2
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc4free17h25e0c0cdb1422e7eE (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const -8
    i32.add
    local.tee 2
    local.get 1
    i32.const -4
    i32.add
    i32.load
    local.tee 3
    i32.const -8
    i32.and
    local.tee 1
    i32.add
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 3
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 3
            i32.const 3
            i32.and
            i32.eqz
            br_if 1 (;@3;)
            local.get 2
            i32.load
            local.tee 3
            local.get 1
            i32.add
            local.set 1
            block  ;; label = @5
              local.get 0
              i32.load offset=408
              local.get 2
              local.get 3
              i32.sub
              local.tee 2
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=4
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 1 (;@4;)
              local.get 0
              local.get 1
              i32.store offset=400
              local.get 4
              local.get 4
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              i32.add
              local.get 1
              i32.store
              return
            end
            local.get 0
            local.get 2
            local.get 3
            call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.const 4
              i32.add
              local.tee 5
              i32.load
              local.tee 3
              i32.const 2
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 5
              local.get 3
              i32.const -2
              i32.and
              i32.store
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              i32.add
              local.get 1
              i32.store
              br 1 (;@4;)
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load offset=412
                local.get 4
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=408
                local.get 4
                i32.eq
                br_if 1 (;@5;)
                local.get 0
                local.get 4
                local.get 3
                i32.const -8
                i32.and
                local.tee 3
                call $_ZN8dlmalloc8dlmalloc8Dlmalloc12unlink_chunk17h157e135eb8712136E
                local.get 2
                local.get 3
                local.get 1
                i32.add
                local.tee 1
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 2
                local.get 1
                i32.add
                local.get 1
                i32.store
                local.get 2
                local.get 0
                i32.load offset=408
                i32.ne
                br_if 2 (;@4;)
                local.get 0
                local.get 1
                i32.store offset=400
                return
              end
              local.get 0
              local.get 2
              i32.store offset=412
              local.get 0
              local.get 0
              i32.load offset=404
              local.get 1
              i32.add
              local.tee 1
              i32.store offset=404
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              block  ;; label = @6
                local.get 2
                local.get 0
                i32.load offset=408
                i32.ne
                br_if 0 (;@6;)
                local.get 0
                i32.const 0
                i32.store offset=400
                local.get 0
                i32.const 0
                i32.store offset=408
              end
              local.get 0
              i32.const 440
              i32.add
              i32.load
              local.tee 3
              local.get 1
              i32.ge_u
              br_if 2 (;@3;)
              local.get 0
              i32.load offset=412
              local.tee 1
              i32.eqz
              br_if 2 (;@3;)
              block  ;; label = @6
                local.get 0
                i32.load offset=404
                local.tee 5
                i32.const 41
                i32.lt_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 424
                i32.add
                local.set 2
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load
                    local.tee 4
                    local.get 1
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 4
                    local.get 2
                    i32.load offset=4
                    i32.add
                    local.get 1
                    i32.gt_u
                    br_if 2 (;@6;)
                  end
                  local.get 2
                  i32.load offset=8
                  local.tee 2
                  br_if 0 (;@7;)
                end
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.const 432
                  i32.add
                  i32.load
                  local.tee 1
                  br_if 0 (;@7;)
                  i32.const 4095
                  local.set 2
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 2
                loop  ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load offset=8
                  local.tee 1
                  br_if 0 (;@7;)
                end
                local.get 2
                i32.const 4095
                local.get 2
                i32.const 4095
                i32.gt_u
                select
                local.set 2
              end
              local.get 0
              local.get 2
              i32.store offset=448
              local.get 5
              local.get 3
              i32.le_u
              br_if 2 (;@3;)
              local.get 0
              i32.const 440
              i32.add
              i32.const -1
              i32.store
              return
            end
            local.get 0
            local.get 2
            i32.store offset=408
            local.get 0
            local.get 0
            i32.load offset=400
            local.get 1
            i32.add
            local.tee 1
            i32.store offset=400
            local.get 2
            local.get 1
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 1
            i32.add
            local.get 1
            i32.store
            return
          end
          local.get 1
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              i32.const 8
              i32.shr_u
              local.tee 3
              br_if 0 (;@5;)
              i32.const 0
              local.set 4
              br 1 (;@4;)
            end
            i32.const 31
            local.set 4
            local.get 1
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 6
            local.get 3
            i32.clz
            local.tee 4
            i32.sub
            i32.const 31
            i32.and
            i32.shr_u
            i32.const 1
            i32.and
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 4
          end
          local.get 2
          i64.const 0
          i64.store offset=16 align=4
          local.get 2
          i32.const 28
          i32.add
          local.get 4
          i32.store
          local.get 0
          local.get 4
          i32.const 2
          i32.shl
          i32.add
          i32.const 272
          i32.add
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 0
                      i32.const 4
                      i32.add
                      local.tee 5
                      i32.load
                      local.tee 6
                      i32.const 1
                      local.get 4
                      i32.const 31
                      i32.and
                      i32.shl
                      local.tee 7
                      i32.and
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 3
                      i32.load
                      local.tee 5
                      i32.const 4
                      i32.add
                      i32.load
                      i32.const -8
                      i32.and
                      local.get 1
                      i32.ne
                      br_if 1 (;@8;)
                      local.get 5
                      local.set 4
                      br 2 (;@7;)
                    end
                    local.get 5
                    local.get 6
                    local.get 7
                    i32.or
                    i32.store
                    local.get 3
                    local.get 2
                    i32.store
                    local.get 2
                    i32.const 24
                    i32.add
                    local.get 3
                    i32.store
                    br 3 (;@5;)
                  end
                  local.get 1
                  i32.const 0
                  i32.const 25
                  local.get 4
                  i32.const 1
                  i32.shr_u
                  i32.sub
                  i32.const 31
                  i32.and
                  local.get 4
                  i32.const 31
                  i32.eq
                  select
                  i32.shl
                  local.set 3
                  loop  ;; label = @8
                    local.get 5
                    local.get 3
                    i32.const 29
                    i32.shr_u
                    i32.const 4
                    i32.and
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 6
                    i32.load
                    local.tee 4
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 3
                    i32.const 1
                    i32.shl
                    local.set 3
                    local.get 4
                    local.set 5
                    local.get 4
                    i32.const 4
                    i32.add
                    i32.load
                    i32.const -8
                    i32.and
                    local.get 1
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                local.get 4
                i32.load offset=8
                local.tee 1
                local.get 2
                i32.store offset=12
                local.get 4
                local.get 2
                i32.store offset=8
                local.get 2
                i32.const 24
                i32.add
                i32.const 0
                i32.store
                local.get 2
                local.get 4
                i32.store offset=12
                local.get 2
                local.get 1
                i32.store offset=8
                br 2 (;@4;)
              end
              local.get 6
              local.get 2
              i32.store
              local.get 2
              i32.const 24
              i32.add
              local.get 5
              i32.store
            end
            local.get 2
            local.get 2
            i32.store offset=12
            local.get 2
            local.get 2
            i32.store offset=8
          end
          local.get 0
          local.get 0
          i32.load offset=448
          i32.const -1
          i32.add
          local.tee 2
          i32.store offset=448
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
        end
        return
      end
      local.get 0
      local.get 1
      i32.const 3
      i32.shr_u
      local.tee 4
      i32.const 3
      i32.shl
      i32.add
      i32.const 8
      i32.add
      local.set 1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load
          local.tee 3
          i32.const 1
          local.get 4
          i32.shl
          local.tee 4
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=8
          local.set 0
          br 1 (;@2;)
        end
        local.get 0
        local.get 3
        local.get 4
        i32.or
        i32.store
        local.get 1
        local.set 0
      end
      local.get 1
      local.get 2
      i32.store offset=8
      local.get 0
      local.get 2
      i32.store offset=12
      local.get 2
      local.get 1
      i32.store offset=12
      local.get 2
      local.get 0
      i32.store offset=8
      return
    end
    block  ;; label = @1
      local.get 0
      i32.const 432
      i32.add
      i32.load
      local.tee 1
      br_if 0 (;@1;)
      local.get 0
      i32.const 4095
      i32.store offset=448
      return
    end
    i32.const 0
    local.set 2
    loop  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.set 2
      local.get 1
      i32.load offset=8
      local.tee 1
      br_if 0 (;@1;)
    end
    local.get 0
    local.get 2
    i32.const 4095
    local.get 2
    i32.const 4095
    i32.gt_u
    select
    i32.store offset=448)
  (func $_ZN8dlmalloc8dlmalloc8Dlmalloc8memalign17he5847471231aa01aE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      i32.const -65587
      local.get 1
      i32.const 16
      local.get 1
      i32.const 16
      i32.gt_u
      select
      local.tee 1
      i32.sub
      local.get 2
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i32.const 16
      local.get 2
      i32.const 11
      i32.add
      i32.const -8
      i32.and
      local.get 2
      i32.const 11
      i32.lt_u
      select
      local.tee 4
      i32.add
      i32.const 12
      i32.add
      call $_ZN8dlmalloc8dlmalloc8Dlmalloc6malloc17hebac3e958590d8ceE
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.const -8
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const -1
          i32.add
          local.tee 5
          local.get 2
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 1
          br 1 (;@2;)
        end
        local.get 2
        i32.const -4
        i32.add
        local.tee 6
        i32.load
        local.tee 7
        i32.const -8
        i32.and
        local.get 5
        local.get 2
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        i32.and
        i32.const -8
        i32.add
        local.tee 2
        local.get 2
        local.get 1
        i32.add
        local.get 2
        local.get 3
        i32.sub
        i32.const 16
        i32.gt_u
        select
        local.tee 1
        local.get 3
        i32.sub
        local.tee 2
        i32.sub
        local.set 5
        block  ;; label = @3
          local.get 7
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store offset=4
          local.get 1
          local.get 5
          i32.add
          local.tee 5
          local.get 5
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 6
          local.get 2
          local.get 6
          i32.load
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          local.get 3
          local.get 2
          call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17h02226e42769db0d1E
          br 1 (;@2;)
        end
        local.get 3
        i32.load
        local.set 3
        local.get 1
        local.get 5
        i32.store offset=4
        local.get 1
        local.get 3
        local.get 2
        i32.add
        i32.store
      end
      block  ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 2
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const -8
        i32.and
        local.tee 3
        local.get 4
        i32.const 16
        i32.add
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        local.get 4
        local.get 2
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 1
        local.get 4
        i32.add
        local.tee 2
        local.get 3
        local.get 4
        i32.sub
        local.tee 4
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        local.get 3
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 2
        local.get 4
        call $_ZN8dlmalloc8dlmalloc8Dlmalloc13dispose_chunk17h02226e42769db0d1E
      end
      local.get 1
      i32.const 8
      i32.add
      local.set 3
    end
    local.get 3)
  (func $_ZN5alloc5alloc18handle_alloc_error17he62175e02bb3fceeE (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $rust_oom
    unreachable)
  (func $_ZN5alloc7raw_vec17capacity_overflow17ha369776b076180eeE (type 6)
    i32.const 1049379
    i32.const 17
    i32.const 1049396
    call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
    unreachable)
  (func $_ZN4core3ops8function6FnOnce9call_once17hbb1f81e74ea5afcbE (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core3ptr13drop_in_place17h0200b96f86cedf64E (type 5) (param i32))
  (func $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE (type 7) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 18
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1049556
    i32.store offset=8
    local.get 3
    i32.const 18
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.store offset=40
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
    unreachable)
  (func $_ZN4core9panicking5panic17h94c4b8c5a269b671E (type 7) (param i32 i32 i32)
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
    i32.const 1049412
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
    call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
    unreachable)
  (func $_ZN4core5slice20slice_index_len_fail17h6e4ffc56e8460238E (type 7) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 18
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1050016
    i32.store offset=8
    local.get 3
    i32.const 18
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
    unreachable)
  (func $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE (type 7) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 28
    i32.add
    i32.const 2
    i32.store
    local.get 3
    i32.const 44
    i32.add
    i32.const 18
    i32.store
    local.get 3
    i64.const 2
    i64.store offset=12 align=4
    local.get 3
    i32.const 1050068
    i32.store offset=8
    local.get 3
    i32.const 18
    i32.store offset=36
    local.get 3
    local.get 3
    i32.const 32
    i32.add
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 4
    i32.add
    i32.store offset=40
    local.get 3
    local.get 3
    i32.store offset=32
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
    unreachable)
  (func $_ZN4core3fmt9Formatter3pad17h671c63eb2e61967cE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=16
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 4
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 3
            i32.const 1
            i32.eq
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 1)
            local.set 3
            br 3 (;@1;)
          end
          local.get 3
          i32.const 1
          i32.ne
          br_if 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 2
            br 1 (;@3;)
          end
          local.get 1
          local.get 2
          i32.add
          local.set 5
          local.get 0
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 7
          local.get 1
          local.set 3
          local.get 1
          local.set 8
          loop  ;; label = @4
            local.get 3
            i32.const 1
            i32.add
            local.set 9
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  i32.load8_s
                  local.tee 10
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 9
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 11
                      local.get 5
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u offset=1
                    i32.const 63
                    i32.and
                    local.set 11
                    local.get 3
                    i32.const 2
                    i32.add
                    local.tee 9
                    local.set 3
                  end
                  local.get 10
                  i32.const 31
                  i32.and
                  local.set 12
                  block  ;; label = @8
                    local.get 10
                    i32.const 255
                    i32.and
                    local.tee 10
                    i32.const 223
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 6
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 13
                      local.get 5
                      local.set 14
                      br 1 (;@8;)
                    end
                    local.get 3
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 13
                    local.get 3
                    i32.const 1
                    i32.add
                    local.tee 9
                    local.set 14
                  end
                  local.get 13
                  local.get 11
                  i32.const 6
                  i32.shl
                  i32.or
                  local.set 11
                  block  ;; label = @8
                    local.get 10
                    i32.const 240
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 11
                    local.get 12
                    i32.const 12
                    i32.shl
                    i32.or
                    local.set 10
                    br 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 14
                      local.get 5
                      i32.ne
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 10
                      local.get 9
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 14
                    i32.const 1
                    i32.add
                    local.set 3
                    local.get 14
                    i32.load8_u
                    i32.const 63
                    i32.and
                    local.set 10
                  end
                  local.get 11
                  i32.const 6
                  i32.shl
                  local.get 12
                  i32.const 18
                  i32.shl
                  i32.const 1835008
                  i32.and
                  i32.or
                  local.get 10
                  i32.or
                  local.tee 10
                  i32.const 1114112
                  i32.ne
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                local.get 10
                i32.const 255
                i32.and
                local.set 10
              end
              local.get 9
              local.set 3
            end
            block  ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 8
              i32.sub
              local.get 3
              i32.add
              local.set 7
              local.get 3
              local.set 8
              local.get 5
              local.get 3
              i32.ne
              br_if 1 (;@4;)
              br 2 (;@3;)
            end
          end
          local.get 10
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              local.get 2
              i32.eq
              br_if 0 (;@5;)
              i32.const 0
              local.set 3
              local.get 7
              local.get 2
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 7
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 1 (;@4;)
            end
            local.get 1
            local.set 3
          end
          local.get 7
          local.get 2
          local.get 3
          select
          local.set 2
          local.get 3
          local.get 1
          local.get 3
          select
          local.set 1
        end
        local.get 4
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 2
        local.get 9
        i32.sub
        local.get 0
        i32.load offset=12
        local.tee 6
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      i32.const 0
      local.set 7
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 9
        local.get 2
        local.set 10
        local.get 1
        local.set 3
        loop  ;; label = @3
          local.get 9
          local.get 3
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 9
      local.get 2
      i32.sub
      local.get 6
      i32.add
      local.tee 9
      local.set 10
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            local.get 0
            i32.load8_u offset=32
            local.tee 3
            local.get 3
            i32.const 3
            i32.eq
            select
            br_table 2 (;@2;) 1 (;@3;) 0 (;@4;) 1 (;@3;) 2 (;@2;)
          end
          local.get 9
          i32.const 1
          i32.shr_u
          local.set 7
          local.get 9
          i32.const 1
          i32.add
          i32.const 1
          i32.shr_u
          local.set 10
          br 1 (;@2;)
        end
        i32.const 0
        local.set 10
        local.get 9
        local.set 7
      end
      local.get 7
      i32.const 1
      i32.add
      local.set 3
      block  ;; label = @2
        loop  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=24
          local.get 0
          i32.load offset=4
          local.get 0
          i32.load offset=28
          i32.load offset=16
          call_indirect (type 2)
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1
        return
      end
      local.get 0
      i32.load offset=4
      local.set 9
      i32.const 1
      local.set 3
      local.get 0
      i32.load offset=24
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 10
      i32.const 1
      i32.add
      local.set 3
      local.get 0
      i32.load offset=28
      local.set 10
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          i32.const -1
          i32.add
          local.tee 3
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        local.get 0
        local.get 9
        local.get 10
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
      i32.const 1
      return
    end
    local.get 3)
  (func $_ZN4core3str16slice_error_fail17hbfdd88935fdca5eaE (type 12) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    i32.const 1
    local.set 6
    local.get 1
    local.set 7
    block  ;; label = @1
      local.get 1
      i32.const 257
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.sub
      local.set 8
      i32.const 256
      local.set 9
      loop  ;; label = @2
        block  ;; label = @3
          local.get 9
          local.get 1
          i32.ge_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 6
          local.get 0
          local.get 9
          i32.add
          i32.load8_s
          i32.const -65
          i32.le_s
          br_if 0 (;@3;)
          local.get 9
          local.set 7
          br 2 (;@1;)
        end
        local.get 9
        i32.const -1
        i32.add
        local.set 7
        i32.const 0
        local.set 6
        local.get 9
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
        local.get 8
        local.get 9
        i32.add
        local.set 10
        local.get 7
        local.set 9
        local.get 10
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 5
    local.get 7
    i32.store offset=20
    local.get 5
    local.get 0
    i32.store offset=16
    local.get 5
    i32.const 0
    i32.const 5
    local.get 6
    select
    i32.store offset=28
    local.get 5
    i32.const 1049412
    i32.const 1050184
    local.get 6
    select
    i32.store offset=24
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            local.get 1
            i32.gt_u
            local.tee 6
            br_if 0 (;@4;)
            local.get 3
            local.get 1
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 3
            i32.gt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                local.get 2
                i32.le_u
                br_if 1 (;@5;)
                local.get 0
                local.get 2
                i32.add
                i32.load8_s
                i32.const -64
                i32.lt_s
                br_if 1 (;@5;)
              end
              local.get 3
              local.set 2
            end
            local.get 5
            local.get 2
            i32.store offset=32
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.eq
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.add
            local.set 9
            loop  ;; label = @5
              block  ;; label = @6
                local.get 2
                local.get 1
                i32.ge_u
                br_if 0 (;@6;)
                local.get 0
                local.get 2
                i32.add
                i32.load8_s
                i32.const -64
                i32.ge_s
                br_if 4 (;@2;)
              end
              local.get 2
              i32.const -1
              i32.add
              local.set 6
              local.get 2
              i32.const 1
              i32.eq
              br_if 4 (;@1;)
              local.get 9
              local.get 2
              i32.eq
              local.set 3
              local.get 6
              local.set 2
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          local.get 5
          local.get 2
          local.get 3
          local.get 6
          select
          i32.store offset=40
          local.get 5
          i32.const 48
          i32.add
          i32.const 20
          i32.add
          i32.const 3
          i32.store
          local.get 5
          i32.const 72
          i32.add
          i32.const 20
          i32.add
          i32.const 19
          i32.store
          local.get 5
          i32.const 84
          i32.add
          i32.const 19
          i32.store
          local.get 5
          i64.const 3
          i64.store offset=52 align=4
          local.get 5
          i32.const 1050224
          i32.store offset=48
          local.get 5
          i32.const 18
          i32.store offset=76
          local.get 5
          local.get 5
          i32.const 72
          i32.add
          i32.store offset=64
          local.get 5
          local.get 5
          i32.const 24
          i32.add
          i32.store offset=88
          local.get 5
          local.get 5
          i32.const 16
          i32.add
          i32.store offset=80
          local.get 5
          local.get 5
          i32.const 40
          i32.add
          i32.store offset=72
          local.get 5
          i32.const 48
          i32.add
          local.get 4
          call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
          unreachable
        end
        local.get 5
        i32.const 100
        i32.add
        i32.const 19
        i32.store
        local.get 5
        i32.const 72
        i32.add
        i32.const 20
        i32.add
        i32.const 19
        i32.store
        local.get 5
        i32.const 84
        i32.add
        i32.const 18
        i32.store
        local.get 5
        i32.const 48
        i32.add
        i32.const 20
        i32.add
        i32.const 4
        i32.store
        local.get 5
        i64.const 4
        i64.store offset=52 align=4
        local.get 5
        i32.const 1050284
        i32.store offset=48
        local.get 5
        i32.const 18
        i32.store offset=76
        local.get 5
        local.get 5
        i32.const 72
        i32.add
        i32.store offset=64
        local.get 5
        local.get 5
        i32.const 24
        i32.add
        i32.store offset=96
        local.get 5
        local.get 5
        i32.const 16
        i32.add
        i32.store offset=88
        local.get 5
        local.get 5
        i32.const 12
        i32.add
        i32.store offset=80
        local.get 5
        local.get 5
        i32.const 8
        i32.add
        i32.store offset=72
        local.get 5
        i32.const 48
        i32.add
        local.get 4
        call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
        unreachable
      end
      local.get 2
      local.set 6
    end
    block  ;; label = @1
      local.get 6
      local.get 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1
      local.set 9
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              local.get 6
              i32.add
              local.tee 3
              i32.load8_s
              local.tee 2
              i32.const -1
              i32.gt_s
              br_if 0 (;@5;)
              i32.const 0
              local.set 9
              local.get 0
              local.get 1
              i32.add
              local.tee 1
              local.set 7
              block  ;; label = @6
                local.get 3
                i32.const 1
                i32.add
                local.get 1
                i32.eq
                br_if 0 (;@6;)
                local.get 3
                i32.const 2
                i32.add
                local.set 7
                local.get 3
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 9
              end
              local.get 2
              i32.const 31
              i32.and
              local.set 3
              local.get 2
              i32.const 255
              i32.and
              i32.const 223
              i32.gt_u
              br_if 1 (;@4;)
              local.get 9
              local.get 3
              i32.const 6
              i32.shl
              i32.or
              local.set 2
              br 2 (;@3;)
            end
            local.get 5
            local.get 2
            i32.const 255
            i32.and
            i32.store offset=36
            local.get 5
            i32.const 40
            i32.add
            local.set 1
            br 2 (;@2;)
          end
          i32.const 0
          local.set 0
          local.get 1
          local.set 8
          block  ;; label = @4
            local.get 7
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 7
            i32.const 1
            i32.add
            local.set 8
            local.get 7
            i32.load8_u
            i32.const 63
            i32.and
            local.set 0
          end
          local.get 0
          local.get 9
          i32.const 6
          i32.shl
          i32.or
          local.set 9
          block  ;; label = @4
            local.get 2
            i32.const 255
            i32.and
            i32.const 240
            i32.ge_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            i32.const 12
            i32.shl
            i32.or
            local.set 2
            br 1 (;@3;)
          end
          i32.const 0
          local.set 2
          block  ;; label = @4
            local.get 8
            local.get 1
            i32.eq
            br_if 0 (;@4;)
            local.get 8
            i32.load8_u
            i32.const 63
            i32.and
            local.set 2
          end
          local.get 9
          i32.const 6
          i32.shl
          local.get 3
          i32.const 18
          i32.shl
          i32.const 1835008
          i32.and
          i32.or
          local.get 2
          i32.or
          local.tee 2
          i32.const 1114112
          i32.eq
          br_if 2 (;@1;)
        end
        local.get 5
        local.get 2
        i32.store offset=36
        i32.const 1
        local.set 9
        local.get 5
        i32.const 40
        i32.add
        local.set 1
        local.get 2
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 9
        local.get 2
        i32.const 2048
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 3
        i32.const 4
        local.get 2
        i32.const 65536
        i32.lt_u
        select
        local.set 9
      end
      local.get 5
      local.get 6
      i32.store offset=40
      local.get 5
      local.get 9
      local.get 6
      i32.add
      i32.store offset=44
      local.get 5
      i32.const 48
      i32.add
      i32.const 20
      i32.add
      i32.const 5
      i32.store
      local.get 5
      i32.const 108
      i32.add
      i32.const 19
      i32.store
      local.get 5
      i32.const 100
      i32.add
      i32.const 19
      i32.store
      local.get 5
      i32.const 72
      i32.add
      i32.const 20
      i32.add
      i32.const 20
      i32.store
      local.get 5
      i32.const 84
      i32.add
      i32.const 21
      i32.store
      local.get 5
      i64.const 5
      i64.store offset=52 align=4
      local.get 5
      i32.const 1050368
      i32.store offset=48
      local.get 5
      local.get 1
      i32.store offset=88
      local.get 5
      i32.const 18
      i32.store offset=76
      local.get 5
      local.get 5
      i32.const 72
      i32.add
      i32.store offset=64
      local.get 5
      local.get 5
      i32.const 24
      i32.add
      i32.store offset=104
      local.get 5
      local.get 5
      i32.const 16
      i32.add
      i32.store offset=96
      local.get 5
      local.get 5
      i32.const 36
      i32.add
      i32.store offset=80
      local.get 5
      local.get 5
      i32.const 32
      i32.add
      i32.store offset=72
      local.get 5
      i32.const 48
      i32.add
      local.get 4
      call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
      unreachable
    end
    i32.const 1049424
    i32.const 43
    local.get 4
    call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E (type 0) (param i32 i32)
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
    i32.const 1049488
    i32.store offset=4
    local.get 2
    i32.const 1049412
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hd7239ff7b4d279a9E (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load32_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h5081cd6222065ff2E)
  (func $_ZN4core3fmt5write17h8b996d8af01475c9E (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=40
    local.get 3
    i64.const 137438953472
    i64.store offset=8
    local.get 3
    local.get 0
    i32.store offset=32
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.load offset=8
              local.tee 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.load
              local.set 5
              local.get 2
              i32.load offset=4
              local.tee 6
              local.get 2
              i32.const 12
              i32.add
              i32.load
              local.tee 7
              local.get 7
              local.get 6
              i32.gt_u
              select
              local.tee 7
              i32.eqz
              br_if 1 (;@4;)
              local.get 2
              i32.const 20
              i32.add
              i32.load
              local.set 8
              local.get 2
              i32.load offset=16
              local.set 9
              local.get 0
              local.get 5
              i32.load
              local.get 5
              i32.load offset=4
              local.get 1
              i32.load offset=12
              call_indirect (type 1)
              br_if 3 (;@2;)
              local.get 5
              i32.const 12
              i32.add
              local.set 2
              i32.const 0
              local.set 10
              block  ;; label = @6
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 3
                    local.get 4
                    i32.const 4
                    i32.add
                    i32.load
                    i32.store offset=12
                    local.get 3
                    local.get 4
                    i32.const 28
                    i32.add
                    i32.load8_u
                    i32.store8 offset=40
                    local.get 3
                    local.get 4
                    i32.const 8
                    i32.add
                    i32.load
                    i32.store offset=8
                    local.get 4
                    i32.const 24
                    i32.add
                    i32.load
                    local.set 0
                    i32.const 0
                    local.set 1
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.const 20
                          i32.add
                          i32.load
                          br_table 1 (;@10;) 0 (;@11;) 2 (;@9;) 1 (;@10;)
                        end
                        local.get 0
                        local.get 8
                        i32.ge_u
                        br_if 3 (;@7;)
                        local.get 0
                        i32.const 3
                        i32.shl
                        local.set 11
                        i32.const 0
                        local.set 1
                        local.get 9
                        local.get 11
                        i32.add
                        local.tee 11
                        i32.load offset=4
                        i32.const 22
                        i32.ne
                        br_if 1 (;@9;)
                        local.get 11
                        i32.load
                        i32.load
                        local.set 0
                      end
                      i32.const 1
                      local.set 1
                    end
                    local.get 3
                    local.get 0
                    i32.store offset=20
                    local.get 3
                    local.get 1
                    i32.store offset=16
                    local.get 4
                    i32.const 16
                    i32.add
                    i32.load
                    local.set 0
                    i32.const 0
                    local.set 1
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.const 12
                          i32.add
                          i32.load
                          br_table 1 (;@10;) 0 (;@11;) 2 (;@9;) 1 (;@10;)
                        end
                        local.get 0
                        local.get 8
                        i32.ge_u
                        br_if 4 (;@6;)
                        local.get 0
                        i32.const 3
                        i32.shl
                        local.set 11
                        local.get 9
                        local.get 11
                        i32.add
                        local.tee 11
                        i32.load offset=4
                        i32.const 22
                        i32.ne
                        br_if 1 (;@9;)
                        local.get 11
                        i32.load
                        i32.load
                        local.set 0
                      end
                      i32.const 1
                      local.set 1
                    end
                    local.get 3
                    local.get 0
                    i32.store offset=28
                    local.get 3
                    local.get 1
                    i32.store offset=24
                    block  ;; label = @9
                      local.get 4
                      i32.load
                      local.tee 0
                      local.get 8
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 9
                      local.get 0
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 0
                      i32.load
                      local.get 3
                      i32.const 8
                      i32.add
                      local.get 0
                      i32.load offset=4
                      call_indirect (type 2)
                      br_if 7 (;@2;)
                      local.get 10
                      i32.const 1
                      i32.add
                      local.tee 10
                      local.get 7
                      i32.ge_u
                      br_if 6 (;@3;)
                      local.get 4
                      i32.const 32
                      i32.add
                      local.set 4
                      local.get 2
                      i32.const -4
                      i32.add
                      local.set 0
                      local.get 2
                      i32.load
                      local.set 1
                      local.get 2
                      i32.const 8
                      i32.add
                      local.set 2
                      local.get 3
                      i32.load offset=32
                      local.get 0
                      i32.load
                      local.get 1
                      local.get 3
                      i32.load offset=36
                      i32.load offset=12
                      call_indirect (type 1)
                      i32.eqz
                      br_if 1 (;@8;)
                      br 7 (;@2;)
                    end
                  end
                  local.get 0
                  local.get 8
                  i32.const 1049900
                  call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
                  unreachable
                end
                local.get 0
                local.get 8
                i32.const 1049916
                call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
                unreachable
              end
              local.get 0
              local.get 8
              i32.const 1049916
              call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
              unreachable
            end
            local.get 2
            i32.load
            local.set 5
            local.get 2
            i32.load offset=4
            local.tee 6
            local.get 2
            i32.const 20
            i32.add
            i32.load
            local.tee 4
            local.get 4
            local.get 6
            i32.gt_u
            select
            local.tee 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=16
            local.set 4
            local.get 0
            local.get 5
            i32.load
            local.get 5
            i32.load offset=4
            local.get 1
            i32.load offset=12
            call_indirect (type 1)
            br_if 2 (;@2;)
            local.get 5
            i32.const 12
            i32.add
            local.set 2
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 4
              i32.load
              local.get 3
              i32.const 8
              i32.add
              local.get 4
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 2)
              br_if 3 (;@2;)
              local.get 0
              i32.const 1
              i32.add
              local.tee 0
              local.get 7
              i32.ge_u
              br_if 2 (;@3;)
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              local.get 2
              i32.const -4
              i32.add
              local.set 1
              local.get 2
              i32.load
              local.set 10
              local.get 2
              i32.const 8
              i32.add
              local.set 2
              local.get 3
              i32.load offset=32
              local.get 1
              i32.load
              local.get 10
              local.get 3
              i32.load offset=36
              i32.load offset=12
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          i32.const 0
          local.set 7
        end
        block  ;; label = @3
          local.get 6
          local.get 7
          i32.le_u
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=32
          local.get 5
          local.get 7
          i32.const 3
          i32.shl
          i32.add
          local.tee 4
          i32.load
          local.get 4
          i32.load offset=4
          local.get 3
          i32.load offset=36
          i32.load offset=12
          call_indirect (type 1)
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h8c8a97b866010196E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hd9ccad900dc49373E
        br_if 0 (;@2;)
        local.get 1
        i32.const 28
        i32.add
        i32.load
        local.set 3
        local.get 1
        i32.load offset=24
        local.set 4
        local.get 2
        i32.const 28
        i32.add
        i32.const 0
        i32.store
        local.get 2
        i32.const 1049412
        i32.store offset=24
        local.get 2
        i64.const 1
        i64.store offset=12 align=4
        local.get 2
        i32.const 1049416
        i32.store offset=8
        local.get 4
        local.get 3
        local.get 2
        i32.const 8
        i32.add
        call $_ZN4core3fmt5write17h8b996d8af01475c9E
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 32
      i32.add
      global.set 0
      i32.const 1
      return
    end
    local.get 0
    i32.const 4
    i32.add
    local.get 1
    call $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hd9ccad900dc49373E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hd9ccad900dc49373E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
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
              local.get 1
              i32.load
              local.tee 3
              i32.const 16
              i32.and
              br_if 0 (;@5;)
              local.get 0
              i32.load
              local.set 4
              local.get 3
              i32.const 32
              i32.and
              br_if 1 (;@4;)
              local.get 4
              i64.extend_i32_u
              i32.const 1
              local.get 1
              call $_ZN4core3fmt3num3imp7fmt_u6417h5081cd6222065ff2E
              local.set 0
              br 2 (;@3;)
            end
            local.get 0
            i32.load
            local.set 4
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 2
              local.get 0
              i32.add
              i32.const 127
              i32.add
              local.get 4
              i32.const 15
              i32.and
              local.tee 3
              i32.const 48
              i32.or
              local.get 3
              i32.const 87
              i32.add
              local.get 3
              i32.const 10
              i32.lt_u
              select
              i32.store8
              local.get 0
              i32.const -1
              i32.add
              local.set 0
              local.get 4
              i32.const 4
              i32.shr_u
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 0
            i32.const 128
            i32.add
            local.tee 4
            i32.const 129
            i32.ge_u
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.const 1049648
            i32.const 2
            local.get 2
            local.get 0
            i32.add
            i32.const 128
            i32.add
            i32.const 0
            local.get 0
            i32.sub
            call $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E
            local.set 0
            br 1 (;@3;)
          end
          i32.const 0
          local.set 0
          loop  ;; label = @4
            local.get 2
            local.get 0
            i32.add
            i32.const 127
            i32.add
            local.get 4
            i32.const 15
            i32.and
            local.tee 3
            i32.const 48
            i32.or
            local.get 3
            i32.const 55
            i32.add
            local.get 3
            i32.const 10
            i32.lt_u
            select
            i32.store8
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 4
            i32.const 4
            i32.shr_u
            local.tee 4
            br_if 0 (;@4;)
          end
          local.get 0
          i32.const 128
          i32.add
          local.tee 4
          i32.const 129
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.const 1
          i32.const 1049648
          i32.const 2
          local.get 2
          local.get 0
          i32.add
          i32.const 128
          i32.add
          i32.const 0
          local.get 0
          i32.sub
          call $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E
          local.set 0
        end
        local.get 2
        i32.const 128
        i32.add
        global.set 0
        local.get 0
        return
      end
      local.get 4
      i32.const 128
      i32.const 1049632
      call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
      unreachable
    end
    local.get 4
    i32.const 128
    i32.const 1049632
    call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
    unreachable)
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h46f25525116e9d7cE (type 10) (param i32) (result i64)
    i64.const -3994463020707609660)
  (func $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h67ec6315566da26cE (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN4core3fmt9Formatter3pad17h671c63eb2e61967cE)
  (func $_ZN4core6option18expect_none_failed17h807851172125790dE (type 12) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 1
    i32.store offset=12
    local.get 5
    local.get 0
    i32.store offset=8
    local.get 5
    local.get 3
    i32.store offset=20
    local.get 5
    local.get 2
    i32.store offset=16
    local.get 5
    i32.const 44
    i32.add
    i32.const 2
    i32.store
    local.get 5
    i32.const 60
    i32.add
    i32.const 23
    i32.store
    local.get 5
    i64.const 2
    i64.store offset=28 align=4
    local.get 5
    i32.const 1049472
    i32.store offset=24
    local.get 5
    i32.const 19
    i32.store offset=52
    local.get 5
    local.get 5
    i32.const 48
    i32.add
    i32.store offset=40
    local.get 5
    local.get 5
    i32.const 16
    i32.add
    i32.store offset=56
    local.get 5
    local.get 5
    i32.const 8
    i32.add
    i32.store offset=48
    local.get 5
    i32.const 24
    i32.add
    local.get 4
    call $_ZN4core9panicking9panic_fmt17h62f7e605cdf65d83E
    unreachable)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h4e3d92a201093296E (type 2) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 2))
  (func $_ZN4core5panic9PanicInfo7message17ha07d8699a1298dc8E (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=8)
  (func $_ZN4core5panic9PanicInfo8location17h24feadd41cc759ecE (type 4) (param i32) (result i32)
    local.get 0
    i32.load offset=12)
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h636f7a801c3cd6dcE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const 40
      i32.add
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  i32.load offset=8
                  i32.load8_u
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load
                  i32.const 1049596
                  i32.const 4
                  local.get 0
                  i32.load offset=4
                  i32.load offset=12
                  call_indirect (type 1)
                  br_if 5 (;@2;)
                end
                local.get 3
                i32.const 10
                i32.store offset=40
                local.get 3
                i64.const 4294967306
                i64.store offset=32
                local.get 3
                local.get 2
                i32.store offset=28
                local.get 3
                i32.const 0
                i32.store offset=24
                local.get 3
                local.get 2
                i32.store offset=20
                local.get 3
                local.get 1
                i32.store offset=16
                local.get 3
                i32.const 8
                i32.add
                i32.const 10
                local.get 1
                local.get 2
                call $_ZN4core5slice6memchr6memchr17hc741e82cd00061e5E
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 3
                        i32.load offset=8
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.load offset=12
                        local.set 4
                        loop  ;; label = @11
                          local.get 3
                          local.get 4
                          local.get 3
                          i32.load offset=24
                          i32.add
                          i32.const 1
                          i32.add
                          local.tee 4
                          i32.store offset=24
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 4
                              local.get 3
                              i32.load offset=36
                              local.tee 6
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 3
                              i32.load offset=20
                              local.set 7
                              br 1 (;@12;)
                            end
                            local.get 3
                            i32.load offset=20
                            local.tee 7
                            local.get 4
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 6
                            i32.const 5
                            i32.ge_u
                            br_if 7 (;@5;)
                            local.get 3
                            i32.load offset=16
                            local.get 4
                            local.get 6
                            i32.sub
                            local.tee 8
                            i32.add
                            local.tee 9
                            local.get 5
                            i32.eq
                            br_if 4 (;@8;)
                            local.get 9
                            local.get 5
                            local.get 6
                            call $bcmp
                            i32.eqz
                            br_if 4 (;@8;)
                          end
                          local.get 3
                          i32.load offset=28
                          local.tee 9
                          local.get 4
                          i32.lt_u
                          br_if 2 (;@9;)
                          local.get 7
                          local.get 9
                          i32.lt_u
                          br_if 2 (;@9;)
                          local.get 3
                          local.get 6
                          local.get 3
                          i32.const 16
                          i32.add
                          i32.add
                          i32.const 23
                          i32.add
                          i32.load8_u
                          local.get 3
                          i32.load offset=16
                          local.get 4
                          i32.add
                          local.get 9
                          local.get 4
                          i32.sub
                          call $_ZN4core5slice6memchr6memchr17hc741e82cd00061e5E
                          local.get 3
                          i32.load offset=4
                          local.set 4
                          local.get 3
                          i32.load
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 3
                      local.get 3
                      i32.load offset=28
                      i32.store offset=24
                    end
                    local.get 0
                    i32.load offset=8
                    i32.const 0
                    i32.store8
                    local.get 2
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=8
                  i32.const 1
                  i32.store8
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 4
                end
                local.get 0
                i32.load offset=4
                local.set 9
                local.get 0
                i32.load
                local.set 6
                block  ;; label = @7
                  local.get 4
                  i32.eqz
                  local.get 2
                  local.get 4
                  i32.eq
                  i32.or
                  local.tee 7
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 4
                  i32.le_u
                  br_if 3 (;@4;)
                  local.get 1
                  local.get 4
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 3 (;@4;)
                end
                local.get 6
                local.get 1
                local.get 4
                local.get 9
                i32.load offset=12
                call_indirect (type 1)
                br_if 4 (;@2;)
                block  ;; label = @7
                  local.get 7
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 4
                  i32.le_u
                  br_if 4 (;@3;)
                  local.get 1
                  local.get 4
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 4 (;@3;)
                end
                local.get 1
                local.get 4
                i32.add
                local.set 1
                local.get 2
                local.get 4
                i32.sub
                local.tee 2
                br_if 0 (;@6;)
              end
              i32.const 0
              local.set 4
              br 4 (;@1;)
            end
            local.get 6
            i32.const 4
            i32.const 1050112
            call $_ZN4core5slice20slice_index_len_fail17h6e4ffc56e8460238E
            unreachable
          end
          local.get 1
          local.get 2
          i32.const 0
          local.get 4
          i32.const 1050152
          call $_ZN4core3str16slice_error_fail17hbfdd88935fdca5eaE
          unreachable
        end
        local.get 1
        local.get 2
        local.get 4
        local.get 2
        i32.const 1050168
        call $_ZN4core3str16slice_error_fail17hbfdd88935fdca5eaE
        unreachable
      end
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN4core5slice6memchr6memchr17hc741e82cd00061e5E (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    i32.const 0
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 3
            i32.and
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            i32.const 4
            local.get 5
            i32.sub
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 5
            local.get 5
            local.get 3
            i32.gt_u
            select
            local.tee 6
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            local.get 1
            i32.const 255
            i32.and
            local.set 4
            loop  ;; label = @5
              local.get 2
              local.get 5
              i32.add
              i32.load8_u
              local.get 4
              i32.eq
              br_if 2 (;@3;)
              local.get 6
              local.get 5
              i32.const 1
              i32.add
              local.tee 5
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 6
            local.set 4
          end
          local.get 3
          i32.const 8
          i32.lt_u
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.const -8
          i32.add
          local.tee 7
          i32.gt_u
          br_if 1 (;@2;)
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 5
          block  ;; label = @4
            loop  ;; label = @5
              local.get 2
              local.get 4
              i32.add
              local.tee 6
              i32.const 4
              i32.add
              i32.load
              local.get 5
              i32.xor
              local.tee 8
              i32.const -1
              i32.xor
              local.get 8
              i32.const -16843009
              i32.add
              i32.and
              local.get 6
              i32.load
              local.get 5
              i32.xor
              local.tee 6
              i32.const -1
              i32.xor
              local.get 6
              i32.const -16843009
              i32.add
              i32.and
              i32.or
              i32.const -2139062144
              i32.and
              br_if 1 (;@4;)
              local.get 4
              i32.const 8
              i32.add
              local.tee 4
              local.get 7
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 4
          local.get 3
          i32.le_u
          br_if 1 (;@2;)
          local.get 4
          local.get 3
          i32.const 1049960
          call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
          unreachable
        end
        i32.const 1
        local.set 6
        br 1 (;@1;)
      end
      i32.const 0
      local.set 5
      i32.const 0
      local.set 6
      block  ;; label = @2
        local.get 4
        local.get 3
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        i32.add
        local.set 2
        local.get 3
        local.get 4
        i32.sub
        local.set 8
        i32.const 0
        local.set 5
        local.get 1
        i32.const 255
        i32.and
        local.set 6
        block  ;; label = @3
          loop  ;; label = @4
            local.get 2
            local.get 5
            i32.add
            i32.load8_u
            local.get 6
            i32.eq
            br_if 1 (;@3;)
            local.get 8
            local.get 5
            i32.const 1
            i32.add
            local.tee 5
            i32.ne
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 6
          local.get 8
          local.get 4
          i32.add
          local.set 5
          br 2 (;@1;)
        end
        i32.const 1
        local.set 6
        local.get 5
        local.set 5
      end
      local.get 5
      local.get 4
      i32.add
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 6
    i32.store)
  (func $_ZN4core3fmt8builders10DebugTuple5field17hc2035b36d8e0d2adE (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      local.get 0
      i32.load8_u offset=8
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.set 5
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 6
        i32.load8_u
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 1049602
        i32.const 1049606
        local.get 5
        select
        i32.const 2
        i32.const 1
        local.get 5
        select
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 1
        local.get 0
        i32.load
        local.get 2
        i32.load offset=12
        call_indirect (type 2)
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 5
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 6
        i32.load offset=24
        i32.const 1049604
        i32.const 2
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.set 6
      end
      i32.const 1
      local.set 4
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 52
      i32.add
      i32.const 1049572
      i32.store
      local.get 3
      local.get 6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 3
      local.get 3
      i32.const 23
      i32.add
      i32.store offset=16
      local.get 6
      i64.load offset=8 align=4
      local.set 7
      local.get 6
      i64.load offset=16 align=4
      local.set 8
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 8
      i64.store offset=40
      local.get 3
      local.get 7
      i64.store offset=32
      local.get 3
      local.get 6
      i64.load align=4
      i64.store offset=24
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 2)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 1049600
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 1)
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8 offset=8
    local.get 0
    local.get 0
    i32.load offset=4
    i32.const 1
    i32.add
    i32.store offset=4
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3fmt8builders10DebugTuple6finish17heeae06fe3ffe888bE (type 4) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u offset=8
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      local.set 3
      i32.const 1
      local.set 1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=9
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.tee 3
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 3
          i32.load offset=24
          i32.const 1049607
          i32.const 1
          local.get 3
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 1)
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.tee 1
        i32.load offset=24
        i32.const 1049608
        i32.const 1
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store8 offset=8
    end
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne)
  (func $_ZN4core3fmt5Write10write_char17hc49b0a0ce112405eE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 2
            i32.const 12
            i32.add
            local.set 3
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          local.get 2
          i32.const 12
          i32.add
          local.set 3
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        local.get 2
        i32.const 12
        i32.add
        local.set 3
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 3
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h636f7a801c3cd6dcE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt5Write9write_fmt17hffeae4bfb3e2780fE (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1049852
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8b996d8af01475c9E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h0293d1d5bf008d06E (type 1) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h636f7a801c3cd6dcE)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hccb20972d94c806aE (type 2) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 2
            i32.const 12
            i32.add
            local.set 3
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          local.get 2
          i32.const 12
          i32.add
          local.set 3
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        local.get 2
        i32.const 12
        i32.add
        local.set 3
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 3
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h636f7a801c3cd6dcE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h61b5701e553039f7E (type 2) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.load
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 1049852
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17h8b996d8af01475c9E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E (type 13) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        i32.const 43
        i32.const 1114112
        local.get 0
        i32.load
        local.tee 6
        i32.const 1
        i32.and
        local.tee 1
        select
        local.set 7
        local.get 1
        local.get 5
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      local.get 5
      i32.const 1
      i32.add
      local.set 8
      local.get 0
      i32.load
      local.set 6
      i32.const 45
      local.set 7
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      i32.const 0
      local.set 9
      block  ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.set 10
        local.get 2
        local.set 1
        loop  ;; label = @3
          local.get 9
          local.get 1
          i32.load8_u
          i32.const 192
          i32.and
          i32.const 128
          i32.eq
          i32.add
          local.set 9
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 8
      local.get 3
      i32.add
      local.get 9
      i32.sub
      local.set 8
    end
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h62d78e01f6fe44a9E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        local.set 1
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        i32.load
        local.tee 9
        local.get 8
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h62d78e01f6fe44a9E
        br_if 1 (;@1;)
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 1)
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          i32.const 8
          i32.and
          br_if 0 (;@3;)
          i32.const 0
          local.set 1
          local.get 9
          local.get 8
          i32.sub
          local.tee 9
          local.set 8
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                i32.const 1
                local.get 0
                i32.load8_u offset=32
                local.tee 10
                local.get 10
                i32.const 3
                i32.eq
                select
                br_table 2 (;@4;) 1 (;@5;) 0 (;@6;) 1 (;@5;) 2 (;@4;)
              end
              local.get 9
              i32.const 1
              i32.shr_u
              local.set 1
              local.get 9
              i32.const 1
              i32.add
              i32.const 1
              i32.shr_u
              local.set 8
              br 1 (;@4;)
            end
            i32.const 0
            local.set 8
            local.get 9
            local.set 1
          end
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=4
        local.set 6
        local.get 0
        i32.const 48
        i32.store offset=4
        local.get 0
        i32.load8_u offset=32
        local.set 11
        i32.const 1
        local.set 1
        local.get 0
        i32.const 1
        i32.store8 offset=32
        local.get 0
        local.get 7
        local.get 2
        local.get 3
        call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h62d78e01f6fe44a9E
        br_if 1 (;@1;)
        i32.const 0
        local.set 1
        local.get 9
        local.get 8
        i32.sub
        local.tee 10
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 1
              local.get 0
              i32.load8_u offset=32
              local.tee 9
              local.get 9
              i32.const 3
              i32.eq
              select
              br_table 2 (;@3;) 1 (;@4;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            local.get 10
            i32.const 1
            i32.shr_u
            local.set 1
            local.get 10
            i32.const 1
            i32.add
            i32.const 1
            i32.shr_u
            local.set 3
            br 1 (;@3;)
          end
          i32.const 0
          local.set 3
          local.get 10
          local.set 1
        end
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 1
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 0
            i32.load offset=4
            local.get 0
            i32.load offset=28
            i32.load offset=16
            call_indirect (type 2)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          return
        end
        local.get 0
        i32.load offset=4
        local.set 10
        i32.const 1
        local.set 1
        local.get 0
        i32.load offset=24
        local.get 4
        local.get 5
        local.get 0
        i32.load offset=28
        i32.load offset=12
        call_indirect (type 1)
        br_if 1 (;@1;)
        local.get 3
        i32.const 1
        i32.add
        local.set 9
        local.get 0
        i32.load offset=28
        local.set 3
        local.get 0
        i32.load offset=24
        local.set 2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            i32.eqz
            br_if 1 (;@3;)
            i32.const 1
            local.set 1
            local.get 2
            local.get 10
            local.get 3
            i32.load offset=16
            call_indirect (type 2)
            br_if 3 (;@1;)
            br 0 (;@4;)
          end
        end
        local.get 0
        local.get 11
        i32.store8 offset=32
        local.get 0
        local.get 6
        i32.store offset=4
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=4
      local.set 10
      i32.const 1
      local.set 1
      local.get 0
      local.get 7
      local.get 2
      local.get 3
      call $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h62d78e01f6fe44a9E
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=24
      local.get 4
      local.get 5
      local.get 0
      i32.load offset=28
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 8
      i32.const 1
      i32.add
      local.set 9
      local.get 0
      i32.load offset=28
      local.set 3
      local.get 0
      i32.load offset=24
      local.set 0
      loop  ;; label = @2
        block  ;; label = @3
          local.get 9
          i32.const -1
          i32.add
          local.tee 9
          br_if 0 (;@3;)
          i32.const 0
          return
        end
        i32.const 1
        local.set 1
        local.get 0
        local.get 10
        local.get 3
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 1)
  (func $_ZN4core3fmt9Formatter12pad_integral12write_prefix17h62d78e01f6fe44a9E (type 8) (param i32 i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 1114112
        i32.eq
        br_if 0 (;@2;)
        i32.const 1
        local.set 4
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=16
        call_indirect (type 2)
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 0
      i32.load offset=24
      local.get 2
      local.get 3
      local.get 0
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 1)
      local.set 4
    end
    local.get 4)
  (func $_ZN4core3fmt9Formatter15debug_lower_hex17h849604c366b47f68E (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u
    i32.const 16
    i32.and
    i32.const 4
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter15debug_upper_hex17ha3884c60210b1dfeE (type 4) (param i32) (result i32)
    local.get 0
    i32.load8_u
    i32.const 32
    i32.and
    i32.const 5
    i32.shr_u)
  (func $_ZN4core3fmt9Formatter11debug_tuple17h9ebea2bacbcce669E (type 11) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.load offset=24
    local.get 2
    local.get 3
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 1)
    i32.store8 offset=8
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 3
    i32.eqz
    i32.store8 offset=9
    local.get 0
    i32.const 0
    i32.store offset=4)
  (func $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h1ff8aafb0b61e6dbE (type 4) (param i32) (result i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.const 15
          local.get 0
          i32.const 68900
          i32.lt_u
          select
          local.tee 1
          local.get 1
          i32.const 8
          i32.add
          local.tee 1
          local.get 1
          i32.const 2
          i32.shl
          i32.const 1051908
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get 0
          i32.const 11
          i32.shl
          local.tee 1
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const 4
          i32.add
          local.tee 2
          local.get 2
          i32.const 2
          i32.shl
          i32.const 1051908
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get 1
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const 2
          i32.add
          local.tee 2
          local.get 2
          i32.const 2
          i32.shl
          i32.const 1051908
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get 1
          i32.gt_u
          select
          local.tee 2
          local.get 2
          i32.const 1
          i32.add
          local.tee 2
          local.get 2
          i32.const 2
          i32.shl
          i32.const 1051908
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.get 1
          i32.gt_u
          select
          local.tee 2
          i32.const 2
          i32.shl
          i32.const 1051908
          i32.add
          i32.load
          i32.const 11
          i32.shl
          local.tee 3
          local.get 1
          i32.eq
          local.get 3
          local.get 1
          i32.lt_u
          i32.add
          local.get 2
          i32.add
          local.tee 1
          i32.const 30
          i32.gt_u
          br_if 0 (;@3;)
          i32.const 689
          local.set 3
          block  ;; label = @4
            local.get 1
            i32.const 30
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 2
            i32.shl
            i32.const 1051912
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.set 3
          end
          i32.const 0
          local.set 2
          block  ;; label = @4
            local.get 1
            i32.const -1
            i32.add
            local.tee 4
            local.get 1
            i32.gt_u
            br_if 0 (;@4;)
            local.get 4
            i32.const 31
            i32.ge_u
            br_if 3 (;@1;)
            local.get 4
            i32.const 2
            i32.shl
            i32.const 1051908
            i32.add
            i32.load
            i32.const 2097151
            i32.and
            local.set 2
          end
          block  ;; label = @4
            local.get 3
            local.get 1
            i32.const 2
            i32.shl
            i32.const 1051908
            i32.add
            i32.load
            i32.const 21
            i32.shr_u
            local.tee 1
            i32.const 1
            i32.add
            i32.eq
            br_if 0 (;@4;)
            local.get 0
            local.get 2
            i32.sub
            local.set 2
            local.get 3
            i32.const -1
            i32.add
            local.set 3
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 1
              i32.const 688
              i32.gt_u
              br_if 3 (;@2;)
              local.get 0
              local.get 1
              i32.const 1052032
              i32.add
              i32.load8_u
              i32.add
              local.tee 0
              local.get 2
              i32.gt_u
              br_if 1 (;@4;)
              local.get 3
              local.get 1
              i32.const 1
              i32.add
              local.tee 1
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 3
            local.set 1
          end
          local.get 1
          i32.const 1
          i32.and
          return
        end
        local.get 1
        i32.const 31
        i32.const 1051860
        call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
        unreachable
      end
      local.get 1
      i32.const 689
      i32.const 1051876
      call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
      unreachable
    end
    local.get 4
    i32.const 31
    i32.const 1051892
    call $_ZN4core9panicking18panic_bounds_check17ha440f41d0e4eca5fE
    unreachable)
  (func $_ZN4core7unicode9printable5check17h8f72fc633c55077fE (type 14) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.const 1
        i32.shl
        i32.add
        local.set 8
        local.get 0
        i32.const 65280
        i32.and
        i32.const 8
        i32.shr_u
        local.set 9
        i32.const 0
        local.set 10
        local.get 0
        i32.const 255
        i32.and
        local.set 11
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            i32.const 2
            i32.add
            local.set 12
            local.get 10
            local.get 1
            i32.load8_u offset=1
            local.tee 2
            i32.add
            local.set 13
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 1
              local.get 9
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              local.get 9
              i32.gt_u
              br_if 3 (;@2;)
              local.get 13
              local.set 10
              local.get 12
              local.set 1
              local.get 12
              local.get 8
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
            block  ;; label = @5
              local.get 13
              local.get 10
              i32.lt_u
              br_if 0 (;@5;)
              local.get 13
              local.get 4
              i32.gt_u
              br_if 2 (;@3;)
              local.get 3
              local.get 10
              i32.add
              local.set 1
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 2
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 2
                  i32.const -1
                  i32.add
                  local.set 2
                  local.get 1
                  i32.load8_u
                  local.set 10
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  local.get 10
                  local.get 11
                  i32.ne
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 7
                br 5 (;@1;)
              end
              local.get 13
              local.set 10
              local.get 12
              local.set 1
              local.get 12
              local.get 8
              i32.ne
              br_if 1 (;@4;)
              br 3 (;@2;)
            end
          end
          local.get 10
          local.get 13
          i32.const 1050440
          call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
          unreachable
        end
        local.get 13
        local.get 4
        i32.const 1050440
        call $_ZN4core5slice20slice_index_len_fail17h6e4ffc56e8460238E
        unreachable
      end
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      local.get 6
      i32.add
      local.set 11
      local.get 0
      i32.const 65535
      i32.and
      local.set 1
      i32.const 1
      local.set 7
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          i32.const 1
          i32.add
          local.set 10
          block  ;; label = @4
            block  ;; label = @5
              local.get 5
              i32.load8_u
              local.tee 2
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              local.tee 13
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
              local.get 10
              local.set 5
              br 1 (;@4;)
            end
            local.get 10
            local.get 11
            i32.eq
            br_if 2 (;@2;)
            local.get 13
            i32.const 127
            i32.and
            i32.const 8
            i32.shl
            local.get 5
            i32.load8_u offset=1
            i32.or
            local.set 2
            local.get 5
            i32.const 2
            i32.add
            local.set 5
          end
          local.get 1
          local.get 2
          i32.sub
          local.tee 1
          i32.const 0
          i32.lt_s
          br_if 2 (;@1;)
          local.get 7
          i32.const 1
          i32.xor
          local.set 7
          local.get 5
          local.get 11
          i32.ne
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      i32.const 1049424
      i32.const 43
      i32.const 1050456
      call $_ZN4core9panicking5panic17h94c4b8c5a269b671E
      unreachable
    end
    local.get 7
    i32.const 1
    i32.and)
  (func $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h51a5a1a4e662ff80E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i64)
    i32.const 1
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      i32.const 39
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=16
      call_indirect (type 2)
      br_if 0 (;@1;)
      i32.const 2
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 0
                i32.load
                local.tee 0
                i32.const -9
                i32.add
                local.tee 4
                i32.const 30
                i32.le_u
                br_if 0 (;@6;)
                local.get 0
                i32.const 92
                i32.ne
                br_if 1 (;@5;)
                br 2 (;@4;)
              end
              i32.const 116
              local.set 5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  br_table 5 (;@2;) 1 (;@6;) 2 (;@5;) 2 (;@5;) 0 (;@7;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 3 (;@4;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 2 (;@5;) 3 (;@4;) 5 (;@2;)
                end
                i32.const 114
                local.set 5
                br 4 (;@2;)
              end
              i32.const 110
              local.set 5
              br 3 (;@2;)
            end
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  call $_ZN4core7unicode12unicode_data15grapheme_extend6lookup17h1ff8aafb0b61e6dbE
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.const 65536
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 131072
                        i32.lt_u
                        br_if 1 (;@9;)
                        local.get 0
                        i32.const -918000
                        i32.add
                        i32.const 196112
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -201547
                        i32.add
                        i32.const 716213
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -195102
                        i32.add
                        i32.const 1506
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -191457
                        i32.add
                        i32.const 3103
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -183970
                        i32.add
                        i32.const 14
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const 2097150
                        i32.and
                        i32.const 178206
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -173790
                        i32.add
                        i32.const 34
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 0
                        i32.const -177973
                        i32.add
                        i32.const 10
                        i32.gt_u
                        br_if 5 (;@5;)
                        br 2 (;@8;)
                      end
                      local.get 0
                      i32.const 1050472
                      i32.const 41
                      i32.const 1050554
                      i32.const 290
                      i32.const 1050844
                      i32.const 309
                      call $_ZN4core7unicode9printable5check17h8f72fc633c55077fE
                      i32.eqz
                      br_if 1 (;@8;)
                      br 4 (;@5;)
                    end
                    local.get 0
                    i32.const 1051153
                    i32.const 38
                    i32.const 1051229
                    i32.const 175
                    i32.const 1051404
                    i32.const 419
                    call $_ZN4core7unicode9printable5check17h8f72fc633c55077fE
                    br_if 3 (;@5;)
                  end
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.clz
                  i32.const 2
                  i32.shr_u
                  i32.const 7
                  i32.xor
                  i64.extend_i32_u
                  i64.const 21474836480
                  i64.or
                  local.set 6
                  br 1 (;@6;)
                end
                local.get 0
                i32.const 1
                i32.or
                i32.clz
                i32.const 2
                i32.shr_u
                i32.const 7
                i32.xor
                i64.extend_i32_u
                i64.const 21474836480
                i64.or
                local.set 6
              end
              i32.const 3
              local.set 3
              br 2 (;@3;)
            end
            i32.const 1
            local.set 3
            br 1 (;@3;)
          end
        end
        local.get 0
        local.set 5
      end
      loop  ;; label = @2
        local.get 3
        local.set 4
        i32.const 92
        local.set 0
        i32.const 1
        local.set 2
        i32.const 1
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    br_table 2 (;@6;) 1 (;@7;) 5 (;@3;) 0 (;@8;) 2 (;@6;)
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 6
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          br_table 5 (;@6;) 3 (;@8;) 2 (;@9;) 1 (;@10;) 0 (;@11;) 6 (;@5;) 5 (;@6;)
                        end
                        local.get 6
                        i64.const -1095216660481
                        i64.and
                        i64.const 12884901888
                        i64.or
                        local.set 6
                        i32.const 117
                        local.set 0
                        br 6 (;@4;)
                      end
                      local.get 6
                      i64.const -1095216660481
                      i64.and
                      i64.const 8589934592
                      i64.or
                      local.set 6
                      i32.const 123
                      local.set 0
                      br 5 (;@4;)
                    end
                    local.get 5
                    local.get 6
                    i32.wrap_i64
                    local.tee 4
                    i32.const 2
                    i32.shl
                    i32.const 28
                    i32.and
                    i32.shr_u
                    i32.const 15
                    i32.and
                    local.tee 3
                    i32.const 48
                    i32.or
                    local.get 3
                    i32.const 87
                    i32.add
                    local.get 3
                    i32.const 10
                    i32.lt_u
                    select
                    local.set 0
                    block  ;; label = @9
                      local.get 4
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 6
                      i64.const -1
                      i64.add
                      i64.const 4294967295
                      i64.and
                      local.get 6
                      i64.const -4294967296
                      i64.and
                      i64.or
                      local.set 6
                      br 5 (;@4;)
                    end
                    local.get 6
                    i64.const -1095216660481
                    i64.and
                    i64.const 4294967296
                    i64.or
                    local.set 6
                    br 4 (;@4;)
                  end
                  local.get 6
                  i64.const -1095216660481
                  i64.and
                  local.set 6
                  i32.const 125
                  local.set 0
                  br 3 (;@4;)
                end
                i32.const 0
                local.set 3
                local.get 5
                local.set 0
                br 3 (;@3;)
              end
              local.get 1
              i32.load offset=24
              i32.const 39
              local.get 1
              i32.load offset=28
              i32.load offset=16
              call_indirect (type 2)
              return
            end
            local.get 6
            i64.const -1095216660481
            i64.and
            i64.const 17179869184
            i64.or
            local.set 6
          end
          i32.const 3
          local.set 3
        end
        local.get 1
        i32.load offset=24
        local.get 0
        local.get 1
        i32.load offset=28
        i32.load offset=16
        call_indirect (type 2)
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 2)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hf65aa2a60facf14eE (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 87
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1049632
      call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1049648
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h59d71fee3c1b5474E (type 2) (param i32 i32) (result i32)
    local.get 0
    i64.load8_u
    i32.const 1
    local.get 1
    call $_ZN4core3fmt3num3imp7fmt_u6417h5081cd6222065ff2E)
  (func $_ZN4core3fmt3num3imp7fmt_u6417h5081cd6222065ff2E (type 15) (param i64 i32 i32) (result i32)
    (local i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    i32.const 39
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i64.const 10000
        i64.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 5
        br 1 (;@1;)
      end
      i32.const 39
      local.set 4
      loop  ;; label = @2
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 0
        local.get 0
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.const 1049650
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.const 1049650
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        local.get 4
        i32.const -4
        i32.add
        local.set 4
        local.get 0
        i64.const 99999999
        i64.gt_u
        local.set 6
        local.get 5
        local.set 0
        local.get 6
        br_if 0 (;@2;)
      end
    end
    block  ;; label = @1
      local.get 5
      i32.wrap_i64
      local.tee 6
      i32.const 99
      i32.le_s
      br_if 0 (;@1;)
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -2
      i32.add
      local.tee 4
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 6
      local.get 6
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 6
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.const 1049650
      i32.add
      i32.load16_u align=1
      i32.store16 align=1
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 6
        i32.const 10
        i32.lt_s
        br_if 0 (;@2;)
        local.get 3
        i32.const 9
        i32.add
        local.get 4
        i32.const -2
        i32.add
        local.tee 4
        i32.add
        local.get 6
        i32.const 1
        i32.shl
        i32.const 1049650
        i32.add
        i32.load16_u align=1
        i32.store16 align=1
        br 1 (;@1;)
      end
      local.get 3
      i32.const 9
      i32.add
      local.get 4
      i32.const -1
      i32.add
      local.tee 4
      i32.add
      local.get 6
      i32.const 48
      i32.add
      i32.store8
    end
    local.get 2
    local.get 1
    i32.const 1049412
    i32.const 0
    local.get 3
    i32.const 9
    i32.add
    local.get 4
    i32.add
    i32.const 39
    local.get 4
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E
    local.set 4
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17he0978d4c9e80e139E (type 2) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 128
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop  ;; label = @1
      local.get 2
      local.get 0
      i32.add
      i32.const 127
      i32.add
      local.get 3
      i32.const 15
      i32.and
      local.tee 4
      i32.const 48
      i32.or
      local.get 4
      i32.const 55
      i32.add
      local.get 4
      i32.const 10
      i32.lt_u
      select
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    block  ;; label = @1
      local.get 0
      i32.const 128
      i32.add
      local.tee 3
      i32.const 129
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 128
      i32.const 1049632
      call $_ZN4core5slice22slice_index_order_fail17h009b7414b170d68dE
      unreachable
    end
    local.get 1
    i32.const 1
    i32.const 1049648
    i32.const 2
    local.get 2
    local.get 0
    i32.add
    i32.const 128
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_ZN4core3fmt9Formatter12pad_integral17hfd6532b3a41ee584E
    local.set 0
    local.get 2
    i32.const 128
    i32.add
    global.set 0
    local.get 0)
  (func $memcpy (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memset (type 1) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.set 3
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 0)
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
  (table (;0;) 32 32 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1053208))
  (global (;2;) i32 (i32.const 1053208))
  (export "memory" (memory 0))
  (export "svm_alloc" (func $svm_alloc))
  (export "initialize" (func $initialize))
  (export "add" (func $add))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr13drop_in_place17h12fe707ca2db124aE $_ZN74_$LT$svm_abi_decoder..decoder..DecodeError$u20$as$u20$core..fmt..Debug$GT$3fmt17h9388a80901b01355E $_ZN4core3ptr13drop_in_place17hca03f35676b9831cE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h71c1f598a49bac44E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf0958354fd7e5680E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h019fb32cc03cc4ccE $_ZN3std5alloc24default_alloc_error_hook17hd9d9385c91b96c15E $_ZN4core3ptr13drop_in_place17h020c7e7e67a167e6E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h196f664680526f53E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hc6e921a65ea9542bE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h633f9c548303c0d8E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h911021edd741cc49E $_ZN4core3ptr13drop_in_place17hfbc67c1b337a35b6E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17hfc3314771b196f49E $_ZN90_$LT$std..panicking..begin_panic_handler..PanicPayload$u20$as$u20$core..panic..BoxMeUp$GT$3get17ha36eed8111393a47E $_ZN4core3ptr13drop_in_place17h2af87d0bdfae1a93E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h08d06f832c14283aE $_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hd7239ff7b4d279a9E $_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h67ec6315566da26cE $_ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h8c8a97b866010196E $_ZN41_$LT$char$u20$as$u20$core..fmt..Debug$GT$3fmt17h51a5a1a4e662ff80E $_ZN4core3ops8function6FnOnce9call_once17hbb1f81e74ea5afcbE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h4e3d92a201093296E $_ZN4core3ptr13drop_in_place17h0200b96f86cedf64E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h46f25525116e9d7cE $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h636f7a801c3cd6dcE $_ZN4core3fmt5Write10write_char17hc49b0a0ce112405eE $_ZN4core3fmt5Write9write_fmt17hffeae4bfb3e2780fE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h0293d1d5bf008d06E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17hccb20972d94c806aE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h61b5701e553039f7E)
  (data (;0;) (i32.const 1048576) "called `Result::unwrap()` on an `Err` value\00\01\00\00\00\02\00\00\00\01\00\00\00\02\00\00\00src/lib.rs\00\00<\00\10\00\0a\00\00\00(\00\00\00:\00\00\00<\00\10\00\0a\00\00\006\00\00\00;\00\00\00internal error: entered unreachable code/Users/yaronwittenstein/work/sm/svm/crates/svm-abi/decoder/src/decoder.rs\00\00\00\90\00\10\00I\00\00\00p\00\00\00\12\00\00\00\90\00\10\00I\00\00\00\8a\00\00\00\12\00\00\00\90\00\10\00I\00\00\00\a7\00\00\00\12\00\00\00\90\00\10\00I\00\00\00\bb\00\00\00\12\00\00\00\90\00\10\00I\00\00\00\d3\00\00\00\12\00\00\00\90\00\10\00I\00\00\00\ef\00\00\00\12\00\00\00\90\00\10\00I\00\00\00d\01\00\00\12\00\00\00InvalidTypeKind\00\03\00\00\00\04\00\00\00\04\00\00\00\04\00\00\00MissingTypeKindNotEnoughBytesValue\00\00\03\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00Type\03\00\00\00\04\00\00\00\04\00\00\00\06\00\00\00\01\00\00\00\02\00\00\00\01\00\00\00\02\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00/Users/yaronwittenstein/work/sm/svm/crates/svm-sdk/src/value.rsinternal error: entered unreachable code\00\e4\01\10\00?\00\00\00\0e\01\00\00\01\00\00\00\08\00\00\00\04\00\00\00\04\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\0c\00\00\00called `Option::unwrap()` on a `None` valuesrc/libstd/panicking.rs\00\00\af\02\10\00\17\00\00\00\b3\01\00\00\1f\00\00\00\af\02\10\00\17\00\00\00\b4\01\00\00\1e\00\00\00\0d\00\00\00\10\00\00\00\04\00\00\00\0e\00\00\00\0f\00\00\00\10\00\00\00\0c\00\00\00\04\00\00\00\11\00\00\00src/liballoc/raw_vec.rscapacity overflow\0c\03\10\00\17\00\00\00\17\02\00\00\05\00\00\00`..\00E\03\10\00\02\00\00\00called `Option::unwrap()` on a `None` value: \00\00\00D\03\10\00\00\00\00\00{\03\10\00\02\00\00\00\18\00\00\00\00\00\00\00\01\00\00\00\19\00\00\00index out of bounds: the len is  but the index is \00\00\a0\03\10\00 \00\00\00\c0\03\10\00\12\00\00\00\18\00\00\00\0c\00\00\00\04\00\00\00\1a\00\00\00\1b\00\00\00\1c\00\00\00    ,\0a, (\0a(,)src/libcore/fmt/num.rs\00\09\04\10\00\16\00\00\00T\00\00\00\14\00\00\000x00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\00\00\18\00\00\00\04\00\00\00\04\00\00\00\1d\00\00\00\1e\00\00\00\1f\00\00\00src/libcore/fmt/mod.rs\00\00\14\05\10\00\16\00\00\00S\04\00\00\11\00\00\00\14\05\10\00\16\00\00\00]\04\00\00$\00\00\00src/libcore/slice/memchr.rs\00L\05\10\00\1b\00\00\00R\00\00\00\05\00\00\00index  out of range for slice of length x\05\10\00\06\00\00\00~\05\10\00\22\00\00\00slice index starts at  but ends at \00\b0\05\10\00\16\00\00\00\c6\05\10\00\0d\00\00\00src/libcore/str/pattern.rs\00\00\e4\05\10\00\1a\00\00\00\b0\01\00\00&\00\00\00src/libcore/str/mod.rs\00\00\10\06\10\00\16\00\00\00\e2\07\00\00/\00\00\00\10\06\10\00\16\00\00\00/\08\00\00/\00\00\00[...]byte index  is out of bounds of `\00\00M\06\10\00\0b\00\00\00X\06\10\00\16\00\00\00D\03\10\00\01\00\00\00begin <= end ( <= ) when slicing `\00\00\88\06\10\00\0e\00\00\00\96\06\10\00\04\00\00\00\9a\06\10\00\10\00\00\00D\03\10\00\01\00\00\00 is not a char boundary; it is inside  (bytes ) of `M\06\10\00\0b\00\00\00\cc\06\10\00&\00\00\00\f2\06\10\00\08\00\00\00\fa\06\10\00\06\00\00\00D\03\10\00\01\00\00\00src/libcore/unicode/printable.rs(\07\10\00 \00\00\00\0a\00\00\00\1c\00\00\00(\07\10\00 \00\00\00\1a\00\00\006\00\00\00\00\01\03\05\05\06\06\03\07\06\08\08\09\11\0a\1c\0b\19\0c\14\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\05\18\02\19\03\1a\07\1c\02\1d\01\1f\16 \03+\03,\02-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\04\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\1d\dd\0e\0fKL\fb\fc./?\5c]_\b5\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11)EIWde\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80\0dmq\de\df\0e\0f\1fno\1c\1d_}~\ae\af\bb\bc\fa\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96/_&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\c0\c1\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91\fe\ffSgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab5(\0b\80\e0\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05:\03\11\07\06\05\10\07W\07\02\07\15\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\15\0b\17\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06!?L\04-\03t\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03 \10!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\b3-t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\ee\0d\03\84\8d\037\09\81\5c\14\80\b8\08\80\cb*8\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a5\11\81m\10x(*\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08$\01j\03k\02\bc\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e8\02\ee \f0\04\f8\02\f9\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\ee\efZb\9a\9b'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afy\ccno\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0b\01\80\90\817\09\16\0a\08\80\989\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*V\1c\14\17\09N\04\1e\0fC\0e\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\139\07\0a6,\04\10\80\c0<dS\0cH\09\0aFE\1bH\08S\1d9\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8a\bc\84/\8f\d1\82G\a1\b9\829\07*\04\02`&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\f7\81\1f1\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\93`\80\f6\0as\08n\17F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\85P+\80\d5-\03\1a\04\02\81p:\05\01\85\00\80\d7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\08\0d\03\0d\03t\0cY\07\0c\14\0c\048\08\0a\06(\08\22N\81T\0c\15\03\03\05\07\09\19\07\07\09\03\0d\07)\80\cb%\0a\84\06src/libcore/unicode/unicode_data.rs\00\00\af\0c\10\00#\00\00\00K\00\00\00(\00\00\00\af\0c\10\00#\00\00\00W\00\00\00\16\00\00\00\af\0c\10\00#\00\00\00R\00\00\00>\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17\a0\1e\0c \e0\1e\ef, +*0\a0+o\a6`,\02\a8\e0,\1e\fb\e0-\00\fe\a05\9e\ff\e05\fd\01a6\01\0a\a16$\0da7\ab\0e\e18/\18!90\1caF\f3\1e\a1J\f0jaNOo\a1N\9d\bc!Oe\d1\e1O\00\da!P\00\e0\e1Q0\e1aS\ec\e2\a1T\d0\e8\e1T \00.U\f0\01\bfU\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03w\0f\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\019\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\03\1d\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03w\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\11?\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6:\01\05\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b\b0\016\0f8\031\04\02\02E\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01?\04Q\01\00\02\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\04\00\07m\07\00`\80\f0\00"))
