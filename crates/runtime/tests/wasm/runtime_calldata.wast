(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32)))
  (type (;8;) (func (param i32) (result i64)))
  (import "svm" "svm_set_returndata" (func $_ZN16svm_sdk_host_ffi3ext18svm_set_returndata17had45b1b4413e0fc5E (type 0)))
  (import "svm" "svm_calldata_offset" (func $_ZN16svm_sdk_host_ffi3ext19svm_calldata_offset17h68725aafcb41b8baE (type 1)))
  (import "svm" "svm_calldata_len" (func $_ZN16svm_sdk_host_ffi3ext16svm_calldata_len17h7de974f6156de6c3E (type 1)))
  (import "svm" "svm_static_alloc" (func $_ZN13svm_sdk_alloc16svm_static_alloc17h8ba4e9d4afc37a5dE (type 2)))
  (import "svm" "svm_store160" (func $_ZN19svm_sdk_storage_ffi3ext12svm_store16017h91b6d8c546b0ce19E (type 0)))
  (import "svm" "svm_load160" (func $_ZN19svm_sdk_storage_ffi3ext11svm_load16017hb38b830f47d4179cE (type 0)))
  (func $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$14set_returndata17h757f3562e9537b0cE (type 0) (param i32 i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1048624
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048624
    end
    local.get 0
    local.get 1
    call $_ZN16svm_sdk_host_ffi3ext18svm_set_returndata17had45b1b4413e0fc5E)
  (func $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$8calldata17hf4745b46f4b83c99E (type 3) (param i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1048624
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048624
    end
    call $_ZN16svm_sdk_host_ffi3ext19svm_calldata_offset17h68725aafcb41b8baE
    local.set 1
    local.get 0
    call $_ZN16svm_sdk_host_ffi3ext16svm_calldata_len17h7de974f6156de6c3E
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $svm_verify (type 1) (result i32)
    i32.const 0)
  (func $svm_alloc (type 2) (param i32) (result i32)
    local.get 0
    call $_ZN13svm_sdk_alloc16svm_static_alloc17h8ba4e9d4afc37a5dE)
  (func $_ZN29svm_runtime_examples_calldata7Storage8set_addr17hd9299a64a840bb37E (type 3) (param i32)
    local.get 0
    i32.const 0
    call $_ZN19svm_sdk_storage_ffi3ext12svm_store16017h91b6d8c546b0ce19E)
  (func $initialize (type 4)
    (local i32 i64 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$8calldata17hf4745b46f4b83c99E
    local.get 0
    i64.load
    local.set 1
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    local.get 1
    i64.store offset=8
    local.get 0
    i32.const 8
    i32.add
    call $_ZN15svm_abi_decoder8calldata8CallData6next_117hafb386aaeea46282E
    local.set 2
    local.get 0
    i32.const 24
    i32.add
    local.get 0
    i32.const 8
    i32.add
    call $_ZN15svm_abi_decoder8calldata8CallData4next17h3b938f6b031f5ab5E
    block  ;; label = @1
      local.get 0
      i32.load offset=24
      local.tee 3
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      br_if 0 (;@1;)
      local.get 0
      i32.const 32
      i32.add
      i32.load8_u
      i32.const 255
      i32.and
      i32.const 2
      i32.ne
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=33
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        call $_ZN29svm_runtime_examples_calldata7Storage8set_addr17hd9299a64a840bb37E
      end
      local.get 0
      i32.const 24
      i32.add
      i32.const 1
      call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17ha5818bdb49307df4E
      local.get 0
      i32.load offset=24
      local.tee 3
      local.get 0
      i32.load offset=28
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=32
      local.tee 2
      local.get 3
      i32.add
      i32.const 16
      i32.store8
      local.get 2
      local.get 3
      i32.const 1
      i32.add
      call $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$14set_returndata17h757f3562e9537b0cE
      local.get 0
      i32.const 48
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN15svm_abi_decoder8calldata8CallData6next_117hafb386aaeea46282E (type 2) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    call $_ZN15svm_abi_decoder8calldata8CallData4next17h3b938f6b031f5ab5E
    block  ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 0
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      i32.const 16
      i32.add
      i32.load8_u
      i32.const 255
      i32.and
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 20
      i32.add
      i32.load
      local.set 0
      local.get 1
      i32.const 32
      i32.add
      global.set 0
      local.get 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN15svm_abi_decoder8calldata8CallData4next17h3b938f6b031f5ab5E (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 96
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
                                        local.get 1
                                        i32.const 8
                                        i32.add
                                        i32.load
                                        local.tee 3
                                        local.get 1
                                        i32.const 4
                                        i32.add
                                        i32.load
                                        local.tee 4
                                        i32.ge_u
                                        br_if 0 (;@18;)
                                        local.get 1
                                        i32.load
                                        local.tee 5
                                        local.get 4
                                        local.get 3
                                        call $_ZN15svm_abi_decoder7decoder7Decoder9peek_kind17h3270d3def6a08f5aE
                                        local.tee 6
                                        i32.const 1
                                        i32.and
                                        br_if 2 (;@16;)
                                        local.get 6
                                        i32.const 65280
                                        i32.and
                                        i32.const 3328
                                        i32.ne
                                        br_if 1 (;@17;)
                                        local.get 5
                                        local.get 4
                                        local.get 3
                                        call $_ZN15svm_abi_decoder7decoder7Decoder9peek_kind17h3270d3def6a08f5aE
                                        local.tee 3
                                        i32.const 1
                                        i32.and
                                        br_if 3 (;@15;)
                                        local.get 3
                                        i32.const 65280
                                        i32.and
                                        i32.const 3328
                                        i32.ne
                                        br_if 17 (;@1;)
                                        local.get 1
                                        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                        local.tee 3
                                        i32.const 16776960
                                        i32.and
                                        i32.const 8
                                        i32.shr_u
                                        local.set 4
                                        block  ;; label = @19
                                          local.get 3
                                          i32.const 1
                                          i32.and
                                          br_if 0 (;@19;)
                                          i32.const 0
                                          local.set 3
                                          local.get 4
                                          i32.const 255
                                          i32.and
                                          local.tee 6
                                          i32.const 24
                                          i32.mul
                                          call $_ZN13svm_sdk_alloc16svm_static_alloc17h8ba4e9d4afc37a5dE
                                          local.set 4
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  local.get 6
                                                  i32.const -6
                                                  i32.add
                                                  br_table 17 (;@6;) 1 (;@22;) 0 (;@23;)
                                                end
                                                i32.const 0
                                                local.set 3
                                                i32.const 0
                                                local.set 5
                                                i32.const 0
                                                local.set 7
                                                i32.const 0
                                                local.set 8
                                                i32.const 0
                                                local.set 9
                                                i32.const 0
                                                local.set 10
                                                i32.const 0
                                                local.set 11
                                                i32.const 0
                                                local.set 12
                                                i32.const 0
                                                local.set 13
                                                block  ;; label = @23
                                                  local.get 6
                                                  i32.const -22
                                                  i32.add
                                                  br_table 16 (;@7;) 3 (;@20;) 0 (;@23;)
                                                end
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              local.get 6
                                                              i32.const -38
                                                              i32.add
                                                              br_table 1 (;@28;) 8 (;@21;) 0 (;@29;)
                                                            end
                                                            local.get 6
                                                            i32.const 54
                                                            i32.eq
                                                            br_if 1 (;@27;)
                                                            local.get 6
                                                            i32.const 70
                                                            i32.eq
                                                            br_if 2 (;@26;)
                                                            local.get 6
                                                            i32.const 86
                                                            i32.eq
                                                            br_if 3 (;@25;)
                                                            local.get 6
                                                            i32.const 102
                                                            i32.eq
                                                            br_if 4 (;@24;)
                                                            local.get 6
                                                            i32.const 118
                                                            i32.eq
                                                            br_if 5 (;@23;)
                                                            br 27 (;@1;)
                                                          end
                                                          i32.const 1
                                                          local.set 3
                                                          i32.const 0
                                                          local.set 5
                                                          br 13 (;@14;)
                                                        end
                                                        i32.const 1
                                                        local.set 3
                                                        i32.const 1
                                                        local.set 5
                                                        br 12 (;@14;)
                                                      end
                                                      i32.const 1
                                                      local.set 3
                                                      i32.const 1
                                                      local.set 5
                                                      i32.const 1
                                                      local.set 7
                                                      br 12 (;@13;)
                                                    end
                                                    i32.const 1
                                                    local.set 3
                                                    i32.const 1
                                                    local.set 5
                                                    i32.const 1
                                                    local.set 7
                                                    i32.const 1
                                                    local.set 8
                                                    br 12 (;@12;)
                                                  end
                                                  i32.const 1
                                                  local.set 3
                                                  i32.const 1
                                                  local.set 5
                                                  i32.const 1
                                                  local.set 7
                                                  i32.const 1
                                                  local.set 8
                                                  i32.const 1
                                                  local.set 9
                                                  br 12 (;@11;)
                                                end
                                                i32.const 1
                                                local.set 3
                                                i32.const 1
                                                local.set 5
                                                i32.const 1
                                                local.set 7
                                                i32.const 1
                                                local.set 8
                                                i32.const 1
                                                local.set 9
                                                i32.const 1
                                                local.set 10
                                                br 12 (;@10;)
                                              end
                                              i32.const 1
                                              local.set 3
                                              i32.const 1
                                              local.set 5
                                              i32.const 1
                                              local.set 7
                                              i32.const 1
                                              local.set 8
                                              i32.const 1
                                              local.set 9
                                              i32.const 1
                                              local.set 10
                                              i32.const 1
                                              local.set 11
                                              br 12 (;@9;)
                                            end
                                            i32.const 1
                                            local.set 3
                                            i32.const 1
                                            local.set 5
                                            i32.const 1
                                            local.set 7
                                            i32.const 1
                                            local.set 8
                                            i32.const 1
                                            local.set 9
                                            i32.const 1
                                            local.set 10
                                            i32.const 1
                                            local.set 11
                                            i32.const 1
                                            local.set 12
                                            i32.const 1
                                            local.set 13
                                            br 13 (;@7;)
                                          end
                                          i32.const 1
                                          local.set 3
                                          i32.const 1
                                          local.set 5
                                          i32.const 1
                                          local.set 7
                                          i32.const 1
                                          local.set 8
                                          i32.const 1
                                          local.set 9
                                          i32.const 1
                                          local.set 10
                                          i32.const 1
                                          local.set 11
                                          i32.const 1
                                          local.set 12
                                          br 11 (;@8;)
                                        end
                                        local.get 3
                                        i32.const 16711680
                                        i32.and
                                        i32.const 16
                                        i32.shr_u
                                        local.set 1
                                        br 16 (;@2;)
                                      end
                                      local.get 0
                                      i32.const 2
                                      i32.store
                                      br 13 (;@4;)
                                    end
                                    local.get 2
                                    i32.const 64
                                    i32.add
                                    local.get 1
                                    call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                                    local.get 2
                                    i32.load8_u offset=64
                                    i32.const 1
                                    i32.eq
                                    br_if 15 (;@1;)
                                    br 11 (;@5;)
                                  end
                                  local.get 2
                                  i32.const 1
                                  i32.store8 offset=64
                                  local.get 2
                                  local.get 6
                                  i32.const 16
                                  i32.shr_u
                                  i32.store8 offset=66
                                  local.get 2
                                  local.get 6
                                  i32.const 8
                                  i32.shr_u
                                  i32.store8 offset=65
                                  br 14 (;@1;)
                                end
                                local.get 2
                                i32.const 1
                                i32.store8 offset=64
                                local.get 2
                                local.get 3
                                i32.const 16
                                i32.shr_u
                                i32.store8 offset=66
                                local.get 2
                                local.get 3
                                i32.const 8
                                i32.shr_u
                                i32.store8 offset=65
                                br 13 (;@1;)
                              end
                              i32.const 0
                              local.set 7
                            end
                            i32.const 0
                            local.set 8
                          end
                          i32.const 0
                          local.set 9
                        end
                        i32.const 0
                        local.set 10
                      end
                      i32.const 0
                      local.set 11
                    end
                    i32.const 0
                    local.set 12
                  end
                  i32.const 0
                  local.set 13
                end
                local.get 2
                i32.const 64
                i32.add
                local.get 1
                call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                local.get 2
                i32.load8_u offset=64
                i32.const 1
                i32.eq
                br_if 3 (;@3;)
                local.get 2
                i32.const 8
                i32.add
                i32.const 8
                i32.add
                local.tee 14
                local.get 2
                i32.const 64
                i32.add
                i32.const 16
                i32.add
                i64.load
                i64.store
                local.get 2
                i32.const 8
                i32.add
                i32.const 16
                i32.add
                local.tee 15
                local.get 2
                i32.const 88
                i32.add
                i64.load
                i64.store
                local.get 2
                local.get 2
                i32.const 64
                i32.add
                i32.const 8
                i32.add
                i64.load
                i64.store offset=8
                local.get 6
                i32.eqz
                br_if 5 (;@1;)
                local.get 4
                local.get 2
                i64.load offset=8
                i64.store
                local.get 4
                i32.const 16
                i32.add
                local.get 15
                i64.load
                i64.store
                local.get 4
                i32.const 8
                i32.add
                local.get 14
                i64.load
                i64.store
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 3
                      br_if 0 (;@9;)
                      i32.const 1
                      local.set 3
                      local.get 5
                      i32.eqz
                      br_if 2 (;@7;)
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 1
                    call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                    local.get 2
                    i32.load8_u offset=64
                    i32.const 1
                    i32.eq
                    br_if 5 (;@3;)
                    local.get 2
                    i32.const 8
                    i32.add
                    i32.const 8
                    i32.add
                    local.tee 3
                    local.get 2
                    i32.const 64
                    i32.add
                    i32.const 16
                    i32.add
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 8
                    i32.add
                    i32.const 16
                    i32.add
                    local.tee 14
                    local.get 2
                    i32.const 88
                    i32.add
                    i64.load
                    i64.store
                    local.get 2
                    local.get 2
                    i32.const 64
                    i32.add
                    i32.const 8
                    i32.add
                    i64.load
                    i64.store offset=8
                    local.get 6
                    i32.const 1
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 2
                    i64.load offset=8
                    i64.store offset=24
                    local.get 4
                    i32.const 40
                    i32.add
                    local.get 14
                    i64.load
                    i64.store
                    local.get 4
                    i32.const 32
                    i32.add
                    local.get 3
                    i64.load
                    i64.store
                    i32.const 2
                    local.set 3
                    local.get 5
                    i32.eqz
                    br_if 1 (;@7;)
                  end
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 14
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 15
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 15
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 14
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 7
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 14
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 14
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 8
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 8
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 9
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 8
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 8
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 10
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 8
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 8
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 11
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 8
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 8
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                block  ;; label = @7
                  local.get 12
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                  local.get 2
                  i32.load8_u offset=64
                  i32.const 1
                  i32.eq
                  br_if 4 (;@3;)
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 7
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 8
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 8
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 24
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store offset=8
                  local.get 3
                  local.get 6
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 24
                  i32.mul
                  i32.add
                  local.tee 5
                  local.get 2
                  i64.load offset=8
                  i64.store
                  local.get 5
                  i32.const 16
                  i32.add
                  local.get 8
                  i64.load
                  i64.store
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 7
                  i64.load
                  i64.store
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                end
                local.get 13
                i32.eqz
                br_if 0 (;@6;)
                local.get 2
                i32.const 64
                i32.add
                local.get 1
                call $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE
                local.get 2
                i32.load8_u offset=64
                i32.const 1
                i32.eq
                br_if 3 (;@3;)
                local.get 2
                i32.const 8
                i32.add
                i32.const 8
                i32.add
                local.tee 5
                local.get 2
                i32.const 64
                i32.add
                i32.const 16
                i32.add
                i64.load
                i64.store
                local.get 2
                i32.const 8
                i32.add
                i32.const 16
                i32.add
                local.tee 7
                local.get 2
                i32.const 64
                i32.add
                i32.const 24
                i32.add
                i64.load
                i64.store
                local.get 2
                local.get 2
                i32.const 64
                i32.add
                i32.const 8
                i32.add
                i64.load
                i64.store offset=8
                local.get 3
                local.get 6
                i32.ge_u
                br_if 5 (;@1;)
                local.get 4
                local.get 3
                i32.const 24
                i32.mul
                i32.add
                local.tee 1
                local.get 2
                i64.load offset=8
                i64.store
                local.get 1
                i32.const 16
                i32.add
                local.get 7
                i64.load
                i64.store
                local.get 1
                i32.const 8
                i32.add
                local.get 5
                i64.load
                i64.store
                local.get 3
                i32.const 1
                i32.add
                local.set 3
              end
              local.get 2
              i32.const 84
              i32.add
              local.get 4
              i32.store
              local.get 2
              i32.const 80
              i32.add
              local.get 6
              i32.store
              local.get 2
              i32.const 76
              i32.add
              local.get 3
              i32.store
              local.get 2
              i32.const 1
              i32.store offset=72
            end
            local.get 0
            local.get 2
            i64.load offset=72
            i64.store
            local.get 0
            i32.const 16
            i32.add
            local.get 2
            i32.const 88
            i32.add
            i64.load
            i64.store
            local.get 0
            i32.const 8
            i32.add
            local.get 2
            i32.const 64
            i32.add
            i32.const 16
            i32.add
            i64.load
            i64.store
          end
          local.get 2
          i32.const 96
          i32.add
          global.set 0
          return
        end
        local.get 2
        i32.load8_u offset=66
        local.set 1
        local.get 2
        i32.load8_u offset=65
        local.set 4
      end
      local.get 2
      local.get 1
      i32.store8 offset=66
      local.get 2
      local.get 4
      i32.store8 offset=65
      local.get 2
      i32.const 1
      i32.store8 offset=64
    end
    unreachable
    unreachable)
  (func $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17ha5818bdb49307df4E (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN13svm_sdk_alloc16svm_static_alloc17h8ba4e9d4afc37a5dE
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store)
  (func $store_addr (type 4)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 8
    i32.add
    call $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$8calldata17hf4745b46f4b83c99E
    local.get 0
    i64.load offset=8
    local.set 1
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    local.get 1
    i64.store offset=16
    local.get 0
    i32.const 16
    i32.add
    call $_ZN15svm_abi_decoder8calldata8CallData6next_117hafb386aaeea46282E
    call $_ZN29svm_runtime_examples_calldata7Storage8set_addr17hd9299a64a840bb37E
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func $load_addr (type 4)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    i32.const 0
    i32.const 20
    call $_ZN13svm_sdk_alloc16svm_static_alloc17h8ba4e9d4afc37a5dE
    local.tee 1
    call $_ZN19svm_sdk_storage_ffi3ext11svm_load16017hb38b830f47d4179cE
    local.get 0
    i32.const 21
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17ha5818bdb49307df4E
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 2
      i32.add
      i32.const 64
      i32.store8
      local.get 0
      local.get 2
      i32.const 1
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u
      i32.store8
      local.get 0
      local.get 2
      i32.const 2
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=1
      i32.store8
      local.get 0
      local.get 2
      i32.const 3
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=2
      i32.store8
      local.get 0
      local.get 2
      i32.const 4
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=3
      i32.store8
      local.get 0
      local.get 2
      i32.const 5
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=4
      i32.store8
      local.get 0
      local.get 2
      i32.const 6
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=5
      i32.store8
      local.get 0
      local.get 2
      i32.const 7
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=6
      i32.store8
      local.get 0
      local.get 2
      i32.const 8
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=7
      i32.store8
      local.get 0
      local.get 2
      i32.const 9
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=8
      i32.store8
      local.get 0
      local.get 2
      i32.const 10
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=9
      i32.store8
      local.get 0
      local.get 2
      i32.const 11
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=10
      i32.store8
      local.get 0
      local.get 2
      i32.const 12
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=11
      i32.store8
      local.get 0
      local.get 2
      i32.const 13
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=12
      i32.store8
      local.get 0
      local.get 2
      i32.const 14
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=13
      i32.store8
      local.get 0
      local.get 2
      i32.const 15
      i32.add
      local.tee 4
      i32.store
      local.get 4
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.tee 3
      local.get 4
      i32.add
      local.get 1
      i32.load8_u offset=14
      i32.store8
      local.get 0
      local.get 2
      i32.const 16
      i32.add
      local.tee 5
      i32.store
      local.get 5
      local.get 0
      i32.load offset=4
      local.tee 4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.add
      local.get 1
      i32.load8_u offset=15
      i32.store8
      local.get 0
      local.get 2
      i32.const 17
      i32.add
      local.tee 5
      i32.store
      local.get 5
      local.get 4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.add
      local.get 1
      i32.load8_u offset=16
      i32.store8
      local.get 0
      local.get 2
      i32.const 18
      i32.add
      local.tee 5
      i32.store
      local.get 5
      local.get 4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      local.get 5
      i32.add
      local.get 1
      i32.load8_u offset=17
      i32.store8
      local.get 0
      local.get 2
      i32.const 19
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=18
      i32.store8
      local.get 0
      local.get 2
      i32.const 20
      i32.add
      local.tee 3
      i32.store
      local.get 3
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=8
      local.get 3
      i32.add
      local.get 1
      i32.load8_u offset=19
      i32.store8
      local.get 0
      i32.load offset=8
      local.get 2
      i32.const 21
      i32.add
      call $_ZN69_$LT$svm_sdk_host_ffi..ext..ExtHost$u20$as$u20$svm_sdk_host..Host$GT$14set_returndata17h757f3562e9537b0cE
      local.get 0
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $svm_fund (type 4))
  (func $_ZN11svm_sdk_std3try96_$LT$impl$u20$core..ops..try_trait..Try$u20$for$u20$svm_sdk_std..result..Result$LT$T$C$E$GT$$GT$6branch17habc8e1f00621f3feE (type 2) (param i32) (result i32)
    local.get 0
    i32.const 16776960
    i32.and
    i32.const 0
    local.get 0
    i32.const -16777216
    i32.and
    local.get 0
    i32.const 1
    i32.and
    local.tee 0
    select
    i32.or
    local.get 0
    i32.or)
  (func $_ZN15svm_abi_decoder7decoder7Decoder9peek_kind17h3270d3def6a08f5aE (type 5) (param i32 i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    local.get 1
    local.get 2
    call $_ZN15svm_abi_decoder6cursor6Cursor4peek17hbc10d49241fc5fb4E
    i32.const 1
    local.set 1
    local.get 3
    i32.load8_u offset=9
    i32.const 2
    local.get 3
    i32.load8_u offset=8
    i32.const 1
    i32.and
    local.tee 2
    select
    local.set 0
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 255
      i32.and
      local.set 2
      i32.const 0
      local.set 1
      i32.const 0
      local.set 0
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
                                  local.get 2
                                  br_table 2 (;@13;) 4 (;@11;) 5 (;@10;) 9 (;@6;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 2 (;@13;) 4 (;@11;) 6 (;@9;) 9 (;@6;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 14 (;@1;) 4 (;@11;) 7 (;@8;) 9 (;@6;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 1 (;@14;) 4 (;@11;) 7 (;@8;) 9 (;@6;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 3 (;@12;) 4 (;@11;) 8 (;@7;) 10 (;@5;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 4 (;@11;) 8 (;@7;) 10 (;@5;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 4 (;@11;) 0 (;@15;) 10 (;@5;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 0 (;@15;) 4 (;@11;) 0 (;@15;) 10 (;@5;) 11 (;@4;) 12 (;@3;) 13 (;@2;) 0 (;@15;)
                                end
                                unreachable
                                unreachable
                              end
                              i32.const 1
                              local.set 0
                              br 12 (;@1;)
                            end
                            i32.const 2
                            local.set 0
                            br 11 (;@1;)
                          end
                          i32.const 3
                          local.set 0
                          br 10 (;@1;)
                        end
                        i32.const 4
                        local.set 0
                        br 9 (;@1;)
                      end
                      i32.const 5
                      local.set 0
                      br 8 (;@1;)
                    end
                    i32.const 6
                    local.set 0
                    br 7 (;@1;)
                  end
                  i32.const 7
                  local.set 0
                  br 6 (;@1;)
                end
                i32.const 8
                local.set 0
                br 5 (;@1;)
              end
              i32.const 9
              local.set 0
              br 4 (;@1;)
            end
            i32.const 10
            local.set 0
            br 3 (;@1;)
          end
          i32.const 11
          local.set 0
          br 2 (;@1;)
        end
        i32.const 12
        local.set 0
        br 1 (;@1;)
      end
      i32.const 13
      local.set 0
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0
    local.get 0
    i32.const 255
    i32.and
    i32.const 8
    i32.shl
    local.get 1
    i32.or)
  (func $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E (type 2) (param i32) (result i32)
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
    i32.load
    local.get 0
    i32.const 4
    i32.add
    i32.load
    local.get 0
    i32.const 8
    i32.add
    local.tee 0
    i32.load
    local.tee 2
    call $_ZN15svm_abi_decoder6cursor6Cursor4peek17hbc10d49241fc5fb4E
    local.get 1
    i32.load8_u offset=9
    local.set 3
    local.get 1
    i32.load8_u offset=8
    local.set 4
    local.get 0
    local.get 2
    i32.const 1
    i32.add
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 3
    i32.const 8
    i32.shl
    i32.const 512
    local.get 4
    i32.const 1
    i32.and
    local.tee 1
    select
    local.get 1
    i32.const 1
    i32.xor
    i32.or)
  (func $_ZN15svm_abi_decoder7decoder7Decoder16decode_primitive17h2fd67ccb10984e8dE (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    local.get 1
    i32.const 4
    i32.add
    i32.load
    local.get 1
    i32.const 8
    i32.add
    i32.load
    call $_ZN15svm_abi_decoder7decoder7Decoder9peek_kind17h3270d3def6a08f5aE
    local.tee 3
    i32.const 16776960
    i32.and
    i32.const 8
    i32.shr_u
    local.set 4
    i32.const 1
    local.set 5
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
                              local.get 3
                              i32.const 1
                              i32.and
                              br_if 0 (;@13;)
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              block  ;; label = @30
                                                                block  ;; label = @31
                                                                  local.get 4
                                                                  i32.const 255
                                                                  i32.and
                                                                  br_table 0 (;@31;) 1 (;@30;) 4 (;@27;) 5 (;@26;) 6 (;@25;) 7 (;@24;) 8 (;@23;) 15 (;@16;) 14 (;@17;) 13 (;@18;) 12 (;@19;) 11 (;@20;) 9 (;@22;) 21 (;@10;)
                                                                end
                                                                local.get 1
                                                                call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                                                local.tee 1
                                                                i32.const 1
                                                                i32.and
                                                                i32.eqz
                                                                br_if 1 (;@29;)
                                                                local.get 1
                                                                i32.const 16777215
                                                                i32.and
                                                                local.tee 1
                                                                i32.const 8
                                                                i32.shr_u
                                                                local.set 3
                                                                local.get 1
                                                                i32.const 16
                                                                i32.shr_u
                                                                local.set 1
                                                                br 26 (;@4;)
                                                              end
                                                              i32.const 1
                                                              local.set 4
                                                              local.get 1
                                                              call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                                              local.tee 1
                                                              i32.const 1
                                                              i32.and
                                                              i32.eqz
                                                              br_if 1 (;@28;)
                                                              local.get 1
                                                              i32.const 16777215
                                                              i32.and
                                                              local.tee 3
                                                              i32.const 16
                                                              i32.shr_u
                                                              local.set 1
                                                              local.get 3
                                                              i32.const 8
                                                              i32.shr_u
                                                              local.set 3
                                                              br 25 (;@4;)
                                                            end
                                                            i32.const 0
                                                            local.set 4
                                                          end
                                                          br 25 (;@2;)
                                                        end
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                                        local.tee 1
                                                        i32.const 16776960
                                                        i32.and
                                                        i32.const 8
                                                        i32.shr_u
                                                        local.set 3
                                                        local.get 1
                                                        i32.const 1
                                                        i32.and
                                                        br_if 5 (;@21;)
                                                        local.get 3
                                                        i32.const 255
                                                        i32.and
                                                        local.set 1
                                                        i32.const 0
                                                        local.set 3
                                                        block  ;; label = @27
                                                          local.get 1
                                                          i32.eqz
                                                          br_if 0 (;@27;)
                                                          local.get 1
                                                          i32.const 16
                                                          i32.ne
                                                          br_if 17 (;@10;)
                                                          i32.const 1
                                                          local.set 3
                                                        end
                                                        i32.const 2
                                                        local.set 4
                                                        br 23 (;@3;)
                                                      end
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                                      local.tee 3
                                                      i32.const 1
                                                      i32.and
                                                      br_if 13 (;@12;)
                                                      local.get 2
                                                      local.get 1
                                                      i32.const 20
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h97cd6fc8fdeaa7d9E
                                                      local.get 2
                                                      i32.load8_u
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 20 (;@5;)
                                                      local.get 2
                                                      i32.load offset=4
                                                      local.set 5
                                                      i32.const 3
                                                      local.set 4
                                                      br 23 (;@2;)
                                                    end
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
                                                    local.tee 4
                                                    i32.const 16776960
                                                    i32.and
                                                    i32.const 8
                                                    i32.shr_u
                                                    local.set 3
                                                    local.get 4
                                                    i32.const 1
                                                    i32.and
                                                    br_if 13 (;@11;)
                                                    local.get 3
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
                                                    local.tee 3
                                                    i32.const 8
                                                    i32.ge_u
                                                    br_if 14 (;@10;)
                                                    local.get 2
                                                    local.get 1
                                                    local.get 3
                                                    i32.const 1
                                                    i32.add
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E
                                                    local.get 2
                                                    i32.load8_u
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 19 (;@5;)
                                                    local.get 2
                                                    i32.const 8
                                                    i32.add
                                                    i64.load
                                                    local.set 6
                                                    i32.const 4
                                                    local.set 4
                                                    br 22 (;@2;)
                                                  end
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817hbeb650df4f6220abE
                                                  local.tee 1
                                                  i32.const 16776960
                                                  i32.and
                                                  i32.const 8
                                                  i32.shr_u
                                                  local.set 3
                                                  local.get 1
                                                  i32.const 1
                                                  i32.and
                                                  br_if 9 (;@14;)
                                                  i32.const 5
                                                  local.set 4
                                                  br 20 (;@3;)
                                                end
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817hbeb650df4f6220abE
                                                local.tee 1
                                                i32.const 16776960
                                                i32.and
                                                i32.const 8
                                                i32.shr_u
                                                local.set 3
                                                local.get 1
                                                i32.const 1
                                                i32.and
                                                br_if 7 (;@15;)
                                                i32.const 6
                                                local.set 4
                                                br 19 (;@3;)
                                              end
                                              local.get 2
                                              local.get 1
                                              call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h57dc199ba3fe3210E
                                              local.get 2
                                              i32.load8_u
                                              i32.const 1
                                              i32.eq
                                              br_if 16 (;@5;)
                                              local.get 2
                                              i32.const 8
                                              i32.add
                                              i64.load
                                              local.set 6
                                              i32.const 12
                                              local.set 4
                                              br 19 (;@2;)
                                            end
                                            local.get 1
                                            i32.const 16711680
                                            i32.and
                                            i32.const 16
                                            i32.shr_u
                                            local.set 1
                                            br 16 (;@4;)
                                          end
                                          local.get 2
                                          local.get 1
                                          call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h57dc199ba3fe3210E
                                          local.get 2
                                          i32.load8_u
                                          i32.const 1
                                          i32.eq
                                          br_if 14 (;@5;)
                                          local.get 2
                                          i32.const 8
                                          i32.add
                                          i64.load
                                          local.set 6
                                          i32.const 11
                                          local.set 4
                                          br 17 (;@2;)
                                        end
                                        local.get 1
                                        call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h517fe6432775e879E
                                        local.tee 6
                                        i64.const 1
                                        i64.and
                                        i64.eqz
                                        br_if 12 (;@6;)
                                        local.get 6
                                        i64.const 8
                                        i64.shr_u
                                        i32.wrap_i64
                                        local.set 3
                                        local.get 6
                                        i64.const 16
                                        i64.shr_u
                                        i32.wrap_i64
                                        local.set 1
                                        br 14 (;@4;)
                                      end
                                      local.get 1
                                      call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h517fe6432775e879E
                                      local.tee 6
                                      i64.const 1
                                      i64.and
                                      i64.eqz
                                      br_if 10 (;@7;)
                                      local.get 6
                                      i64.const 8
                                      i64.shr_u
                                      i32.wrap_i64
                                      local.set 3
                                      local.get 6
                                      i64.const 16
                                      i64.shr_u
                                      i32.wrap_i64
                                      local.set 1
                                      br 13 (;@4;)
                                    end
                                    local.get 1
                                    call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617hb0d3c26768c1919eE
                                    call $_ZN11svm_sdk_std3try96_$LT$impl$u20$core..ops..try_trait..Try$u20$for$u20$svm_sdk_std..result..Result$LT$T$C$E$GT$$GT$6branch17habc8e1f00621f3feE
                                    local.tee 3
                                    i32.const 16711680
                                    i32.and
                                    i32.const 0
                                    local.get 3
                                    i32.const -16777216
                                    i32.and
                                    local.get 3
                                    i32.const 1
                                    i32.and
                                    local.tee 4
                                    select
                                    i32.or
                                    i32.const 16
                                    i32.shr_u
                                    local.set 1
                                    local.get 4
                                    i32.eqz
                                    br_if 8 (;@8;)
                                    local.get 3
                                    i32.const 8
                                    i32.shr_u
                                    local.set 3
                                    br 12 (;@4;)
                                  end
                                  local.get 1
                                  call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617hb0d3c26768c1919eE
                                  call $_ZN11svm_sdk_std3try96_$LT$impl$u20$core..ops..try_trait..Try$u20$for$u20$svm_sdk_std..result..Result$LT$T$C$E$GT$$GT$6branch17habc8e1f00621f3feE
                                  local.tee 3
                                  i32.const 16
                                  i32.shr_u
                                  local.set 1
                                  local.get 3
                                  i32.const 1
                                  i32.and
                                  i32.eqz
                                  br_if 6 (;@9;)
                                  local.get 3
                                  i32.const 8
                                  i32.shr_u
                                  local.set 3
                                  br 11 (;@4;)
                                end
                                local.get 1
                                i32.const 16711680
                                i32.and
                                i32.const 16
                                i32.shr_u
                                local.set 1
                                br 10 (;@4;)
                              end
                              local.get 1
                              i32.const 16711680
                              i32.and
                              i32.const 16
                              i32.shr_u
                              local.set 1
                              br 9 (;@4;)
                            end
                            local.get 0
                            local.get 4
                            i32.store8 offset=1
                            local.get 0
                            i32.const 2
                            i32.add
                            local.get 3
                            i32.const 16
                            i32.shr_u
                            i32.store8
                            br 11 (;@1;)
                          end
                          local.get 3
                          i32.const 16777215
                          i32.and
                          local.tee 1
                          i32.const 8
                          i32.shr_u
                          local.set 3
                          local.get 1
                          i32.const 16
                          i32.shr_u
                          local.set 1
                          br 7 (;@4;)
                        end
                        local.get 4
                        i32.const 16711680
                        i32.and
                        i32.const 16
                        i32.shr_u
                        local.set 1
                        br 6 (;@4;)
                      end
                      unreachable
                      unreachable
                    end
                    i32.const 7
                    local.set 4
                    br 6 (;@2;)
                  end
                  i32.const 8
                  local.set 4
                  br 5 (;@2;)
                end
                local.get 6
                i64.const 32
                i64.shr_u
                i32.wrap_i64
                local.set 5
                i32.const 9
                local.set 4
                br 4 (;@2;)
              end
              local.get 6
              i64.const 32
              i64.shr_u
              i32.wrap_i64
              local.set 5
              i32.const 10
              local.set 4
              br 3 (;@2;)
            end
            local.get 2
            i32.load8_u offset=2
            local.set 1
            local.get 2
            i32.load8_u offset=1
            local.set 3
          end
          local.get 0
          local.get 3
          i32.store8 offset=1
          local.get 0
          i32.const 2
          i32.add
          local.get 1
          i32.store8
          br 2 (;@1;)
        end
      end
      local.get 0
      i32.const 24
      i32.add
      local.get 6
      i64.store
      local.get 0
      i32.const 20
      i32.add
      local.get 5
      i32.store
      local.get 0
      i32.const 18
      i32.add
      local.get 1
      i32.store16
      local.get 0
      i32.const 17
      i32.add
      local.get 3
      i32.store8
      local.get 0
      i32.const 16
      i32.add
      local.get 4
      i32.store8
      i32.const 0
      local.set 5
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
    end
    local.get 0
    local.get 5
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN15svm_abi_decoder6cursor6Cursor4peek17hbc10d49241fc5fb4E (type 6) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 3
      local.get 2
      i32.lt_u
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      local.get 3
      i32.add
      i32.load8_u
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8 offset=1
    local.get 0
    local.get 2
    i32.store8)
  (func $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h97cd6fc8fdeaa7d9E (type 7) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=8
          local.tee 3
          local.get 2
          i32.add
          local.tee 4
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.load offset=4
          i32.le_u
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 2
        i32.store8 offset=1
        i32.const 1
        local.set 1
        br 1 (;@1;)
      end
      local.get 1
      local.get 4
      i32.store offset=8
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i32.store
      local.get 0
      i32.const 4
      i32.add
      local.get 1
      i32.load
      local.get 3
      i32.add
      i32.store
      i32.const 0
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8)
  (func $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E (type 7) (param i32 i32 i32)
    (local i32 i32 i64 i64 i64 i64 i64 i64 i64 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    local.get 2
    call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h97cd6fc8fdeaa7d9E
    i32.const 1
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.load8_u
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        local.get 3
        i32.load16_u offset=1 align=1
        i32.store16 offset=1 align=1
        br 1 (;@1;)
      end
      local.get 2
      local.get 3
      i32.load offset=4
      local.tee 4
      i32.add
      local.tee 1
      i32.const -1
      i32.add
      i64.load8_u
      local.set 5
      i64.const 0
      local.set 6
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              i64.const 0
              local.set 7
              i64.const 0
              local.set 8
              i64.const 0
              local.set 9
              i64.const 0
              local.set 10
              br 1 (;@4;)
            end
            local.get 1
            i32.const -2
            i32.add
            i64.load8_u
            i64.const 48
            i64.shl
            local.set 10
            block  ;; label = @5
              local.get 2
              i32.const 3
              i32.ge_u
              br_if 0 (;@5;)
              i64.const 0
              local.set 7
              i64.const 0
              local.set 8
              i64.const 0
              local.set 9
              br 1 (;@4;)
            end
            local.get 1
            i32.const -3
            i32.add
            i64.load8_u
            i64.const 40
            i64.shl
            local.set 9
            local.get 2
            i32.const 3
            i32.ne
            br_if 1 (;@3;)
            i64.const 0
            local.set 7
            i64.const 0
            local.set 8
          end
          i64.const 0
          local.set 11
          i64.const 0
          local.set 12
          br 1 (;@2;)
        end
        local.get 1
        i32.const -4
        i32.add
        i64.load8_u
        i64.const 32
        i64.shl
        local.set 11
        block  ;; label = @3
          local.get 2
          i32.const 5
          i32.ge_u
          br_if 0 (;@3;)
          i64.const 0
          local.set 7
          i64.const 0
          local.set 8
          i64.const 0
          local.set 12
          br 1 (;@2;)
        end
        local.get 1
        i32.const -5
        i32.add
        i64.load8_u
        i64.const 24
        i64.shl
        local.set 8
        block  ;; label = @3
          local.get 2
          i32.const 5
          i32.ne
          br_if 0 (;@3;)
          i64.const 0
          local.set 7
          i64.const 0
          local.set 12
          br 1 (;@2;)
        end
        local.get 1
        i32.const -6
        i32.add
        i64.load8_u
        i64.const 16
        i64.shl
        local.set 12
        block  ;; label = @3
          local.get 2
          i32.const 7
          i32.ge_u
          br_if 0 (;@3;)
          i64.const 0
          local.set 7
          br 1 (;@2;)
        end
        local.get 1
        i32.const -7
        i32.add
        i64.load8_u
        i64.const 8
        i64.shl
        local.set 7
        local.get 2
        i32.const 7
        i32.eq
        br_if 0 (;@2;)
        local.get 4
        i64.load8_u
        local.set 6
      end
      local.get 0
      i32.const 8
      i32.add
      local.get 7
      local.get 5
      i64.const 56
      i64.shl
      i64.or
      local.get 8
      i64.or
      local.get 9
      i64.or
      local.get 10
      i64.or
      local.get 11
      i64.or
      local.get 12
      i64.or
      local.get 6
      i64.or
      local.tee 6
      i64.const 56
      i64.shl
      local.get 6
      i64.const 40
      i64.shl
      i64.const 71776119061217280
      i64.and
      i64.or
      local.get 6
      i64.const 24
      i64.shl
      i64.const 280375465082880
      i64.and
      local.get 6
      i64.const 8
      i64.shl
      i64.const 1095216660480
      i64.and
      i64.or
      i64.or
      local.get 6
      i64.const 8
      i64.shr_u
      i64.const 4278190080
      i64.and
      local.get 6
      i64.const 24
      i64.shr_u
      i64.const 16711680
      i64.and
      i64.or
      local.get 6
      i64.const 40
      i64.shr_u
      i64.const 65280
      i64.and
      local.get 6
      i64.const 56
      i64.shr_u
      i64.or
      i64.or
      i64.or
      i64.store
      i32.const 0
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store8
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817hbeb650df4f6220abE (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    i32.const 1
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        local.get 1
        local.get 0
        i32.const 1
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E
        block  ;; label = @3
          local.get 1
          i32.load8_u
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 8
          i32.add
          i32.load8_u
          local.set 0
          i32.const 0
          local.set 2
          br 2 (;@1;)
        end
        local.get 1
        i32.load8_u offset=2
        local.set 3
        local.get 1
        i32.load8_u offset=1
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i32.const 16777215
      i32.and
      local.tee 0
      i32.const 16
      i32.shr_u
      local.set 3
      local.get 0
      i32.const 8
      i32.shr_u
      local.set 0
    end
    local.get 1
    i32.const 16
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h57dc199ba3fe3210E (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
    local.tee 3
    i32.const 16776960
    i32.and
    i32.const 8
    i32.shr_u
    local.set 4
    i32.const 1
    local.set 5
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        block  ;; label = @3
          local.get 4
          i32.const 255
          i32.and
          local.tee 3
          i32.const -4
          i32.add
          i32.const 2
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
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
                        i32.lt_u
                        br_if 6 (;@4;)
                        unreachable
                        unreachable
                      end
                      i32.const 2
                      local.set 5
                      br 6 (;@3;)
                    end
                    i32.const 3
                    local.set 5
                    br 5 (;@3;)
                  end
                  i32.const 4
                  local.set 5
                  br 4 (;@3;)
                end
                i32.const 5
                local.set 5
                br 3 (;@3;)
              end
              i32.const 6
              local.set 5
              br 2 (;@3;)
            end
            i32.const 7
            local.set 5
            br 1 (;@3;)
          end
          i32.const 8
          local.set 5
        end
        local.get 2
        local.get 1
        local.get 5
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E
        block  ;; label = @3
          local.get 2
          i32.load8_u
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.const 8
          i32.add
          i64.load
          i64.store
          i32.const 0
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        i32.load16_u offset=1 align=1
        i32.store16 offset=1 align=1
        i32.const 1
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i32.store8 offset=1
      local.get 0
      i32.const 2
      i32.add
      local.get 3
      i32.const 16
      i32.shr_u
      i32.store8
    end
    local.get 0
    local.get 5
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h517fe6432775e879E (type 8) (param i32) (result i64)
    (local i32 i32 i64 i64 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
            local.tee 2
            i32.const 1
            i32.and
            br_if 0 (;@4;)
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
            local.tee 2
            i32.const 255
            i32.and
            i32.const 8
            i32.ge_u
            br_if 1 (;@3;)
            local.get 1
            local.get 0
            local.get 2
            i32.const 24
            i32.shl
            i32.const 24
            i32.shr_s
            i32.const 2
            i32.shl
            i32.const 1048592
            i32.add
            i32.load
            call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E
            block  ;; label = @5
              local.get 1
              i32.load8_u
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              local.get 1
              i32.const 8
              i32.add
              i64.load
              i64.const 32
              i64.shl
              local.set 3
              i64.const 0
              local.set 4
              i64.const 0
              local.set 5
              br 4 (;@1;)
            end
            local.get 1
            i64.load8_u offset=2
            i64.const 16
            i64.shl
            local.get 1
            i64.load8_u offset=1
            i64.const 8
            i64.shl
            i64.or
            local.tee 4
            i64.const 1
            i64.or
            local.set 5
            br 2 (;@2;)
          end
          local.get 2
          i32.const 1
          i32.or
          i64.extend_i32_u
          local.tee 3
          i64.const 16776960
          i64.and
          local.set 4
          local.get 3
          i64.const 16776961
          i64.and
          local.set 5
          br 1 (;@2;)
        end
        unreachable
        unreachable
      end
      i64.const 0
      local.set 3
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 3
    local.get 4
    i64.const 16776960
    i64.and
    i64.or
    local.get 5
    i64.const 255
    i64.and
    i64.or)
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617hb0d3c26768c1919eE (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h925d8c0577d7a617E
          local.tee 2
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.const 16776960
          i32.and
          i32.const 8
          i32.shr_u
          i32.const -34
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
          local.tee 2
          i32.const 255
          i32.and
          i32.const 4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 1
          local.get 0
          local.get 2
          i32.const 24
          i32.shl
          i32.const 24
          i32.shr_s
          i32.const 2
          i32.shl
          i32.const 1048576
          i32.add
          i32.load
          call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h4a6e47a05aaa8034E
          block  ;; label = @4
            local.get 1
            i32.load8_u
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 8
            i32.add
            i32.load
            local.set 0
            i32.const 0
            local.set 2
            i32.const 0
            local.set 3
            br 3 (;@1;)
          end
          local.get 1
          i32.load8_u offset=2
          local.tee 0
          i32.const 16
          i32.shl
          local.get 1
          i32.load8_u offset=1
          i32.const 8
          i32.shl
          i32.or
          local.tee 2
          i32.const 1
          i32.or
          local.set 3
          br 2 (;@1;)
        end
        local.get 2
        i32.const 1
        i32.or
        local.tee 0
        i32.const 16776960
        i32.and
        local.set 2
        local.get 0
        i32.const 16776961
        i32.and
        local.tee 3
        i32.const 16
        i32.shr_u
        local.set 0
        br 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
    i32.const 65280
    i32.and
    local.get 0
    i32.const 16
    i32.shl
    i32.or
    local.get 3
    i32.const 255
    i32.and
    i32.or)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048625))
  (global (;2;) i32 (i32.const 1048640))
  (export "memory" (memory 0))
  (export "svm_verify" (func $svm_verify))
  (export "svm_alloc" (func $svm_alloc))
  (export "initialize" (func $initialize))
  (export "store_addr" (func $store_addr))
  (export "load_addr" (func $load_addr))
  (export "svm_fund" (func $svm_fund))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "\01\00\00\00\02\00\00\00\01\00\00\00\02\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00"))
