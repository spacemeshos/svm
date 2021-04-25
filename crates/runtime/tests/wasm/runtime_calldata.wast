(module
  (type (;0;) (func (param i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func))
  (type (;4;) (func (param i32 i32) (result i64)))
  (type (;5;) (func (param i32 i64)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32) (result i64)))
  (import "svm" "svm_static_alloc" (func $_ZN13svm_sdk_alloc16svm_static_alloc17hd7c2b9494a084826E (type 0)))
  (import "svm" "svm_calldata_offset" (func $_ZN12svm_sdk_host3ext19svm_calldata_offset17ha645deed970a699cE (type 1)))
  (import "svm" "svm_calldata_len" (func $_ZN12svm_sdk_host3ext16svm_calldata_len17h3119c678b06c0b04E (type 1)))
  (import "svm" "svm_store160" (func $_ZN15svm_sdk_storage3ext12svm_store16017h77c58584d160d8c0E (type 2)))
  (import "svm" "svm_load160" (func $_ZN15svm_sdk_storage3ext11svm_load16017hb1b13333e9a2c166E (type 2)))
  (import "svm" "svm_set_returndata" (func $_ZN12svm_sdk_host3ext18svm_set_returndata17h64ba12fb97dce274E (type 2)))
  (func $svm_alloc (type 0) (param i32) (result i32)
    local.get 0
    call $_ZN13svm_sdk_alloc16svm_static_alloc17hd7c2b9494a084826E)
  (func $initialize (type 3))
  (func $store_addr (type 3)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
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
    call $_ZN12svm_sdk_host3ext19svm_calldata_offset17ha645deed970a699cE
    local.set 1
    local.get 0
    call $_ZN12svm_sdk_host3ext16svm_calldata_len17h3119c678b06c0b04E
    local.tee 2
    i32.store offset=8
    local.get 0
    i32.const 0
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      i32.add
      local.get 0
      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
      local.get 0
      i32.load8_u offset=16
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 0
        i32.load offset=24
        br_table 0 (;@2;) 1 (;@1;) 1 (;@1;) 1 (;@1;)
      end
      local.get 0
      i32.const 32
      i32.add
      i32.load8_u
      i32.const 255
      i32.and
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 36
      i32.add
      i32.load
      i32.const 0
      call $_ZN15svm_sdk_storage3ext12svm_store16017h77c58584d160d8c0E
      local.get 0
      i32.const 48
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i64 i64 i32)
    global.get 0
    i32.const 240
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=4
          local.get 1
          i32.load offset=8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 1
          i32.store16
          br 1 (;@2;)
        end
        local.get 2
        i32.const 120
        i32.add
        local.get 1
        call $_ZN15svm_abi_decoder6cursor6Cursor4peek17h9ae4de5b86ca23f8E
        local.get 2
        i32.load8_u offset=121
        i32.const 2
        local.get 2
        i32.load8_u offset=120
        i32.const 1
        i32.and
        local.tee 3
        select
        local.set 4
        local.get 3
        i32.const 1
        i32.xor
        local.set 5
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
                                                local.get 3
                                                i32.eqz
                                                br_if 0 (;@22;)
                                                local.get 5
                                                br_if 21 (;@1;)
                                                local.get 4
                                                i32.const 255
                                                i32.and
                                                br_table 4 (;@18;) 6 (;@16;) 7 (;@15;) 11 (;@11;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 4 (;@18;) 6 (;@16;) 8 (;@14;) 11 (;@11;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 2 (;@20;) 6 (;@16;) 9 (;@13;) 11 (;@11;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 3 (;@19;) 6 (;@16;) 9 (;@13;) 11 (;@11;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 5 (;@17;) 6 (;@16;) 10 (;@12;) 12 (;@10;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 6 (;@16;) 10 (;@12;) 12 (;@10;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 6 (;@16;) 21 (;@1;) 12 (;@10;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 21 (;@1;) 6 (;@16;) 21 (;@1;) 12 (;@10;) 13 (;@9;) 14 (;@8;) 1 (;@21;) 21 (;@1;)
                                              end
                                              local.get 2
                                              local.get 4
                                              i32.const 255
                                              i32.and
                                              i32.const 8
                                              i32.shl
                                              local.get 5
                                              i32.or
                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
                                              local.get 0
                                              local.get 2
                                              i32.load16_u
                                              i32.store16 offset=1 align=1
                                              local.get 0
                                              i32.const 1
                                              i32.store8
                                              br 19 (;@2;)
                                            end
                                            local.get 1
                                            call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                            local.tee 3
                                            i32.const 255
                                            i32.and
                                            i32.const 1
                                            i32.eq
                                            br_if 13 (;@7;)
                                            local.get 3
                                            i32.const 1
                                            i32.and
                                            br_if 19 (;@1;)
                                            i32.const 0
                                            local.set 4
                                            local.get 3
                                            i32.const 8
                                            i32.shr_u
                                            i32.const 255
                                            i32.and
                                            local.tee 3
                                            i32.const 24
                                            i32.mul
                                            call $_ZN13svm_sdk_alloc16svm_static_alloc17hd7c2b9494a084826E
                                            local.set 5
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        local.get 3
                                                        i32.const -6
                                                        i32.add
                                                        br_table 4 (;@22;) 1 (;@25;) 0 (;@26;)
                                                      end
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              block  ;; label = @30
                                                                block  ;; label = @31
                                                                  block  ;; label = @32
                                                                    block  ;; label = @33
                                                                      local.get 3
                                                                      i32.const -22
                                                                      i32.add
                                                                      br_table 1 (;@32;) 9 (;@24;) 0 (;@33;)
                                                                    end
                                                                    block  ;; label = @33
                                                                      local.get 3
                                                                      i32.const -38
                                                                      i32.add
                                                                      br_table 2 (;@31;) 10 (;@23;) 0 (;@33;)
                                                                    end
                                                                    local.get 3
                                                                    i32.const 54
                                                                    i32.eq
                                                                    br_if 2 (;@30;)
                                                                    local.get 3
                                                                    i32.const 70
                                                                    i32.eq
                                                                    br_if 3 (;@29;)
                                                                    local.get 3
                                                                    i32.const 86
                                                                    i32.eq
                                                                    br_if 4 (;@28;)
                                                                    local.get 3
                                                                    i32.const 102
                                                                    i32.eq
                                                                    br_if 5 (;@27;)
                                                                    local.get 3
                                                                    i32.const 118
                                                                    i32.eq
                                                                    br_if 6 (;@26;)
                                                                    br 31 (;@1;)
                                                                  end
                                                                  local.get 2
                                                                  i32.const 176
                                                                  i32.add
                                                                  local.get 1
                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                                  local.get 2
                                                                  i32.load8_u offset=176
                                                                  i32.const 1
                                                                  i32.eq
                                                                  br_if 10 (;@21;)
                                                                  local.get 2
                                                                  i32.const 208
                                                                  i32.add
                                                                  i32.const 16
                                                                  i32.add
                                                                  local.get 2
                                                                  i32.const 193
                                                                  i32.add
                                                                  i64.load align=1
                                                                  i64.store
                                                                  local.get 2
                                                                  i32.const 208
                                                                  i32.add
                                                                  i32.const 8
                                                                  i32.add
                                                                  local.get 2
                                                                  i32.const 185
                                                                  i32.add
                                                                  i64.load align=1
                                                                  i64.store
                                                                  local.get 2
                                                                  i32.const 231
                                                                  i32.add
                                                                  local.tee 1
                                                                  local.get 2
                                                                  i32.const 200
                                                                  i32.add
                                                                  i64.load align=1
                                                                  i64.store align=1
                                                                  local.get 2
                                                                  i32.const 152
                                                                  i32.add
                                                                  i32.const 8
                                                                  i32.add
                                                                  local.tee 4
                                                                  local.get 2
                                                                  i32.const 223
                                                                  i32.add
                                                                  i64.load align=1
                                                                  i64.store
                                                                  local.get 2
                                                                  i32.const 152
                                                                  i32.add
                                                                  i32.const 16
                                                                  i32.add
                                                                  local.tee 6
                                                                  local.get 1
                                                                  i64.load align=1
                                                                  i64.store
                                                                  local.get 2
                                                                  local.get 2
                                                                  i64.load offset=177 align=1
                                                                  i64.store offset=208
                                                                  local.get 2
                                                                  local.get 2
                                                                  i64.load offset=215 align=1
                                                                  i64.store offset=152
                                                                  local.get 3
                                                                  i32.eqz
                                                                  br_if 30 (;@1;)
                                                                  local.get 4
                                                                  i64.load
                                                                  local.set 7
                                                                  local.get 2
                                                                  i64.load offset=152
                                                                  local.set 8
                                                                  local.get 5
                                                                  i32.const 16
                                                                  i32.add
                                                                  local.get 6
                                                                  i64.load
                                                                  i64.store
                                                                  local.get 5
                                                                  i32.const 8
                                                                  i32.add
                                                                  local.get 7
                                                                  i64.store
                                                                  local.get 5
                                                                  local.get 8
                                                                  i64.store
                                                                  i32.const 1
                                                                  local.set 4
                                                                  br 9 (;@22;)
                                                                end
                                                                local.get 2
                                                                i32.const 176
                                                                i32.add
                                                                local.get 1
                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                                local.get 2
                                                                i32.load8_u offset=176
                                                                i32.const 1
                                                                i32.eq
                                                                br_if 9 (;@21;)
                                                                local.get 2
                                                                i32.const 208
                                                                i32.add
                                                                i32.const 16
                                                                i32.add
                                                                local.get 2
                                                                i32.const 193
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 208
                                                                i32.add
                                                                i32.const 8
                                                                i32.add
                                                                local.get 2
                                                                i32.const 185
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 231
                                                                i32.add
                                                                local.tee 4
                                                                local.get 2
                                                                i32.const 200
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store align=1
                                                                local.get 2
                                                                i32.const 152
                                                                i32.add
                                                                i32.const 8
                                                                i32.add
                                                                local.tee 6
                                                                local.get 2
                                                                i32.const 223
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 152
                                                                i32.add
                                                                i32.const 16
                                                                i32.add
                                                                local.tee 9
                                                                local.get 4
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                local.get 2
                                                                i64.load offset=177 align=1
                                                                i64.store offset=208
                                                                local.get 2
                                                                local.get 2
                                                                i64.load offset=215 align=1
                                                                i64.store offset=152
                                                                local.get 3
                                                                i32.eqz
                                                                br_if 29 (;@1;)
                                                                local.get 6
                                                                i64.load
                                                                local.set 7
                                                                local.get 2
                                                                i64.load offset=152
                                                                local.set 8
                                                                local.get 5
                                                                i32.const 16
                                                                i32.add
                                                                local.get 9
                                                                i64.load
                                                                i64.store
                                                                local.get 5
                                                                i32.const 8
                                                                i32.add
                                                                local.get 7
                                                                i64.store
                                                                local.get 5
                                                                local.get 8
                                                                i64.store
                                                                local.get 2
                                                                i32.const 176
                                                                i32.add
                                                                local.get 1
                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                                local.get 2
                                                                i32.load8_u offset=176
                                                                i32.const 1
                                                                i32.eq
                                                                br_if 9 (;@21;)
                                                                local.get 2
                                                                i32.const 208
                                                                i32.add
                                                                i32.const 16
                                                                i32.add
                                                                local.get 2
                                                                i32.const 193
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 208
                                                                i32.add
                                                                i32.const 8
                                                                i32.add
                                                                local.get 2
                                                                i32.const 185
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 231
                                                                i32.add
                                                                local.tee 1
                                                                local.get 2
                                                                i32.const 200
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store align=1
                                                                local.get 2
                                                                i32.const 152
                                                                i32.add
                                                                i32.const 8
                                                                i32.add
                                                                local.tee 4
                                                                local.get 2
                                                                i32.const 223
                                                                i32.add
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                i32.const 152
                                                                i32.add
                                                                i32.const 16
                                                                i32.add
                                                                local.tee 6
                                                                local.get 1
                                                                i64.load align=1
                                                                i64.store
                                                                local.get 2
                                                                local.get 2
                                                                i64.load offset=177 align=1
                                                                i64.store offset=208
                                                                local.get 2
                                                                local.get 2
                                                                i64.load offset=215 align=1
                                                                i64.store offset=152
                                                                local.get 3
                                                                i32.const 1
                                                                i32.le_u
                                                                br_if 29 (;@1;)
                                                                local.get 5
                                                                local.get 2
                                                                i64.load offset=152
                                                                i64.store offset=24
                                                                local.get 5
                                                                i32.const 40
                                                                i32.add
                                                                local.get 6
                                                                i64.load
                                                                i64.store
                                                                local.get 5
                                                                i32.const 32
                                                                i32.add
                                                                local.get 4
                                                                i64.load
                                                                i64.store
                                                                i32.const 2
                                                                local.set 4
                                                                br 8 (;@22;)
                                                              end
                                                              local.get 2
                                                              i32.const 176
                                                              i32.add
                                                              local.get 1
                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                              local.get 2
                                                              i32.load8_u offset=176
                                                              i32.const 1
                                                              i32.eq
                                                              br_if 8 (;@21;)
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.get 2
                                                              i32.const 193
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.get 2
                                                              i32.const 185
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 231
                                                              i32.add
                                                              local.tee 4
                                                              local.get 2
                                                              i32.const 200
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store align=1
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.tee 6
                                                              local.get 2
                                                              i32.const 223
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.tee 9
                                                              local.get 4
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=177 align=1
                                                              i64.store offset=208
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=215 align=1
                                                              i64.store offset=152
                                                              local.get 3
                                                              i32.eqz
                                                              br_if 28 (;@1;)
                                                              local.get 6
                                                              i64.load
                                                              local.set 7
                                                              local.get 2
                                                              i64.load offset=152
                                                              local.set 8
                                                              local.get 5
                                                              i32.const 16
                                                              i32.add
                                                              local.get 9
                                                              i64.load
                                                              i64.store
                                                              local.get 5
                                                              i32.const 8
                                                              i32.add
                                                              local.get 7
                                                              i64.store
                                                              local.get 5
                                                              local.get 8
                                                              i64.store
                                                              local.get 2
                                                              i32.const 176
                                                              i32.add
                                                              local.get 1
                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                              local.get 2
                                                              i32.load8_u offset=176
                                                              i32.const 1
                                                              i32.eq
                                                              br_if 8 (;@21;)
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.get 2
                                                              i32.const 193
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.get 2
                                                              i32.const 185
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 231
                                                              i32.add
                                                              local.tee 4
                                                              local.get 2
                                                              i32.const 200
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store align=1
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.tee 6
                                                              local.get 2
                                                              i32.const 223
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.tee 9
                                                              local.get 4
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=177 align=1
                                                              i64.store offset=208
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=215 align=1
                                                              i64.store offset=152
                                                              local.get 3
                                                              i32.const 1
                                                              i32.le_u
                                                              br_if 28 (;@1;)
                                                              local.get 5
                                                              local.get 2
                                                              i64.load offset=152
                                                              i64.store offset=24
                                                              local.get 5
                                                              i32.const 40
                                                              i32.add
                                                              local.get 9
                                                              i64.load
                                                              i64.store
                                                              local.get 5
                                                              i32.const 32
                                                              i32.add
                                                              local.get 6
                                                              i64.load
                                                              i64.store
                                                              local.get 2
                                                              i32.const 176
                                                              i32.add
                                                              local.get 1
                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                              local.get 2
                                                              i32.load8_u offset=176
                                                              i32.const 1
                                                              i32.eq
                                                              br_if 8 (;@21;)
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.get 2
                                                              i32.const 193
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 208
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.get 2
                                                              i32.const 185
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 231
                                                              i32.add
                                                              local.tee 1
                                                              local.get 2
                                                              i32.const 200
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store align=1
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 8
                                                              i32.add
                                                              local.tee 4
                                                              local.get 2
                                                              i32.const 223
                                                              i32.add
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              i32.const 152
                                                              i32.add
                                                              i32.const 16
                                                              i32.add
                                                              local.tee 6
                                                              local.get 1
                                                              i64.load align=1
                                                              i64.store
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=177 align=1
                                                              i64.store offset=208
                                                              local.get 2
                                                              local.get 2
                                                              i64.load offset=215 align=1
                                                              i64.store offset=152
                                                              local.get 3
                                                              i32.const 2
                                                              i32.le_u
                                                              br_if 28 (;@1;)
                                                              local.get 5
                                                              local.get 2
                                                              i64.load offset=152
                                                              i64.store offset=48
                                                              local.get 5
                                                              i32.const 64
                                                              i32.add
                                                              local.get 6
                                                              i64.load
                                                              i64.store
                                                              local.get 5
                                                              i32.const 56
                                                              i32.add
                                                              local.get 4
                                                              i64.load
                                                              i64.store
                                                              i32.const 3
                                                              local.set 4
                                                              br 7 (;@22;)
                                                            end
                                                            local.get 2
                                                            i32.const 176
                                                            i32.add
                                                            local.get 1
                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                            local.get 2
                                                            i32.load8_u offset=176
                                                            i32.const 1
                                                            i32.eq
                                                            br_if 7 (;@21;)
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.get 2
                                                            i32.const 193
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.get 2
                                                            i32.const 185
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 231
                                                            i32.add
                                                            local.tee 4
                                                            local.get 2
                                                            i32.const 200
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store align=1
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.tee 6
                                                            local.get 2
                                                            i32.const 223
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.tee 9
                                                            local.get 4
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=177 align=1
                                                            i64.store offset=208
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=215 align=1
                                                            i64.store offset=152
                                                            local.get 3
                                                            i32.eqz
                                                            br_if 27 (;@1;)
                                                            local.get 6
                                                            i64.load
                                                            local.set 7
                                                            local.get 2
                                                            i64.load offset=152
                                                            local.set 8
                                                            local.get 5
                                                            i32.const 16
                                                            i32.add
                                                            local.get 9
                                                            i64.load
                                                            i64.store
                                                            local.get 5
                                                            i32.const 8
                                                            i32.add
                                                            local.get 7
                                                            i64.store
                                                            local.get 5
                                                            local.get 8
                                                            i64.store
                                                            local.get 2
                                                            i32.const 176
                                                            i32.add
                                                            local.get 1
                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                            local.get 2
                                                            i32.load8_u offset=176
                                                            i32.const 1
                                                            i32.eq
                                                            br_if 7 (;@21;)
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.get 2
                                                            i32.const 193
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.get 2
                                                            i32.const 185
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 231
                                                            i32.add
                                                            local.tee 4
                                                            local.get 2
                                                            i32.const 200
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store align=1
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.tee 6
                                                            local.get 2
                                                            i32.const 223
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.tee 9
                                                            local.get 4
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=177 align=1
                                                            i64.store offset=208
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=215 align=1
                                                            i64.store offset=152
                                                            local.get 3
                                                            i32.const 1
                                                            i32.le_u
                                                            br_if 27 (;@1;)
                                                            local.get 5
                                                            local.get 2
                                                            i64.load offset=152
                                                            i64.store offset=24
                                                            local.get 5
                                                            i32.const 40
                                                            i32.add
                                                            local.get 9
                                                            i64.load
                                                            i64.store
                                                            local.get 5
                                                            i32.const 32
                                                            i32.add
                                                            local.get 6
                                                            i64.load
                                                            i64.store
                                                            local.get 2
                                                            i32.const 176
                                                            i32.add
                                                            local.get 1
                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                            local.get 2
                                                            i32.load8_u offset=176
                                                            i32.const 1
                                                            i32.eq
                                                            br_if 7 (;@21;)
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.get 2
                                                            i32.const 193
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.get 2
                                                            i32.const 185
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 231
                                                            i32.add
                                                            local.tee 4
                                                            local.get 2
                                                            i32.const 200
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store align=1
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.tee 6
                                                            local.get 2
                                                            i32.const 223
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.tee 9
                                                            local.get 4
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=177 align=1
                                                            i64.store offset=208
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=215 align=1
                                                            i64.store offset=152
                                                            local.get 3
                                                            i32.const 2
                                                            i32.le_u
                                                            br_if 27 (;@1;)
                                                            local.get 5
                                                            local.get 2
                                                            i64.load offset=152
                                                            i64.store offset=48
                                                            local.get 5
                                                            i32.const 64
                                                            i32.add
                                                            local.get 9
                                                            i64.load
                                                            i64.store
                                                            local.get 5
                                                            i32.const 56
                                                            i32.add
                                                            local.get 6
                                                            i64.load
                                                            i64.store
                                                            local.get 2
                                                            i32.const 176
                                                            i32.add
                                                            local.get 1
                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                            local.get 2
                                                            i32.load8_u offset=176
                                                            i32.const 1
                                                            i32.eq
                                                            br_if 7 (;@21;)
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.get 2
                                                            i32.const 193
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 208
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.get 2
                                                            i32.const 185
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 231
                                                            i32.add
                                                            local.tee 1
                                                            local.get 2
                                                            i32.const 200
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store align=1
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 8
                                                            i32.add
                                                            local.tee 4
                                                            local.get 2
                                                            i32.const 223
                                                            i32.add
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            i32.const 152
                                                            i32.add
                                                            i32.const 16
                                                            i32.add
                                                            local.tee 6
                                                            local.get 1
                                                            i64.load align=1
                                                            i64.store
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=177 align=1
                                                            i64.store offset=208
                                                            local.get 2
                                                            local.get 2
                                                            i64.load offset=215 align=1
                                                            i64.store offset=152
                                                            local.get 3
                                                            i32.const 3
                                                            i32.le_u
                                                            br_if 27 (;@1;)
                                                            local.get 5
                                                            local.get 2
                                                            i64.load offset=152
                                                            i64.store offset=72
                                                            local.get 5
                                                            i32.const 88
                                                            i32.add
                                                            local.get 6
                                                            i64.load
                                                            i64.store
                                                            local.get 5
                                                            i32.const 80
                                                            i32.add
                                                            local.get 4
                                                            i64.load
                                                            i64.store
                                                            i32.const 4
                                                            local.set 4
                                                            br 6 (;@22;)
                                                          end
                                                          local.get 2
                                                          i32.const 176
                                                          i32.add
                                                          local.get 1
                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                          local.get 2
                                                          i32.load8_u offset=176
                                                          i32.const 1
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.get 2
                                                          i32.const 193
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.get 2
                                                          i32.const 185
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 231
                                                          i32.add
                                                          local.tee 4
                                                          local.get 2
                                                          i32.const 200
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store align=1
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.tee 6
                                                          local.get 2
                                                          i32.const 223
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.tee 9
                                                          local.get 4
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=177 align=1
                                                          i64.store offset=208
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=215 align=1
                                                          i64.store offset=152
                                                          local.get 3
                                                          i32.eqz
                                                          br_if 26 (;@1;)
                                                          local.get 6
                                                          i64.load
                                                          local.set 7
                                                          local.get 2
                                                          i64.load offset=152
                                                          local.set 8
                                                          local.get 5
                                                          i32.const 16
                                                          i32.add
                                                          local.get 9
                                                          i64.load
                                                          i64.store
                                                          local.get 5
                                                          i32.const 8
                                                          i32.add
                                                          local.get 7
                                                          i64.store
                                                          local.get 5
                                                          local.get 8
                                                          i64.store
                                                          local.get 2
                                                          i32.const 176
                                                          i32.add
                                                          local.get 1
                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                          local.get 2
                                                          i32.load8_u offset=176
                                                          i32.const 1
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.get 2
                                                          i32.const 193
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.get 2
                                                          i32.const 185
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 231
                                                          i32.add
                                                          local.tee 4
                                                          local.get 2
                                                          i32.const 200
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store align=1
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.tee 6
                                                          local.get 2
                                                          i32.const 223
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.tee 9
                                                          local.get 4
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=177 align=1
                                                          i64.store offset=208
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=215 align=1
                                                          i64.store offset=152
                                                          local.get 3
                                                          i32.const 1
                                                          i32.le_u
                                                          br_if 26 (;@1;)
                                                          local.get 5
                                                          local.get 2
                                                          i64.load offset=152
                                                          i64.store offset=24
                                                          local.get 5
                                                          i32.const 40
                                                          i32.add
                                                          local.get 9
                                                          i64.load
                                                          i64.store
                                                          local.get 5
                                                          i32.const 32
                                                          i32.add
                                                          local.get 6
                                                          i64.load
                                                          i64.store
                                                          local.get 2
                                                          i32.const 176
                                                          i32.add
                                                          local.get 1
                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                          local.get 2
                                                          i32.load8_u offset=176
                                                          i32.const 1
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.get 2
                                                          i32.const 193
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.get 2
                                                          i32.const 185
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 231
                                                          i32.add
                                                          local.tee 4
                                                          local.get 2
                                                          i32.const 200
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store align=1
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.tee 6
                                                          local.get 2
                                                          i32.const 223
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.tee 9
                                                          local.get 4
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=177 align=1
                                                          i64.store offset=208
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=215 align=1
                                                          i64.store offset=152
                                                          local.get 3
                                                          i32.const 2
                                                          i32.le_u
                                                          br_if 26 (;@1;)
                                                          local.get 5
                                                          local.get 2
                                                          i64.load offset=152
                                                          i64.store offset=48
                                                          local.get 5
                                                          i32.const 64
                                                          i32.add
                                                          local.get 9
                                                          i64.load
                                                          i64.store
                                                          local.get 5
                                                          i32.const 56
                                                          i32.add
                                                          local.get 6
                                                          i64.load
                                                          i64.store
                                                          local.get 2
                                                          i32.const 176
                                                          i32.add
                                                          local.get 1
                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                          local.get 2
                                                          i32.load8_u offset=176
                                                          i32.const 1
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.get 2
                                                          i32.const 193
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.get 2
                                                          i32.const 185
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 231
                                                          i32.add
                                                          local.tee 4
                                                          local.get 2
                                                          i32.const 200
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store align=1
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.tee 6
                                                          local.get 2
                                                          i32.const 223
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.tee 9
                                                          local.get 4
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=177 align=1
                                                          i64.store offset=208
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=215 align=1
                                                          i64.store offset=152
                                                          local.get 3
                                                          i32.const 3
                                                          i32.le_u
                                                          br_if 26 (;@1;)
                                                          local.get 5
                                                          local.get 2
                                                          i64.load offset=152
                                                          i64.store offset=72
                                                          local.get 5
                                                          i32.const 88
                                                          i32.add
                                                          local.get 9
                                                          i64.load
                                                          i64.store
                                                          local.get 5
                                                          i32.const 80
                                                          i32.add
                                                          local.get 6
                                                          i64.load
                                                          i64.store
                                                          local.get 2
                                                          i32.const 176
                                                          i32.add
                                                          local.get 1
                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                          local.get 2
                                                          i32.load8_u offset=176
                                                          i32.const 1
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.get 2
                                                          i32.const 193
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 208
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.get 2
                                                          i32.const 185
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 231
                                                          i32.add
                                                          local.tee 1
                                                          local.get 2
                                                          i32.const 200
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store align=1
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 8
                                                          i32.add
                                                          local.tee 4
                                                          local.get 2
                                                          i32.const 223
                                                          i32.add
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          i32.const 152
                                                          i32.add
                                                          i32.const 16
                                                          i32.add
                                                          local.tee 6
                                                          local.get 1
                                                          i64.load align=1
                                                          i64.store
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=177 align=1
                                                          i64.store offset=208
                                                          local.get 2
                                                          local.get 2
                                                          i64.load offset=215 align=1
                                                          i64.store offset=152
                                                          local.get 3
                                                          i32.const 4
                                                          i32.le_u
                                                          br_if 26 (;@1;)
                                                          local.get 5
                                                          local.get 2
                                                          i64.load offset=152
                                                          i64.store offset=96
                                                          local.get 5
                                                          i32.const 112
                                                          i32.add
                                                          local.get 6
                                                          i64.load
                                                          i64.store
                                                          local.get 5
                                                          i32.const 104
                                                          i32.add
                                                          local.get 4
                                                          i64.load
                                                          i64.store
                                                          i32.const 5
                                                          local.set 4
                                                          br 5 (;@22;)
                                                        end
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 6
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 9
                                                        local.get 4
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.eqz
                                                        br_if 25 (;@1;)
                                                        local.get 6
                                                        i64.load
                                                        local.set 7
                                                        local.get 2
                                                        i64.load offset=152
                                                        local.set 8
                                                        local.get 5
                                                        i32.const 16
                                                        i32.add
                                                        local.get 9
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 8
                                                        i32.add
                                                        local.get 7
                                                        i64.store
                                                        local.get 5
                                                        local.get 8
                                                        i64.store
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 6
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 9
                                                        local.get 4
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.const 1
                                                        i32.le_u
                                                        br_if 25 (;@1;)
                                                        local.get 5
                                                        local.get 2
                                                        i64.load offset=152
                                                        i64.store offset=24
                                                        local.get 5
                                                        i32.const 40
                                                        i32.add
                                                        local.get 9
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 32
                                                        i32.add
                                                        local.get 6
                                                        i64.load
                                                        i64.store
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 6
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 9
                                                        local.get 4
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.const 2
                                                        i32.le_u
                                                        br_if 25 (;@1;)
                                                        local.get 5
                                                        local.get 2
                                                        i64.load offset=152
                                                        i64.store offset=48
                                                        local.get 5
                                                        i32.const 64
                                                        i32.add
                                                        local.get 9
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 56
                                                        i32.add
                                                        local.get 6
                                                        i64.load
                                                        i64.store
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 6
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 9
                                                        local.get 4
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.const 3
                                                        i32.le_u
                                                        br_if 25 (;@1;)
                                                        local.get 5
                                                        local.get 2
                                                        i64.load offset=152
                                                        i64.store offset=72
                                                        local.get 5
                                                        i32.const 88
                                                        i32.add
                                                        local.get 9
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 80
                                                        i32.add
                                                        local.get 6
                                                        i64.load
                                                        i64.store
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 6
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 9
                                                        local.get 4
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.const 4
                                                        i32.le_u
                                                        br_if 25 (;@1;)
                                                        local.get 5
                                                        local.get 2
                                                        i64.load offset=152
                                                        i64.store offset=96
                                                        local.get 5
                                                        i32.const 112
                                                        i32.add
                                                        local.get 9
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 104
                                                        i32.add
                                                        local.get 6
                                                        i64.load
                                                        i64.store
                                                        local.get 2
                                                        i32.const 176
                                                        i32.add
                                                        local.get 1
                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                        local.get 2
                                                        i32.load8_u offset=176
                                                        i32.const 1
                                                        i32.eq
                                                        br_if 5 (;@21;)
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.get 2
                                                        i32.const 193
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 208
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.get 2
                                                        i32.const 185
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 231
                                                        i32.add
                                                        local.tee 1
                                                        local.get 2
                                                        i32.const 200
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store align=1
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 8
                                                        i32.add
                                                        local.tee 4
                                                        local.get 2
                                                        i32.const 223
                                                        i32.add
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        i32.const 152
                                                        i32.add
                                                        i32.const 16
                                                        i32.add
                                                        local.tee 6
                                                        local.get 1
                                                        i64.load align=1
                                                        i64.store
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=177 align=1
                                                        i64.store offset=208
                                                        local.get 2
                                                        local.get 2
                                                        i64.load offset=215 align=1
                                                        i64.store offset=152
                                                        local.get 3
                                                        i32.const 5
                                                        i32.le_u
                                                        br_if 25 (;@1;)
                                                        local.get 5
                                                        local.get 2
                                                        i64.load offset=152
                                                        i64.store offset=120
                                                        local.get 5
                                                        i32.const 136
                                                        i32.add
                                                        local.get 6
                                                        i64.load
                                                        i64.store
                                                        local.get 5
                                                        i32.const 128
                                                        i32.add
                                                        local.get 4
                                                        i64.load
                                                        i64.store
                                                        i32.const 6
                                                        local.set 4
                                                        br 4 (;@22;)
                                                      end
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.eqz
                                                      br_if 24 (;@1;)
                                                      local.get 6
                                                      i64.load
                                                      local.set 7
                                                      local.get 2
                                                      i64.load offset=152
                                                      local.set 8
                                                      local.get 5
                                                      i32.const 16
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 8
                                                      i32.add
                                                      local.get 7
                                                      i64.store
                                                      local.get 5
                                                      local.get 8
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 1
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=24
                                                      local.get 5
                                                      i32.const 40
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 32
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 2
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=48
                                                      local.get 5
                                                      i32.const 64
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 56
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 3
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=72
                                                      local.get 5
                                                      i32.const 88
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 80
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 4
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=96
                                                      local.get 5
                                                      i32.const 112
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 104
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 6
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 9
                                                      local.get 4
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 5
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=120
                                                      local.get 5
                                                      i32.const 136
                                                      i32.add
                                                      local.get 9
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 128
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 2
                                                      i32.const 176
                                                      i32.add
                                                      local.get 1
                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                      local.get 2
                                                      i32.load8_u offset=176
                                                      i32.const 1
                                                      i32.eq
                                                      br_if 4 (;@21;)
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.get 2
                                                      i32.const 193
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 208
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.get 2
                                                      i32.const 185
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 231
                                                      i32.add
                                                      local.tee 1
                                                      local.get 2
                                                      i32.const 200
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store align=1
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 8
                                                      i32.add
                                                      local.tee 4
                                                      local.get 2
                                                      i32.const 223
                                                      i32.add
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      i32.const 152
                                                      i32.add
                                                      i32.const 16
                                                      i32.add
                                                      local.tee 6
                                                      local.get 1
                                                      i64.load align=1
                                                      i64.store
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=177 align=1
                                                      i64.store offset=208
                                                      local.get 2
                                                      local.get 2
                                                      i64.load offset=215 align=1
                                                      i64.store offset=152
                                                      local.get 3
                                                      i32.const 6
                                                      i32.le_u
                                                      br_if 24 (;@1;)
                                                      local.get 5
                                                      local.get 2
                                                      i64.load offset=152
                                                      i64.store offset=144
                                                      local.get 5
                                                      i32.const 160
                                                      i32.add
                                                      local.get 6
                                                      i64.load
                                                      i64.store
                                                      local.get 5
                                                      i32.const 152
                                                      i32.add
                                                      local.get 4
                                                      i64.load
                                                      i64.store
                                                      i32.const 7
                                                      local.set 4
                                                      br 3 (;@22;)
                                                    end
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.eqz
                                                    br_if 23 (;@1;)
                                                    local.get 6
                                                    i64.load
                                                    local.set 7
                                                    local.get 2
                                                    i64.load offset=152
                                                    local.set 8
                                                    local.get 5
                                                    i32.const 16
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 8
                                                    i32.add
                                                    local.get 7
                                                    i64.store
                                                    local.get 5
                                                    local.get 8
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 1
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=24
                                                    local.get 5
                                                    i32.const 40
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 32
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 2
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=48
                                                    local.get 5
                                                    i32.const 64
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 56
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 3
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=72
                                                    local.get 5
                                                    i32.const 88
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 80
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 4
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=96
                                                    local.get 5
                                                    i32.const 112
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 104
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 5
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=120
                                                    local.get 5
                                                    i32.const 136
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 128
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 4
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 4
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 6
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=144
                                                    local.get 5
                                                    i32.const 160
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 152
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    local.get 2
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                    local.get 2
                                                    i32.load8_u offset=176
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 2
                                                    i32.const 193
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    i32.const 8
                                                    local.set 4
                                                    local.get 2
                                                    i32.const 208
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 2
                                                    i32.const 185
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 231
                                                    i32.add
                                                    local.tee 1
                                                    local.get 2
                                                    i32.const 200
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store align=1
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 6
                                                    local.get 2
                                                    i32.const 223
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    i32.const 152
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 9
                                                    local.get 1
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=177 align=1
                                                    i64.store offset=208
                                                    local.get 2
                                                    local.get 2
                                                    i64.load offset=215 align=1
                                                    i64.store offset=152
                                                    local.get 3
                                                    i32.const 7
                                                    i32.le_u
                                                    br_if 23 (;@1;)
                                                    local.get 5
                                                    local.get 2
                                                    i64.load offset=152
                                                    i64.store offset=168
                                                    local.get 5
                                                    i32.const 184
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 5
                                                    i32.const 176
                                                    i32.add
                                                    local.get 6
                                                    i64.load
                                                    i64.store
                                                    br 2 (;@22;)
                                                  end
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.eqz
                                                  br_if 22 (;@1;)
                                                  local.get 6
                                                  i64.load
                                                  local.set 7
                                                  local.get 2
                                                  i64.load offset=152
                                                  local.set 8
                                                  local.get 5
                                                  i32.const 16
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 8
                                                  i32.add
                                                  local.get 7
                                                  i64.store
                                                  local.get 5
                                                  local.get 8
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 1
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=24
                                                  local.get 5
                                                  i32.const 40
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 32
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 2
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=48
                                                  local.get 5
                                                  i32.const 64
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 56
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 3
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=72
                                                  local.get 5
                                                  i32.const 88
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 80
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 4
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=96
                                                  local.get 5
                                                  i32.const 112
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 104
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 5
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=120
                                                  local.get 5
                                                  i32.const 136
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 128
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 6
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=144
                                                  local.get 5
                                                  i32.const 160
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 152
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 185
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 4
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 4
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 7
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=168
                                                  local.get 5
                                                  i32.const 184
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 176
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  local.get 1
                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                  local.get 2
                                                  i32.load8_u offset=176
                                                  i32.const 1
                                                  i32.eq
                                                  br_if 2 (;@21;)
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.get 2
                                                  i32.const 193
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  i32.const 9
                                                  local.set 4
                                                  local.get 2
                                                  i32.const 208
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.get 2
                                                  i32.const 176
                                                  i32.add
                                                  i32.const 9
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 231
                                                  i32.add
                                                  local.tee 1
                                                  local.get 2
                                                  i32.const 200
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store align=1
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 8
                                                  i32.add
                                                  local.tee 6
                                                  local.get 2
                                                  i32.const 223
                                                  i32.add
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  i32.const 152
                                                  i32.add
                                                  i32.const 16
                                                  i32.add
                                                  local.tee 9
                                                  local.get 1
                                                  i64.load align=1
                                                  i64.store
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=177 align=1
                                                  i64.store offset=208
                                                  local.get 2
                                                  local.get 2
                                                  i64.load offset=215 align=1
                                                  i64.store offset=152
                                                  local.get 3
                                                  i32.const 8
                                                  i32.le_u
                                                  br_if 22 (;@1;)
                                                  local.get 5
                                                  local.get 2
                                                  i64.load offset=152
                                                  i64.store offset=192
                                                  local.get 5
                                                  i32.const 208
                                                  i32.add
                                                  local.get 9
                                                  i64.load
                                                  i64.store
                                                  local.get 5
                                                  i32.const 200
                                                  i32.add
                                                  local.get 6
                                                  i64.load
                                                  i64.store
                                                  br 1 (;@22;)
                                                end
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.eqz
                                                br_if 21 (;@1;)
                                                local.get 6
                                                i64.load
                                                local.set 7
                                                local.get 2
                                                i64.load offset=152
                                                local.set 8
                                                local.get 5
                                                i32.const 16
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 8
                                                i32.add
                                                local.get 7
                                                i64.store
                                                local.get 5
                                                local.get 8
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 1
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=24
                                                local.get 5
                                                i32.const 40
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 32
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 2
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=48
                                                local.get 5
                                                i32.const 64
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 56
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 3
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=72
                                                local.get 5
                                                i32.const 88
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 80
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 4
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=96
                                                local.get 5
                                                i32.const 112
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 104
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 5
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=120
                                                local.get 5
                                                i32.const 136
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 128
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 6
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=144
                                                local.get 5
                                                i32.const 160
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 152
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 7
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=168
                                                local.get 5
                                                i32.const 184
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 176
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 185
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 6
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 9
                                                local.get 4
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 8
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=192
                                                local.get 5
                                                i32.const 208
                                                i32.add
                                                local.get 9
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 200
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                local.get 1
                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h0ee53de005d4a081E
                                                local.get 2
                                                i32.load8_u offset=176
                                                i32.const 1
                                                i32.eq
                                                br_if 1 (;@21;)
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.get 2
                                                i32.const 193
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 208
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.get 2
                                                i32.const 176
                                                i32.add
                                                i32.const 9
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 231
                                                i32.add
                                                local.tee 1
                                                local.get 2
                                                i32.const 200
                                                i32.add
                                                i64.load align=1
                                                i64.store align=1
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 8
                                                i32.add
                                                local.tee 4
                                                local.get 2
                                                i32.const 223
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                i32.const 152
                                                i32.add
                                                i32.const 16
                                                i32.add
                                                local.tee 6
                                                local.get 1
                                                i64.load align=1
                                                i64.store
                                                local.get 2
                                                local.get 2
                                                i64.load offset=177 align=1
                                                i64.store offset=208
                                                local.get 2
                                                local.get 2
                                                i64.load offset=215 align=1
                                                i64.store offset=152
                                                local.get 3
                                                i32.const 9
                                                i32.le_u
                                                br_if 21 (;@1;)
                                                local.get 5
                                                local.get 2
                                                i64.load offset=152
                                                i64.store offset=216
                                                local.get 5
                                                i32.const 232
                                                i32.add
                                                local.get 6
                                                i64.load
                                                i64.store
                                                local.get 5
                                                i32.const 224
                                                i32.add
                                                local.get 4
                                                i64.load
                                                i64.store
                                                i32.const 10
                                                local.set 4
                                              end
                                              local.get 2
                                              i32.const 140
                                              i32.add
                                              local.get 5
                                              i32.store
                                              local.get 2
                                              local.get 3
                                              i32.store offset=136
                                              local.get 2
                                              local.get 4
                                              i32.store offset=132
                                              local.get 2
                                              i32.const 1
                                              i32.store offset=128
                                              local.get 2
                                              i32.const 128
                                              i32.add
                                              local.set 1
                                              br 17 (;@4;)
                                            end
                                            local.get 2
                                            i32.load8_u offset=177
                                            local.set 1
                                            local.get 2
                                            i32.load8_u offset=178
                                            local.set 3
                                            br 14 (;@6;)
                                          end
                                          local.get 1
                                          call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                          local.tee 1
                                          i32.const 255
                                          i32.and
                                          i32.const 1
                                          i32.eq
                                          br_if 16 (;@3;)
                                          local.get 1
                                          i32.const 1
                                          i32.and
                                          br_if 18 (;@1;)
                                          local.get 2
                                          i32.const 0
                                          i32.store8 offset=136
                                          local.get 2
                                          i32.const 0
                                          i32.store offset=128
                                          local.get 2
                                          i32.const 128
                                          i32.add
                                          local.set 1
                                          br 15 (;@4;)
                                        end
                                        block  ;; label = @19
                                          local.get 1
                                          call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                          local.tee 1
                                          i32.const 255
                                          i32.and
                                          i32.const 1
                                          i32.eq
                                          br_if 0 (;@19;)
                                          local.get 1
                                          i32.const 1
                                          i32.and
                                          br_if 18 (;@1;)
                                          local.get 2
                                          i32.const 1
                                          i32.store8 offset=136
                                          local.get 2
                                          i32.const 0
                                          i32.store offset=128
                                          local.get 2
                                          i32.const 128
                                          i32.add
                                          local.set 1
                                          br 15 (;@4;)
                                        end
                                        local.get 2
                                        i32.const 16
                                        i32.add
                                        local.get 1
                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
                                        local.get 0
                                        local.get 2
                                        i32.load16_u offset=16
                                        i32.store16 offset=1 align=1
                                        local.get 0
                                        i32.const 1
                                        i32.store8
                                        br 16 (;@2;)
                                      end
                                      block  ;; label = @18
                                        local.get 1
                                        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                        local.tee 1
                                        i32.const 255
                                        i32.and
                                        i32.const 1
                                        i32.eq
                                        br_if 0 (;@18;)
                                        local.get 1
                                        i32.const 1
                                        i32.and
                                        br_if 17 (;@1;)
                                        i32.const 0
                                        local.set 3
                                        block  ;; label = @19
                                          local.get 1
                                          i32.const 8
                                          i32.shr_u
                                          i32.const 255
                                          i32.and
                                          local.tee 1
                                          i32.eqz
                                          br_if 0 (;@19;)
                                          local.get 1
                                          i32.const 16
                                          i32.ne
                                          br_if 18 (;@1;)
                                          i32.const 1
                                          local.set 3
                                        end
                                        local.get 2
                                        i32.const 137
                                        i32.add
                                        local.get 3
                                        i32.store8
                                        local.get 2
                                        i32.const 2
                                        i32.store8 offset=136
                                        local.get 2
                                        i32.const 0
                                        i32.store offset=128
                                        local.get 2
                                        i32.const 128
                                        i32.add
                                        local.set 1
                                        br 14 (;@4;)
                                      end
                                      local.get 2
                                      i32.const 24
                                      i32.add
                                      local.get 1
                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
                                      local.get 0
                                      local.get 2
                                      i32.load16_u offset=24
                                      i32.store16 offset=1 align=1
                                      local.get 0
                                      i32.const 1
                                      i32.store8
                                      br 15 (;@2;)
                                    end
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 1
                                        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                        local.tee 3
                                        i32.const 255
                                        i32.and
                                        i32.const 1
                                        i32.eq
                                        br_if 0 (;@18;)
                                        local.get 3
                                        i32.const 1
                                        i32.and
                                        br_if 17 (;@1;)
                                        block  ;; label = @19
                                          local.get 1
                                          i32.const 20
                                          call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17ha4c30d7c817cf3eaE
                                          local.tee 7
                                          i32.wrap_i64
                                          i32.const 255
                                          i32.and
                                          i32.const 1
                                          i32.ne
                                          br_if 0 (;@19;)
                                          local.get 2
                                          i32.const 40
                                          i32.add
                                          local.get 7
                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h5521cd02bbf9bbb1E
                                          local.get 2
                                          i32.load8_u offset=41
                                          local.set 1
                                          local.get 2
                                          i32.load8_u offset=40
                                          local.set 3
                                          br 2 (;@17;)
                                        end
                                        local.get 7
                                        i64.const 1
                                        i64.and
                                        i64.eqz
                                        i32.eqz
                                        br_if 17 (;@1;)
                                        local.get 2
                                        i32.const 140
                                        i32.add
                                        local.get 7
                                        i64.const 32
                                        i64.shr_u
                                        i64.store32
                                        local.get 2
                                        i32.const 3
                                        i32.store8 offset=136
                                        local.get 2
                                        i32.const 0
                                        i32.store offset=128
                                        local.get 2
                                        i32.const 128
                                        i32.add
                                        local.set 1
                                        br 14 (;@4;)
                                      end
                                      local.get 2
                                      i32.const 32
                                      i32.add
                                      local.get 3
                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
                                    br 14 (;@2;)
                                  end
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 1
                                      call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
                                      local.tee 3
                                      i32.const 255
                                      i32.and
                                      i32.const 1
                                      i32.eq
                                      br_if 0 (;@17;)
                                      local.get 3
                                      i32.const 1
                                      i32.and
                                      br_if 16 (;@1;)
                                      i32.const 1
                                      local.set 5
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
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
                                                      br_table 7 (;@18;) 0 (;@25;) 1 (;@24;) 2 (;@23;) 3 (;@22;) 4 (;@21;) 5 (;@20;) 6 (;@19;) 24 (;@1;)
                                                    end
                                                    i32.const 2
                                                    local.set 5
                                                    br 6 (;@18;)
                                                  end
                                                  i32.const 3
                                                  local.set 5
                                                  br 5 (;@18;)
                                                end
                                                i32.const 4
                                                local.set 5
                                                br 4 (;@18;)
                                              end
                                              i32.const 5
                                              local.set 5
                                              br 3 (;@18;)
                                            end
                                            i32.const 6
                                            local.set 5
                                            br 2 (;@18;)
                                          end
                                          i32.const 7
                                          local.set 5
                                          br 1 (;@18;)
                                        end
                                        i32.const 8
                                        local.set 5
                                      end
                                      local.get 2
                                      i32.const 176
                                      i32.add
                                      local.get 1
                                      local.get 5
                                      call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E
                                      local.get 2
                                      i32.load8_u offset=176
                                      i32.const 1
                                      i32.ne
                                      br_if 12 (;@5;)
                                      local.get 2
                                      i32.load8_u offset=177
                                      local.set 1
                                      local.get 2
                                      i32.load8_u offset=178
                                      local.set 3
                                      br 1 (;@16;)
                                    end
                                    local.get 2
                                    i32.const 48
                                    i32.add
                                    local.get 3
                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
                                  br 13 (;@2;)
                                end
                                block  ;; label = @15
                                  local.get 1
                                  call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h267fed9dda9b6080E
                                  local.tee 1
                                  i32.const 255
                                  i32.and
                                  i32.const 1
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 1
                                  i32.const 1
                                  i32.and
                                  br_if 14 (;@1;)
                                  local.get 2
                                  i32.const 137
                                  i32.add
                                  local.get 1
                                  i32.const 8
                                  i32.shr_u
                                  i32.store8
                                  local.get 2
                                  i32.const 5
                                  i32.store8 offset=136
                                  local.get 2
                                  i32.const 0
                                  i32.store offset=128
                                  local.get 2
                                  i32.const 128
                                  i32.add
                                  local.set 1
                                  br 11 (;@4;)
                                end
                                local.get 2
                                i32.const 56
                                i32.add
                                local.get 1
                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h3598ecf6f2410cd0E
                                local.get 0
                                local.get 2
                                i32.load16_u offset=56
                                i32.store16 offset=1 align=1
                                local.get 0
                                i32.const 1
                                i32.store8
                                br 12 (;@2;)
                              end
                              block  ;; label = @14
                                local.get 1
                                call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h267fed9dda9b6080E
                                local.tee 1
                                i32.const 255
                                i32.and
                                i32.const 1
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 1
                                i32.const 1
                                i32.and
                                br_if 13 (;@1;)
                                local.get 2
                                i32.const 137
                                i32.add
                                local.get 1
                                i32.const 8
                                i32.shr_u
                                i32.store8
                                local.get 2
                                i32.const 6
                                i32.store8 offset=136
                                local.get 2
                                i32.const 0
                                i32.store offset=128
                                local.get 2
                                i32.const 128
                                i32.add
                                local.set 1
                                br 10 (;@4;)
                              end
                              local.get 2
                              i32.const 72
                              i32.add
                              local.get 1
                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h3598ecf6f2410cd0E
                              local.get 2
                              i32.const 64
                              i32.add
                              local.get 2
                              i32.load16_u offset=72
                              i32.const 8
                              i32.shl
                              i32.const 1
                              i32.or
                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
                              local.get 0
                              local.get 2
                              i32.load16_u offset=64
                              i32.store16 offset=1 align=1
                              local.get 0
                              i32.const 1
                              i32.store8
                              br 11 (;@2;)
                            end
                            block  ;; label = @13
                              local.get 1
                              call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h798148aec9978708E
                              local.tee 1
                              i32.const 255
                              i32.and
                              i32.const 1
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 1
                              i32.const 1
                              i32.and
                              br_if 12 (;@1;)
                              local.get 2
                              i32.const 138
                              i32.add
                              local.get 1
                              i32.const 16
                              i32.shr_u
                              i32.store16
                              local.get 2
                              i32.const 7
                              i32.store8 offset=136
                              local.get 2
                              i32.const 0
                              i32.store offset=128
                              local.get 2
                              i32.const 128
                              i32.add
                              local.set 1
                              br 9 (;@4;)
                            end
                            local.get 2
                            i32.const 80
                            i32.add
                            local.get 1
                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hb693a1cd083fe8e9E
                            local.get 0
                            local.get 2
                            i32.load16_u offset=80
                            i32.store16 offset=1 align=1
                            local.get 0
                            i32.const 1
                            i32.store8
                            br 10 (;@2;)
                          end
                          block  ;; label = @12
                            local.get 1
                            call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h798148aec9978708E
                            local.tee 1
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 1
                            i32.and
                            br_if 11 (;@1;)
                            local.get 2
                            i32.const 138
                            i32.add
                            local.get 1
                            i32.const 16
                            i32.shr_u
                            i32.store16
                            local.get 2
                            i32.const 8
                            i32.store8 offset=136
                            local.get 2
                            i32.const 0
                            i32.store offset=128
                            local.get 2
                            i32.const 128
                            i32.add
                            local.set 1
                            br 8 (;@4;)
                          end
                          local.get 2
                          i32.const 88
                          i32.add
                          local.get 1
                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hb693a1cd083fe8e9E
                          local.get 0
                          local.get 2
                          i32.load16_u offset=88
                          i32.store16 offset=1 align=1
                          local.get 0
                          i32.const 1
                          i32.store8
                          br 9 (;@2;)
                        end
                        block  ;; label = @11
                          local.get 1
                          call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h1f2a8ad6743b71daE
                          local.tee 7
                          i32.wrap_i64
                          i32.const 255
                          i32.and
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 7
                          i64.const 1
                          i64.and
                          i64.eqz
                          i32.eqz
                          br_if 10 (;@1;)
                          local.get 2
                          i32.const 140
                          i32.add
                          local.get 7
                          i64.const 32
                          i64.shr_u
                          i64.store32
                          local.get 2
                          i32.const 9
                          i32.store8 offset=136
                          local.get 2
                          i32.const 0
                          i32.store offset=128
                          local.get 2
                          i32.const 128
                          i32.add
                          local.set 1
                          br 7 (;@4;)
                        end
                        local.get 2
                        i32.const 96
                        i32.add
                        local.get 7
                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h448baf931d95396eE
                        local.get 0
                        local.get 2
                        i32.load16_u offset=96
                        i32.store16 offset=1 align=1
                        local.get 0
                        i32.const 1
                        i32.store8
                        br 8 (;@2;)
                      end
                      block  ;; label = @10
                        local.get 1
                        call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h1f2a8ad6743b71daE
                        local.tee 7
                        i32.wrap_i64
                        i32.const 255
                        i32.and
                        i32.const 1
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 7
                        i64.const 1
                        i64.and
                        i64.eqz
                        i32.eqz
                        br_if 9 (;@1;)
                        local.get 2
                        i32.const 140
                        i32.add
                        local.get 7
                        i64.const 32
                        i64.shr_u
                        i64.store32
                        local.get 2
                        i32.const 10
                        i32.store8 offset=136
                        local.get 2
                        i32.const 0
                        i32.store offset=128
                        local.get 2
                        i32.const 128
                        i32.add
                        local.set 1
                        br 6 (;@4;)
                      end
                      local.get 2
                      i32.const 104
                      i32.add
                      local.get 7
                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h448baf931d95396eE
                      local.get 0
                      local.get 2
                      i32.load16_u offset=104
                      i32.store16 offset=1 align=1
                      local.get 0
                      i32.const 1
                      i32.store8
                      br 7 (;@2;)
                    end
                    local.get 2
                    i32.const 176
                    i32.add
                    local.get 1
                    call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h92db93bb0c278829E
                    block  ;; label = @9
                      local.get 2
                      i32.load8_u offset=176
                      i32.const 1
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 2
                      i32.const 144
                      i32.add
                      local.get 2
                      i32.const 184
                      i32.add
                      i64.load
                      i64.store
                      local.get 2
                      i32.const 11
                      i32.store8 offset=136
                      local.get 2
                      i32.const 0
                      i32.store offset=128
                      local.get 2
                      i32.const 128
                      i32.add
                      local.set 1
                      br 5 (;@4;)
                    end
                    local.get 0
                    local.get 2
                    i32.load16_u offset=177 align=1
                    i32.store16 offset=1 align=1
                    local.get 0
                    i32.const 1
                    i32.store8
                    br 6 (;@2;)
                  end
                  local.get 2
                  i32.const 176
                  i32.add
                  local.get 1
                  call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h92db93bb0c278829E
                  block  ;; label = @8
                    local.get 2
                    i32.load8_u offset=176
                    i32.const 1
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 2
                    i32.const 144
                    i32.add
                    local.get 2
                    i32.const 184
                    i32.add
                    i64.load
                    i64.store
                    local.get 2
                    i32.const 12
                    i32.store8 offset=136
                    local.get 2
                    i32.const 0
                    i32.store offset=128
                    local.get 2
                    i32.const 128
                    i32.add
                    local.set 1
                    br 4 (;@4;)
                  end
                  local.get 0
                  local.get 2
                  i32.load16_u offset=177 align=1
                  i32.store16 offset=1 align=1
                  local.get 0
                  i32.const 1
                  i32.store8
                  br 5 (;@2;)
                end
                local.get 2
                i32.const 112
                i32.add
                local.get 3
                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
                local.get 2
                i32.load8_u offset=113
                local.set 3
                local.get 2
                i32.load8_u offset=112
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
              br 3 (;@2;)
            end
            local.get 2
            i32.const 144
            i32.add
            local.get 2
            i32.const 184
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 4
            i32.store8 offset=136
            local.get 2
            i32.const 0
            i32.store offset=128
            local.get 2
            i32.const 128
            i32.add
            local.set 1
          end
          local.get 0
          i32.const 0
          i32.store8
          local.get 0
          i32.const 8
          i32.add
          local.get 1
          i64.load
          i64.store
          local.get 0
          i32.const 24
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 0
          i32.const 16
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i64.load
          i64.store
          br 1 (;@2;)
        end
        local.get 2
        i32.const 8
        i32.add
        local.get 1
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
        local.get 0
        local.get 2
        i32.load16_u offset=8
        i32.store16 offset=1 align=1
        local.get 0
        i32.const 1
        i32.store8
      end
      local.get 2
      i32.const 240
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $load_addr (type 3)
    (local i32 i32)
    i32.const 0
    i32.const 20
    call $_ZN13svm_sdk_alloc16svm_static_alloc17hd7c2b9494a084826E
    local.tee 0
    call $_ZN15svm_sdk_storage3ext11svm_load16017hb1b13333e9a2c166E
    i32.const 21
    call $_ZN13svm_sdk_alloc16svm_static_alloc17hd7c2b9494a084826E
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
    call $_ZN12svm_sdk_host3ext18svm_set_returndata17h64ba12fb97dce274E)
  (func $svm_fund (type 3))
  (func $_ZN15svm_abi_decoder6cursor6Cursor4peek17h9ae4de5b86ca23f8E (type 2) (param i32 i32)
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE (type 2) (param i32 i32)
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E (type 0) (param i32) (result i32)
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
    call $_ZN15svm_abi_decoder6cursor6Cursor4peek17h9ae4de5b86ca23f8E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17ha4c30d7c817cf3eaE (type 4) (param i32 i32) (result i64)
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h5521cd02bbf9bbb1E (type 5) (param i32 i64)
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E (type 6) (param i32 i32 i32)
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
                            call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17ha4c30d7c817cf3eaE
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
                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h5521cd02bbf9bbb1E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h267fed9dda9b6080E (type 0) (param i32) (result i32)
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
        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
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
          call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E
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
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h3598ecf6f2410cd0E (type 2) (param i32 i32)
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h798148aec9978708E (type 0) (param i32) (result i32)
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
              call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
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
            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hb693a1cd083fe8e9E (type 2) (param i32 i32)
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h1f2a8ad6743b71daE (type 7) (param i32) (result i64)
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
                  call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
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
                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h448baf931d95396eE (type 5) (param i32 i64)
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h92db93bb0c278829E (type 2) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h15940ba508a181e8E
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
          call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17h86a8b8c79229aa54E
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
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h889ddf23de5ba8caE
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
  (export "svm_alloc" (func $svm_alloc))
  (export "initialize" (func $initialize))
  (export "store_addr" (func $store_addr))
  (export "load_addr" (func $load_addr))
  (export "svm_fund" (func $svm_fund))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
