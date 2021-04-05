(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32) (result i64)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32) (result i64)))
  (type (;9;) (func (param i32 i64)))
  (type (;10;) (func (param i64) (result i32)))
  (import "svm" "svm_calldata_offset" (func $_ZN12svm_sdk_host3ext19svm_calldata_offset17h07d24f45c5ac567fE (type 0)))
  (import "svm" "svm_calldata_len" (func $_ZN12svm_sdk_host3ext16svm_calldata_len17h6d77304c58f043adE (type 0)))
  (import "svm" "svm_set_returndata" (func $_ZN12svm_sdk_host3ext18svm_set_returndata17hc3f114854661f072E (type 1)))
  (import "svm" "svm_allocate" (func $_ZN13svm_sdk_alloc12svm_allocate17h12629aa7a0722256E (type 2)))
  (import "svm" "svm_store160" (func $_ZN15svm_sdk_storage3ext12svm_store16017hcc5844b34ea651e4E (type 1)))
  (import "svm" "svm_load160" (func $_ZN15svm_sdk_storage3ext11svm_load16017hb02d905ebf79dc3bE (type 1)))
  (func $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$12get_calldata17h9c3a1c3f928c46f6E (type 3) (param i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1049484
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1049484
    end
    call $_ZN12svm_sdk_host3ext19svm_calldata_offset17h07d24f45c5ac567fE
    local.set 1
    local.get 0
    call $_ZN12svm_sdk_host3ext16svm_calldata_len17h6d77304c58f043adE
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$14set_returndata17h50222e5da971306cE (type 1) (param i32 i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1049484
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1049484
    end
    local.get 0
    local.get 1
    call $_ZN12svm_sdk_host3ext18svm_set_returndata17hc3f114854661f072E)
  (func $svm_alloc (type 2) (param i32) (result i32)
    local.get 0
    call $_ZN13svm_sdk_alloc12svm_allocate17h12629aa7a0722256E)
  (func $initialize (type 4)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 8
    i32.add
    call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$12get_calldata17h9c3a1c3f928c46f6E
    local.get 0
    i32.const 16
    i32.add
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17h5c5f0a792dfa361bE
    local.get 0
    i32.const 16
    i32.add
    call $_ZN15svm_abi_encoder5types4unit110_$LT$impl$u20$svm_abi_encoder..traits..Encoder$LT$svm_sdk_std..vec..Vec$LT$u8$GT$$GT$$u20$for$u20$$LP$$RP$$GT$6encode17hbc07c9d4026a86d9E
    local.get 0
    i32.load offset=24
    local.get 0
    i32.load offset=16
    call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$14set_returndata17h50222e5da971306cE
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17h5c5f0a792dfa361bE (type 3) (param i32)
    local.get 0
    i32.const 10000
    call $_ZN13svm_sdk_alloc12svm_allocate17h12629aa7a0722256E
    i32.store offset=8
    local.get 0
    i64.const 42949672960000
    i64.store align=4)
  (func $_ZN15svm_abi_encoder5types4unit110_$LT$impl$u20$svm_abi_encoder..traits..Encoder$LT$svm_sdk_std..vec..Vec$LT$u8$GT$$GT$$u20$for$u20$$LP$$RP$$GT$6encode17hbc07c9d4026a86d9E (type 3) (param i32)
    local.get 0
    i32.const 48
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE)
  (func $store_addr (type 4)
    (local i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 8
    i32.add
    call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$12get_calldata17h9c3a1c3f928c46f6E
    local.get 0
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 0
    i32.load offset=12
    local.tee 2
    i32.store offset=24
    local.get 0
    i32.const 0
    i32.store offset=20
    local.get 0
    local.get 1
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 32
            i32.add
            local.get 0
            i32.const 16
            i32.add
            local.get 0
            i32.const 16
            i32.add
            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
            local.get 0
            i32.load8_u offset=32
            i32.const 1
            i32.eq
            br_if 2 (;@2;)
            local.get 0
            i32.load offset=40
            br_table 1 (;@3;) 3 (;@1;) 0 (;@4;) 3 (;@1;)
          end
          i32.const 1049248
          i32.const 14
          i32.const 1049264
          call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
          unreachable
        end
        local.get 0
        i32.const 48
        i32.add
        i32.load8_u
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 0
        i32.const 52
        i32.add
        i32.load
        i32.const 0
        call $_ZN15svm_sdk_storage3ext12svm_store16017hcc5844b34ea651e4E
        local.get 0
        i32.const 32
        i32.add
        call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17h5c5f0a792dfa361bE
        local.get 0
        i32.const 32
        i32.add
        call $_ZN15svm_abi_encoder5types4unit110_$LT$impl$u20$svm_abi_encoder..traits..Encoder$LT$svm_sdk_std..vec..Vec$LT$u8$GT$$GT$$u20$for$u20$$LP$$RP$$GT$6encode17hbc07c9d4026a86d9E
        local.get 0
        i32.load offset=40
        local.get 0
        i32.load offset=32
        call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$14set_returndata17h50222e5da971306cE
        local.get 0
        i32.const 64
        i32.add
        global.set 0
        return
      end
      unreachable
      unreachable
    end
    i32.const 1049343
    i32.const 40
    i32.const 1049384
    call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
    unreachable)
  (func $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 800
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=4
        local.get 2
        i32.load offset=8
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 1
        i32.store16
        br 1 (;@1;)
      end
      local.get 3
      i32.const 640
      i32.add
      local.get 2
      call $_ZN15svm_abi_decoder6cursor6Cursor4peek17h67be1e4f303ac267E
      local.get 3
      i32.load8_u offset=641
      i32.const 2
      local.get 3
      i32.load8_u offset=640
      i32.const 1
      i32.and
      local.tee 4
      select
      i32.const 255
      i32.and
      i32.const 8
      i32.shl
      local.get 4
      i32.const 1
      i32.xor
      i32.or
      local.set 5
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
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      block  ;; label = @26
                                                        block  ;; label = @27
                                                          block  ;; label = @28
                                                            block  ;; label = @29
                                                              block  ;; label = @30
                                                                block  ;; label = @31
                                                                  block  ;; label = @32
                                                                    block  ;; label = @33
                                                                      block  ;; label = @34
                                                                        block  ;; label = @35
                                                                          block  ;; label = @36
                                                                            block  ;; label = @37
                                                                              block  ;; label = @38
                                                                                block  ;; label = @39
                                                                                  block  ;; label = @40
                                                                                    block  ;; label = @41
                                                                                      block  ;; label = @42
                                                                                        block  ;; label = @43
                                                                                          block  ;; label = @44
                                                                                            block  ;; label = @45
                                                                                              block  ;; label = @46
                                                                                                block  ;; label = @47
                                                                                                  block  ;; label = @48
                                                                                                    block  ;; label = @49
                                                                                                      block  ;; label = @50
                                                                                                        block  ;; label = @51
                                                                                                          block  ;; label = @52
                                                                                                            block  ;; label = @53
                                                                                                              block  ;; label = @54
                                                                                                                block  ;; label = @55
                                                                                                                  block  ;; label = @56
                                                                                                                    block  ;; label = @57
                                                                                                                      block  ;; label = @58
                                                                                                                        block  ;; label = @59
                                                                                                                          block  ;; label = @60
                                                                                                                            block  ;; label = @61
                                                                                                                              block  ;; label = @62
                                                                                                                                block  ;; label = @63
                                                                                                                                  block  ;; label = @64
                                                                                                                                    block  ;; label = @65
                                                                                                                                      block  ;; label = @66
                                                                                                                                        block  ;; label = @67
                                                                                                                                          local.get 4
                                                                                                                                          i32.eqz
                                                                                                                                          br_if 0 (;@67;)
                                                                                                                                          block  ;; label = @68
                                                                                                                                            local.get 5
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                            i32.const 255
                                                                                                                                            i32.and
                                                                                                                                            br_table 5 (;@63;) 7 (;@61;) 8 (;@60;) 12 (;@56;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 5 (;@63;) 7 (;@61;) 9 (;@59;) 12 (;@56;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 3 (;@65;) 7 (;@61;) 10 (;@58;) 12 (;@56;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 4 (;@64;) 7 (;@61;) 10 (;@58;) 12 (;@56;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 6 (;@62;) 7 (;@61;) 11 (;@57;) 13 (;@55;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 7 (;@61;) 11 (;@57;) 13 (;@55;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 7 (;@61;) 0 (;@68;) 13 (;@55;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 0 (;@68;) 7 (;@61;) 0 (;@68;) 13 (;@55;) 14 (;@54;) 15 (;@53;) 2 (;@66;) 0 (;@68;)
                                                                                                                                          end
                                                                                                                                          i32.const 1049343
                                                                                                                                          i32.const 40
                                                                                                                                          i32.const 1048680
                                                                                                                                          call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
                                                                                                                                          unreachable
                                                                                                                                        end
                                                                                                                                        local.get 3
                                                                                                                                        local.get 5
                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                                        local.get 0
                                                                                                                                        local.get 3
                                                                                                                                        i32.load16_u
                                                                                                                                        i32.store16 offset=1 align=1
                                                                                                                                        local.get 0
                                                                                                                                        i32.const 1
                                                                                                                                        i32.store8
                                                                                                                                        br 65 (;@1;)
                                                                                                                                      end
                                                                                                                                      block  ;; label = @66
                                                                                                                                        local.get 2
                                                                                                                                        i32.load offset=4
                                                                                                                                        local.get 2
                                                                                                                                        i32.load offset=8
                                                                                                                                        i32.lt_u
                                                                                                                                        br_if 0 (;@66;)
                                                                                                                                        i32.const 0
                                                                                                                                        local.set 2
                                                                                                                                        br 64 (;@2;)
                                                                                                                                      end
                                                                                                                                      local.get 2
                                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                      local.tee 4
                                                                                                                                      i32.const 255
                                                                                                                                      i32.and
                                                                                                                                      i32.const 1
                                                                                                                                      i32.eq
                                                                                                                                      br_if 13 (;@52;)
                                                                                                                                      local.get 3
                                                                                                                                      local.get 4
                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                      i32.const 255
                                                                                                                                      i32.and
                                                                                                                                      local.tee 4
                                                                                                                                      i32.const 24
                                                                                                                                      i32.mul
                                                                                                                                      call $_ZN13svm_sdk_alloc12svm_allocate17h12629aa7a0722256E
                                                                                                                                      i32.store offset=704
                                                                                                                                      local.get 3
                                                                                                                                      local.get 4
                                                                                                                                      i32.store offset=700
                                                                                                                                      local.get 3
                                                                                                                                      i32.const 0
                                                                                                                                      i32.store offset=696
                                                                                                                                      block  ;; label = @66
                                                                                                                                        block  ;; label = @67
                                                                                                                                          block  ;; label = @68
                                                                                                                                            block  ;; label = @69
                                                                                                                                              block  ;; label = @70
                                                                                                                                                block  ;; label = @71
                                                                                                                                                  local.get 4
                                                                                                                                                  i32.const -6
                                                                                                                                                  i32.add
                                                                                                                                                  br_table 4 (;@67;) 1 (;@70;) 0 (;@71;)
                                                                                                                                                end
                                                                                                                                                block  ;; label = @71
                                                                                                                                                  block  ;; label = @72
                                                                                                                                                    block  ;; label = @73
                                                                                                                                                      block  ;; label = @74
                                                                                                                                                        block  ;; label = @75
                                                                                                                                                          block  ;; label = @76
                                                                                                                                                            block  ;; label = @77
                                                                                                                                                              block  ;; label = @78
                                                                                                                                                                local.get 4
                                                                                                                                                                i32.const -22
                                                                                                                                                                i32.add
                                                                                                                                                                br_table 1 (;@77;) 9 (;@69;) 0 (;@78;)
                                                                                                                                                              end
                                                                                                                                                              block  ;; label = @78
                                                                                                                                                                local.get 4
                                                                                                                                                                i32.const -38
                                                                                                                                                                i32.add
                                                                                                                                                                br_table 2 (;@76;) 10 (;@68;) 0 (;@78;)
                                                                                                                                                              end
                                                                                                                                                              local.get 4
                                                                                                                                                              i32.const 54
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 2 (;@75;)
                                                                                                                                                              local.get 4
                                                                                                                                                              i32.const 70
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 3 (;@74;)
                                                                                                                                                              local.get 4
                                                                                                                                                              i32.const 86
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 4 (;@73;)
                                                                                                                                                              local.get 4
                                                                                                                                                              i32.const 102
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 5 (;@72;)
                                                                                                                                                              local.get 4
                                                                                                                                                              i32.const 118
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 6 (;@71;)
                                                                                                                                                              i32.const 1049343
                                                                                                                                                              i32.const 40
                                                                                                                                                              i32.const 1048696
                                                                                                                                                              call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
                                                                                                                                                              unreachable
                                                                                                                                                            end
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 736
                                                                                                                                                            i32.add
                                                                                                                                                            local.get 1
                                                                                                                                                            local.get 2
                                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                            block  ;; label = @77
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.load8_u offset=736
                                                                                                                                                              i32.const 1
                                                                                                                                                              i32.eq
                                                                                                                                                              br_if 0 (;@77;)
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 768
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 24
                                                                                                                                                              i32.add
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 736
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 24
                                                                                                                                                              i32.add
                                                                                                                                                              i64.load
                                                                                                                                                              i64.store
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 768
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 16
                                                                                                                                                              i32.add
                                                                                                                                                              local.tee 2
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 736
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 16
                                                                                                                                                              i32.add
                                                                                                                                                              i64.load
                                                                                                                                                              i64.store
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 768
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 8
                                                                                                                                                              i32.add
                                                                                                                                                              local.tee 4
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 736
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 8
                                                                                                                                                              i32.add
                                                                                                                                                              i64.load
                                                                                                                                                              i64.store
                                                                                                                                                              local.get 3
                                                                                                                                                              local.get 3
                                                                                                                                                              i64.load offset=736
                                                                                                                                                              i64.store offset=768
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 712
                                                                                                                                                              i32.add
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 768
                                                                                                                                                              i32.add
                                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                              local.get 2
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 712
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 16
                                                                                                                                                              i32.add
                                                                                                                                                              i64.load
                                                                                                                                                              i64.store
                                                                                                                                                              local.get 4
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 712
                                                                                                                                                              i32.add
                                                                                                                                                              i32.const 8
                                                                                                                                                              i32.add
                                                                                                                                                              i64.load
                                                                                                                                                              i64.store
                                                                                                                                                              local.get 3
                                                                                                                                                              local.get 3
                                                                                                                                                              i64.load offset=712
                                                                                                                                                              i64.store offset=768
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 696
                                                                                                                                                              i32.add
                                                                                                                                                              local.get 3
                                                                                                                                                              i32.const 768
                                                                                                                                                              i32.add
                                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                              br 10 (;@67;)
                                                                                                                                                            end
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 200
                                                                                                                                                            i32.add
                                                                                                                                                            i32.const 1
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.load8_u offset=737
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.load8_u offset=738
                                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.load8_u offset=201
                                                                                                                                                            local.set 4
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.load8_u offset=200
                                                                                                                                                            local.set 2
                                                                                                                                                            br 74 (;@2;)
                                                                                                                                                          end
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 1
                                                                                                                                                          local.get 2
                                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=736
                                                                                                                                                          i32.const 1
                                                                                                                                                          i32.eq
                                                                                                                                                          br_if 27 (;@48;)
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 24
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 6
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 24
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 7
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 4
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 8
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 5
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 9
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          local.get 3
                                                                                                                                                          i64.load offset=736
                                                                                                                                                          i64.store offset=768
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                          local.get 4
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 10
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 5
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 11
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          local.get 3
                                                                                                                                                          i64.load offset=712
                                                                                                                                                          i64.store offset=768
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 696
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 1
                                                                                                                                                          local.get 2
                                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                          block  ;; label = @76
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.load8_u offset=736
                                                                                                                                                            i32.const 1
                                                                                                                                                            i32.eq
                                                                                                                                                            br_if 0 (;@76;)
                                                                                                                                                            local.get 6
                                                                                                                                                            local.get 7
                                                                                                                                                            i64.load
                                                                                                                                                            i64.store
                                                                                                                                                            local.get 4
                                                                                                                                                            local.get 8
                                                                                                                                                            i64.load
                                                                                                                                                            i64.store
                                                                                                                                                            local.get 5
                                                                                                                                                            local.get 9
                                                                                                                                                            i64.load
                                                                                                                                                            i64.store
                                                                                                                                                            local.get 3
                                                                                                                                                            local.get 3
                                                                                                                                                            i64.load offset=736
                                                                                                                                                            i64.store offset=768
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 712
                                                                                                                                                            i32.add
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 768
                                                                                                                                                            i32.add
                                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                            local.get 4
                                                                                                                                                            local.get 10
                                                                                                                                                            i64.load
                                                                                                                                                            i64.store
                                                                                                                                                            local.get 5
                                                                                                                                                            local.get 11
                                                                                                                                                            i64.load
                                                                                                                                                            i64.store
                                                                                                                                                            local.get 3
                                                                                                                                                            local.get 3
                                                                                                                                                            i64.load offset=712
                                                                                                                                                            i64.store offset=768
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 696
                                                                                                                                                            i32.add
                                                                                                                                                            local.get 3
                                                                                                                                                            i32.const 768
                                                                                                                                                            i32.add
                                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                            br 9 (;@67;)
                                                                                                                                                          end
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 216
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 1
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=737
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=738
                                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=217
                                                                                                                                                          local.set 4
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=216
                                                                                                                                                          local.set 2
                                                                                                                                                          br 73 (;@2;)
                                                                                                                                                        end
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 1
                                                                                                                                                        local.get 2
                                                                                                                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=736
                                                                                                                                                        i32.const 1
                                                                                                                                                        i32.eq
                                                                                                                                                        br_if 27 (;@47;)
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 24
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 6
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 24
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 7
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 16
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 4
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 16
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 8
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 8
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 5
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 8
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 9
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=736
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 712
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                        local.get 4
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 712
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 16
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 10
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 5
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 712
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 8
                                                                                                                                                        i32.add
                                                                                                                                                        local.tee 11
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=712
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 696
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 1
                                                                                                                                                        local.get 2
                                                                                                                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=736
                                                                                                                                                        i32.const 1
                                                                                                                                                        i32.eq
                                                                                                                                                        br_if 28 (;@46;)
                                                                                                                                                        local.get 6
                                                                                                                                                        local.get 7
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 4
                                                                                                                                                        local.get 8
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 5
                                                                                                                                                        local.get 9
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=736
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 712
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                        local.get 4
                                                                                                                                                        local.get 10
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 5
                                                                                                                                                        local.get 11
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=712
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 696
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 736
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 1
                                                                                                                                                        local.get 2
                                                                                                                                                        call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                        block  ;; label = @75
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.load8_u offset=736
                                                                                                                                                          i32.const 1
                                                                                                                                                          i32.eq
                                                                                                                                                          br_if 0 (;@75;)
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 24
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 24
                                                                                                                                                          i32.add
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 2
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          local.tee 4
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 736
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          local.get 3
                                                                                                                                                          i64.load offset=736
                                                                                                                                                          i64.store offset=768
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                          local.get 2
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 16
                                                                                                                                                          i32.add
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 4
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 712
                                                                                                                                                          i32.add
                                                                                                                                                          i32.const 8
                                                                                                                                                          i32.add
                                                                                                                                                          i64.load
                                                                                                                                                          i64.store
                                                                                                                                                          local.get 3
                                                                                                                                                          local.get 3
                                                                                                                                                          i64.load offset=712
                                                                                                                                                          i64.store offset=768
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 696
                                                                                                                                                          i32.add
                                                                                                                                                          local.get 3
                                                                                                                                                          i32.const 768
                                                                                                                                                          i32.add
                                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                          br 8 (;@67;)
                                                                                                                                                        end
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 240
                                                                                                                                                        i32.add
                                                                                                                                                        i32.const 1
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=737
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=738
                                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=241
                                                                                                                                                        local.set 4
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=240
                                                                                                                                                        local.set 2
                                                                                                                                                        br 72 (;@2;)
                                                                                                                                                      end
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 1
                                                                                                                                                      local.get 2
                                                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=736
                                                                                                                                                      i32.const 1
                                                                                                                                                      i32.eq
                                                                                                                                                      br_if 28 (;@45;)
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 6
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 7
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 8
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 5
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 9
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=736
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                      local.get 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 10
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 5
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 11
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=712
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 696
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 1
                                                                                                                                                      local.get 2
                                                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=736
                                                                                                                                                      i32.const 1
                                                                                                                                                      i32.eq
                                                                                                                                                      br_if 29 (;@44;)
                                                                                                                                                      local.get 6
                                                                                                                                                      local.get 7
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 4
                                                                                                                                                      local.get 8
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 5
                                                                                                                                                      local.get 9
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=736
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                      local.get 4
                                                                                                                                                      local.get 10
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 5
                                                                                                                                                      local.get 11
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=712
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 696
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 1
                                                                                                                                                      local.get 2
                                                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=736
                                                                                                                                                      i32.const 1
                                                                                                                                                      i32.eq
                                                                                                                                                      br_if 30 (;@43;)
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 6
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 7
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 8
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 5
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 9
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=736
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                      local.get 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 10
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 5
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 11
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=712
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 696
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 1
                                                                                                                                                      local.get 2
                                                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                      block  ;; label = @74
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.load8_u offset=736
                                                                                                                                                        i32.const 1
                                                                                                                                                        i32.eq
                                                                                                                                                        br_if 0 (;@74;)
                                                                                                                                                        local.get 6
                                                                                                                                                        local.get 7
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 4
                                                                                                                                                        local.get 8
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 5
                                                                                                                                                        local.get 9
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=736
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 712
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                        local.get 4
                                                                                                                                                        local.get 10
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 5
                                                                                                                                                        local.get 11
                                                                                                                                                        i64.load
                                                                                                                                                        i64.store
                                                                                                                                                        local.get 3
                                                                                                                                                        local.get 3
                                                                                                                                                        i64.load offset=712
                                                                                                                                                        i64.store offset=768
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 696
                                                                                                                                                        i32.add
                                                                                                                                                        local.get 3
                                                                                                                                                        i32.const 768
                                                                                                                                                        i32.add
                                                                                                                                                        call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                        br 7 (;@67;)
                                                                                                                                                      end
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 272
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 1
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=737
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=738
                                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=273
                                                                                                                                                      local.set 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=272
                                                                                                                                                      local.set 2
                                                                                                                                                      br 71 (;@2;)
                                                                                                                                                    end
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 1
                                                                                                                                                    local.get 2
                                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=736
                                                                                                                                                    i32.const 1
                                                                                                                                                    i32.eq
                                                                                                                                                    br_if 30 (;@42;)
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 24
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 6
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 24
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 7
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 4
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 8
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 5
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 9
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=736
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 10
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 11
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=712
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 696
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 1
                                                                                                                                                    local.get 2
                                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=736
                                                                                                                                                    i32.const 1
                                                                                                                                                    i32.eq
                                                                                                                                                    br_if 31 (;@41;)
                                                                                                                                                    local.get 6
                                                                                                                                                    local.get 7
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 8
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 9
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=736
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 10
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 11
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=712
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 696
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 1
                                                                                                                                                    local.get 2
                                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=736
                                                                                                                                                    i32.const 1
                                                                                                                                                    i32.eq
                                                                                                                                                    br_if 32 (;@40;)
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 24
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 6
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 24
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 7
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 4
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 8
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 5
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 9
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=736
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 16
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 10
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 8
                                                                                                                                                    i32.add
                                                                                                                                                    local.tee 11
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=712
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 696
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 1
                                                                                                                                                    local.get 2
                                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=736
                                                                                                                                                    i32.const 1
                                                                                                                                                    i32.eq
                                                                                                                                                    br_if 33 (;@39;)
                                                                                                                                                    local.get 6
                                                                                                                                                    local.get 7
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 8
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 9
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=736
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 10
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 11
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=712
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 696
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 736
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 1
                                                                                                                                                    local.get 2
                                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                    block  ;; label = @73
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.load8_u offset=736
                                                                                                                                                      i32.const 1
                                                                                                                                                      i32.eq
                                                                                                                                                      br_if 0 (;@73;)
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 24
                                                                                                                                                      i32.add
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 2
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      local.tee 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 736
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=736
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                      local.get 2
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 16
                                                                                                                                                      i32.add
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 4
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 712
                                                                                                                                                      i32.add
                                                                                                                                                      i32.const 8
                                                                                                                                                      i32.add
                                                                                                                                                      i64.load
                                                                                                                                                      i64.store
                                                                                                                                                      local.get 3
                                                                                                                                                      local.get 3
                                                                                                                                                      i64.load offset=712
                                                                                                                                                      i64.store offset=768
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 696
                                                                                                                                                      i32.add
                                                                                                                                                      local.get 3
                                                                                                                                                      i32.const 768
                                                                                                                                                      i32.add
                                                                                                                                                      call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                      br 6 (;@67;)
                                                                                                                                                    end
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 312
                                                                                                                                                    i32.add
                                                                                                                                                    i32.const 1
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=737
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=738
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=313
                                                                                                                                                    local.set 4
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=312
                                                                                                                                                    local.set 2
                                                                                                                                                    br 70 (;@2;)
                                                                                                                                                  end
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 33 (;@38;)
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 6
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 7
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 8
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 9
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 10
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 11
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 34 (;@37;)
                                                                                                                                                  local.get 6
                                                                                                                                                  local.get 7
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 8
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 9
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 10
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 11
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 35 (;@36;)
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 6
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 7
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 8
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 9
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 10
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 11
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 36 (;@35;)
                                                                                                                                                  local.get 6
                                                                                                                                                  local.get 7
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 8
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 9
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 10
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 11
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 37 (;@34;)
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 6
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 7
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 8
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 9
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 10
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 5
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 11
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 1
                                                                                                                                                  local.get 2
                                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                  block  ;; label = @72
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.load8_u offset=736
                                                                                                                                                    i32.const 1
                                                                                                                                                    i32.eq
                                                                                                                                                    br_if 0 (;@72;)
                                                                                                                                                    local.get 6
                                                                                                                                                    local.get 7
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 8
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 9
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=736
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 712
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                    local.get 4
                                                                                                                                                    local.get 10
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 5
                                                                                                                                                    local.get 11
                                                                                                                                                    i64.load
                                                                                                                                                    i64.store
                                                                                                                                                    local.get 3
                                                                                                                                                    local.get 3
                                                                                                                                                    i64.load offset=712
                                                                                                                                                    i64.store offset=768
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 696
                                                                                                                                                    i32.add
                                                                                                                                                    local.get 3
                                                                                                                                                    i32.const 768
                                                                                                                                                    i32.add
                                                                                                                                                    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                    br 5 (;@67;)
                                                                                                                                                  end
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 360
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 1
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=737
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=738
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=361
                                                                                                                                                  local.set 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=360
                                                                                                                                                  local.set 2
                                                                                                                                                  br 69 (;@2;)
                                                                                                                                                end
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 37 (;@33;)
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 6
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 38 (;@32;)
                                                                                                                                                local.get 6
                                                                                                                                                local.get 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 4
                                                                                                                                                local.get 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 39 (;@31;)
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 6
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 40 (;@30;)
                                                                                                                                                local.get 6
                                                                                                                                                local.get 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 4
                                                                                                                                                local.get 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 41 (;@29;)
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 6
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 24
                                                                                                                                                i32.add
                                                                                                                                                local.tee 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 16
                                                                                                                                                i32.add
                                                                                                                                                local.tee 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                i32.const 8
                                                                                                                                                i32.add
                                                                                                                                                local.tee 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 42 (;@28;)
                                                                                                                                                local.get 6
                                                                                                                                                local.get 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 4
                                                                                                                                                local.get 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 736
                                                                                                                                                i32.add
                                                                                                                                                local.get 1
                                                                                                                                                local.get 2
                                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                                block  ;; label = @71
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.load8_u offset=736
                                                                                                                                                  i32.const 1
                                                                                                                                                  i32.eq
                                                                                                                                                  br_if 0 (;@71;)
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 24
                                                                                                                                                  i32.add
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 2
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 736
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=736
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                  local.get 2
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 16
                                                                                                                                                  i32.add
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 4
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 712
                                                                                                                                                  i32.add
                                                                                                                                                  i32.const 8
                                                                                                                                                  i32.add
                                                                                                                                                  i64.load
                                                                                                                                                  i64.store
                                                                                                                                                  local.get 3
                                                                                                                                                  local.get 3
                                                                                                                                                  i64.load offset=712
                                                                                                                                                  i64.store offset=768
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 696
                                                                                                                                                  i32.add
                                                                                                                                                  local.get 3
                                                                                                                                                  i32.const 768
                                                                                                                                                  i32.add
                                                                                                                                                  call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                  br 4 (;@67;)
                                                                                                                                                end
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 416
                                                                                                                                                i32.add
                                                                                                                                                i32.const 1
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=737
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=738
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=417
                                                                                                                                                local.set 4
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=416
                                                                                                                                                local.set 2
                                                                                                                                                br 68 (;@2;)
                                                                                                                                              end
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 42 (;@27;)
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 6
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 43 (;@26;)
                                                                                                                                              local.get 6
                                                                                                                                              local.get 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 4
                                                                                                                                              local.get 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 44 (;@25;)
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 6
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 45 (;@24;)
                                                                                                                                              local.get 6
                                                                                                                                              local.get 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 4
                                                                                                                                              local.get 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 46 (;@23;)
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 6
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 47 (;@22;)
                                                                                                                                              local.get 6
                                                                                                                                              local.get 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 4
                                                                                                                                              local.get 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 48 (;@21;)
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 6
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.tee 7
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 8
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 9
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 10
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 5
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 11
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              local.get 1
                                                                                                                                              local.get 2
                                                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                              block  ;; label = @70
                                                                                                                                                local.get 3
                                                                                                                                                i32.load8_u offset=736
                                                                                                                                                i32.const 1
                                                                                                                                                i32.eq
                                                                                                                                                br_if 0 (;@70;)
                                                                                                                                                local.get 6
                                                                                                                                                local.get 7
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 4
                                                                                                                                                local.get 8
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 9
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=736
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 712
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                                local.get 4
                                                                                                                                                local.get 10
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 5
                                                                                                                                                local.get 11
                                                                                                                                                i64.load
                                                                                                                                                i64.store
                                                                                                                                                local.get 3
                                                                                                                                                local.get 3
                                                                                                                                                i64.load offset=712
                                                                                                                                                i64.store offset=768
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 696
                                                                                                                                                i32.add
                                                                                                                                                local.get 3
                                                                                                                                                i32.const 768
                                                                                                                                                i32.add
                                                                                                                                                call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                                br 3 (;@67;)
                                                                                                                                              end
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 480
                                                                                                                                              i32.add
                                                                                                                                              i32.const 1
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=737
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=738
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=481
                                                                                                                                              local.set 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=480
                                                                                                                                              local.set 2
                                                                                                                                              br 67 (;@2;)
                                                                                                                                            end
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 48 (;@20;)
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 6
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 49 (;@19;)
                                                                                                                                            local.get 6
                                                                                                                                            local.get 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 4
                                                                                                                                            local.get 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 50 (;@18;)
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 6
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 51 (;@17;)
                                                                                                                                            local.get 6
                                                                                                                                            local.get 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 4
                                                                                                                                            local.get 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 52 (;@16;)
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 6
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 53 (;@15;)
                                                                                                                                            local.get 6
                                                                                                                                            local.get 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 4
                                                                                                                                            local.get 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 54 (;@14;)
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 6
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 24
                                                                                                                                            i32.add
                                                                                                                                            local.tee 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 16
                                                                                                                                            i32.add
                                                                                                                                            local.tee 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            i32.const 8
                                                                                                                                            i32.add
                                                                                                                                            local.tee 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=736
                                                                                                                                            i32.const 1
                                                                                                                                            i32.eq
                                                                                                                                            br_if 55 (;@13;)
                                                                                                                                            local.get 6
                                                                                                                                            local.get 7
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 4
                                                                                                                                            local.get 8
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 9
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=736
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 712
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                            local.get 4
                                                                                                                                            local.get 10
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 5
                                                                                                                                            local.get 11
                                                                                                                                            i64.load
                                                                                                                                            i64.store
                                                                                                                                            local.get 3
                                                                                                                                            local.get 3
                                                                                                                                            i64.load offset=712
                                                                                                                                            i64.store offset=768
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 696
                                                                                                                                            i32.add
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 768
                                                                                                                                            i32.add
                                                                                                                                            call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 736
                                                                                                                                            i32.add
                                                                                                                                            local.get 1
                                                                                                                                            local.get 2
                                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                            block  ;; label = @69
                                                                                                                                              local.get 3
                                                                                                                                              i32.load8_u offset=736
                                                                                                                                              i32.const 1
                                                                                                                                              i32.eq
                                                                                                                                              br_if 0 (;@69;)
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 24
                                                                                                                                              i32.add
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              local.tee 2
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              local.tee 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 736
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=736
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                              local.get 2
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 16
                                                                                                                                              i32.add
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 4
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 712
                                                                                                                                              i32.add
                                                                                                                                              i32.const 8
                                                                                                                                              i32.add
                                                                                                                                              i64.load
                                                                                                                                              i64.store
                                                                                                                                              local.get 3
                                                                                                                                              local.get 3
                                                                                                                                              i64.load offset=712
                                                                                                                                              i64.store offset=768
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 696
                                                                                                                                              i32.add
                                                                                                                                              local.get 3
                                                                                                                                              i32.const 768
                                                                                                                                              i32.add
                                                                                                                                              call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                              br 2 (;@67;)
                                                                                                                                            end
                                                                                                                                            local.get 3
                                                                                                                                            i32.const 552
                                                                                                                                            i32.add
                                                                                                                                            i32.const 1
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=737
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=738
                                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=553
                                                                                                                                            local.set 4
                                                                                                                                            local.get 3
                                                                                                                                            i32.load8_u offset=552
                                                                                                                                            local.set 2
                                                                                                                                            br 66 (;@2;)
                                                                                                                                          end
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 55 (;@12;)
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 6
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 56 (;@11;)
                                                                                                                                          local.get 6
                                                                                                                                          local.get 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 4
                                                                                                                                          local.get 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 57 (;@10;)
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 6
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 58 (;@9;)
                                                                                                                                          local.get 6
                                                                                                                                          local.get 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 4
                                                                                                                                          local.get 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 59 (;@8;)
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 6
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 60 (;@7;)
                                                                                                                                          local.get 6
                                                                                                                                          local.get 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 4
                                                                                                                                          local.get 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 61 (;@6;)
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 6
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 62 (;@5;)
                                                                                                                                          local.get 6
                                                                                                                                          local.get 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 4
                                                                                                                                          local.get 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 63 (;@4;)
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 6
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 24
                                                                                                                                          i32.add
                                                                                                                                          local.tee 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 16
                                                                                                                                          i32.add
                                                                                                                                          local.tee 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          i32.const 8
                                                                                                                                          i32.add
                                                                                                                                          local.tee 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 736
                                                                                                                                          i32.add
                                                                                                                                          local.get 1
                                                                                                                                          local.get 2
                                                                                                                                          call $_ZN15svm_abi_decoder7decoder7Decoder12decode_value17h9d37954dd9c0d308E
                                                                                                                                          local.get 3
                                                                                                                                          i32.load8_u offset=736
                                                                                                                                          i32.const 1
                                                                                                                                          i32.eq
                                                                                                                                          br_if 1 (;@66;)
                                                                                                                                          local.get 6
                                                                                                                                          local.get 7
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 4
                                                                                                                                          local.get 8
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 9
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=736
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 712
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                          local.get 4
                                                                                                                                          local.get 10
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 5
                                                                                                                                          local.get 11
                                                                                                                                          i64.load
                                                                                                                                          i64.store
                                                                                                                                          local.get 3
                                                                                                                                          local.get 3
                                                                                                                                          i64.load offset=712
                                                                                                                                          i64.store offset=768
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 696
                                                                                                                                          i32.add
                                                                                                                                          local.get 3
                                                                                                                                          i32.const 768
                                                                                                                                          i32.add
                                                                                                                                          call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E
                                                                                                                                        end
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 672
                                                                                                                                        i32.add
                                                                                                                                        i32.const 8
                                                                                                                                        i32.add
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 696
                                                                                                                                        i32.add
                                                                                                                                        i32.const 8
                                                                                                                                        i32.add
                                                                                                                                        i32.load
                                                                                                                                        local.tee 2
                                                                                                                                        i32.store
                                                                                                                                        local.get 3
                                                                                                                                        local.get 3
                                                                                                                                        i64.load offset=696
                                                                                                                                        local.tee 12
                                                                                                                                        i64.store offset=672
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 768
                                                                                                                                        i32.add
                                                                                                                                        i32.const 8
                                                                                                                                        i32.add
                                                                                                                                        i32.const 1
                                                                                                                                        i32.store
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 780
                                                                                                                                        i32.add
                                                                                                                                        local.get 12
                                                                                                                                        i64.store align=4
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 788
                                                                                                                                        i32.add
                                                                                                                                        local.get 2
                                                                                                                                        i32.store
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 0
                                                                                                                                        i32.store8 offset=768
                                                                                                                                        local.get 3
                                                                                                                                        local.get 3
                                                                                                                                        i32.load offset=691 align=1
                                                                                                                                        i32.store offset=771 align=1
                                                                                                                                        local.get 3
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 695
                                                                                                                                        i32.add
                                                                                                                                        i32.load8_u
                                                                                                                                        i32.store8 offset=775
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 648
                                                                                                                                        i32.add
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 768
                                                                                                                                        i32.add
                                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                        local.get 3
                                                                                                                                        i32.const 648
                                                                                                                                        i32.add
                                                                                                                                        local.set 2
                                                                                                                                        br 16 (;@50;)
                                                                                                                                      end
                                                                                                                                      local.get 3
                                                                                                                                      i32.const 632
                                                                                                                                      i32.add
                                                                                                                                      i32.const 1
                                                                                                                                      local.get 3
                                                                                                                                      i32.load8_u offset=737
                                                                                                                                      local.get 3
                                                                                                                                      i32.load8_u offset=738
                                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                      local.get 3
                                                                                                                                      i32.load8_u offset=633
                                                                                                                                      local.set 4
                                                                                                                                      local.get 3
                                                                                                                                      i32.load8_u offset=632
                                                                                                                                      local.set 2
                                                                                                                                      br 63 (;@2;)
                                                                                                                                    end
                                                                                                                                    local.get 2
                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                    local.tee 2
                                                                                                                                    i32.const 255
                                                                                                                                    i32.and
                                                                                                                                    i32.const 1
                                                                                                                                    i32.eq
                                                                                                                                    br_if 15 (;@49;)
                                                                                                                                    local.get 2
                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                    drop
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    i32.const 0
                                                                                                                                    i32.store8
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    i32.const 0
                                                                                                                                    i32.store
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 0
                                                                                                                                    i32.store8 offset=768
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    i64.load
                                                                                                                                    i64.store
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    i64.load
                                                                                                                                    i64.store
                                                                                                                                    local.get 3
                                                                                                                                    local.get 3
                                                                                                                                    i64.load offset=736
                                                                                                                                    i64.store offset=648
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    local.set 2
                                                                                                                                    br 14 (;@50;)
                                                                                                                                  end
                                                                                                                                  block  ;; label = @64
                                                                                                                                    local.get 2
                                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                    local.tee 2
                                                                                                                                    i32.const 255
                                                                                                                                    i32.and
                                                                                                                                    i32.const 1
                                                                                                                                    i32.eq
                                                                                                                                    br_if 0 (;@64;)
                                                                                                                                    local.get 2
                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                    drop
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    i32.const 1
                                                                                                                                    i32.store8
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    i32.const 0
                                                                                                                                    i32.store
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 0
                                                                                                                                    i32.store8 offset=768
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 768
                                                                                                                                    i32.add
                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    i32.const 8
                                                                                                                                    i32.add
                                                                                                                                    i64.load
                                                                                                                                    i64.store
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 736
                                                                                                                                    i32.add
                                                                                                                                    i32.const 16
                                                                                                                                    i32.add
                                                                                                                                    i64.load
                                                                                                                                    i64.store
                                                                                                                                    local.get 3
                                                                                                                                    local.get 3
                                                                                                                                    i64.load offset=736
                                                                                                                                    i64.store offset=648
                                                                                                                                    local.get 3
                                                                                                                                    i32.const 648
                                                                                                                                    i32.add
                                                                                                                                    local.set 2
                                                                                                                                    br 14 (;@50;)
                                                                                                                                  end
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 32
                                                                                                                                  i32.add
                                                                                                                                  local.get 2
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 24
                                                                                                                                  i32.add
                                                                                                                                  i32.const 1
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=32
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=33
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                  local.get 0
                                                                                                                                  local.get 3
                                                                                                                                  i32.load16_u offset=24
                                                                                                                                  i32.store16 offset=1 align=1
                                                                                                                                  local.get 0
                                                                                                                                  i32.const 1
                                                                                                                                  i32.store8
                                                                                                                                  br 62 (;@1;)
                                                                                                                                end
                                                                                                                                block  ;; label = @63
                                                                                                                                  local.get 2
                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                  local.tee 2
                                                                                                                                  i32.const 255
                                                                                                                                  i32.and
                                                                                                                                  i32.const 1
                                                                                                                                  i32.eq
                                                                                                                                  br_if 0 (;@63;)
                                                                                                                                  i32.const 0
                                                                                                                                  local.set 4
                                                                                                                                  block  ;; label = @64
                                                                                                                                    local.get 2
                                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                    i32.const 255
                                                                                                                                    i32.and
                                                                                                                                    local.tee 2
                                                                                                                                    i32.eqz
                                                                                                                                    br_if 0 (;@64;)
                                                                                                                                    local.get 2
                                                                                                                                    i32.const 16
                                                                                                                                    i32.ne
                                                                                                                                    br_if 61 (;@3;)
                                                                                                                                    i32.const 1
                                                                                                                                    local.set 4
                                                                                                                                  end
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 785
                                                                                                                                  i32.add
                                                                                                                                  local.get 4
                                                                                                                                  i32.store8
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 768
                                                                                                                                  i32.add
                                                                                                                                  i32.const 16
                                                                                                                                  i32.add
                                                                                                                                  i32.const 2
                                                                                                                                  i32.store8
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 768
                                                                                                                                  i32.add
                                                                                                                                  i32.const 8
                                                                                                                                  i32.add
                                                                                                                                  i32.const 0
                                                                                                                                  i32.store
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 0
                                                                                                                                  i32.store8 offset=768
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 736
                                                                                                                                  i32.add
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 768
                                                                                                                                  i32.add
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 648
                                                                                                                                  i32.add
                                                                                                                                  i32.const 8
                                                                                                                                  i32.add
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 736
                                                                                                                                  i32.add
                                                                                                                                  i32.const 8
                                                                                                                                  i32.add
                                                                                                                                  i64.load
                                                                                                                                  i64.store
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 648
                                                                                                                                  i32.add
                                                                                                                                  i32.const 16
                                                                                                                                  i32.add
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 736
                                                                                                                                  i32.add
                                                                                                                                  i32.const 16
                                                                                                                                  i32.add
                                                                                                                                  i64.load
                                                                                                                                  i64.store
                                                                                                                                  local.get 3
                                                                                                                                  local.get 3
                                                                                                                                  i64.load offset=736
                                                                                                                                  i64.store offset=648
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 648
                                                                                                                                  i32.add
                                                                                                                                  local.set 2
                                                                                                                                  br 13 (;@50;)
                                                                                                                                end
                                                                                                                                local.get 3
                                                                                                                                i32.const 48
                                                                                                                                i32.add
                                                                                                                                local.get 2
                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                                local.get 3
                                                                                                                                i32.const 40
                                                                                                                                i32.add
                                                                                                                                i32.const 1
                                                                                                                                local.get 3
                                                                                                                                i32.load8_u offset=48
                                                                                                                                local.get 3
                                                                                                                                i32.load8_u offset=49
                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                                local.get 0
                                                                                                                                local.get 3
                                                                                                                                i32.load16_u offset=40
                                                                                                                                i32.store16 offset=1 align=1
                                                                                                                                local.get 0
                                                                                                                                i32.const 1
                                                                                                                                i32.store8
                                                                                                                                br 61 (;@1;)
                                                                                                                              end
                                                                                                                              block  ;; label = @62
                                                                                                                                block  ;; label = @63
                                                                                                                                  local.get 2
                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                  local.tee 4
                                                                                                                                  i32.const 255
                                                                                                                                  i32.and
                                                                                                                                  i32.const 1
                                                                                                                                  i32.eq
                                                                                                                                  br_if 0 (;@63;)
                                                                                                                                  local.get 4
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                  drop
                                                                                                                                  local.get 2
                                                                                                                                  i32.const 20
                                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h60a0e41c7e5770a4E
                                                                                                                                  local.tee 12
                                                                                                                                  i32.wrap_i64
                                                                                                                                  i32.const 255
                                                                                                                                  i32.and
                                                                                                                                  i32.const 1
                                                                                                                                  i32.ne
                                                                                                                                  br_if 12 (;@51;)
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 72
                                                                                                                                  i32.add
                                                                                                                                  local.get 12
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h7658ac3f58c8eb3aE
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=73
                                                                                                                                  local.set 2
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=72
                                                                                                                                  local.set 4
                                                                                                                                  br 1 (;@62;)
                                                                                                                                end
                                                                                                                                local.get 3
                                                                                                                                i32.const 64
                                                                                                                                i32.add
                                                                                                                                local.get 4
                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                                local.get 3
                                                                                                                                i32.load8_u offset=65
                                                                                                                                local.set 2
                                                                                                                                local.get 3
                                                                                                                                i32.load8_u offset=64
                                                                                                                                local.set 4
                                                                                                                              end
                                                                                                                              local.get 3
                                                                                                                              i32.const 56
                                                                                                                              i32.add
                                                                                                                              i32.const 1
                                                                                                                              local.get 4
                                                                                                                              local.get 2
                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                              local.get 0
                                                                                                                              local.get 3
                                                                                                                              i32.load16_u offset=56
                                                                                                                              i32.store16 offset=1 align=1
                                                                                                                              local.get 0
                                                                                                                              i32.const 1
                                                                                                                              i32.store8
                                                                                                                              br 60 (;@1;)
                                                                                                                            end
                                                                                                                            i32.const 1
                                                                                                                            local.set 1
                                                                                                                            block  ;; label = @61
                                                                                                                              block  ;; label = @62
                                                                                                                                local.get 2
                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                                                                                                                                local.tee 4
                                                                                                                                i32.const 255
                                                                                                                                i32.and
                                                                                                                                i32.const 1
                                                                                                                                i32.eq
                                                                                                                                br_if 0 (;@62;)
                                                                                                                                block  ;; label = @63
                                                                                                                                  block  ;; label = @64
                                                                                                                                    block  ;; label = @65
                                                                                                                                      block  ;; label = @66
                                                                                                                                        block  ;; label = @67
                                                                                                                                          block  ;; label = @68
                                                                                                                                            block  ;; label = @69
                                                                                                                                              block  ;; label = @70
                                                                                                                                                block  ;; label = @71
                                                                                                                                                  local.get 4
                                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                                                  i32.const -1
                                                                                                                                                  i32.add
                                                                                                                                                  local.tee 4
                                                                                                                                                  i32.const 4
                                                                                                                                                  i32.shl
                                                                                                                                                  local.get 4
                                                                                                                                                  i32.const 240
                                                                                                                                                  i32.and
                                                                                                                                                  i32.const 4
                                                                                                                                                  i32.shr_u
                                                                                                                                                  i32.or
                                                                                                                                                  i32.const 255
                                                                                                                                                  i32.and
                                                                                                                                                  br_table 8 (;@63;) 1 (;@70;) 2 (;@69;) 3 (;@68;) 4 (;@67;) 5 (;@66;) 6 (;@65;) 7 (;@64;) 0 (;@71;)
                                                                                                                                                end
                                                                                                                                                i32.const 1049343
                                                                                                                                                i32.const 40
                                                                                                                                                i32.const 1048728
                                                                                                                                                call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
                                                                                                                                                unreachable
                                                                                                                                              end
                                                                                                                                              i32.const 2
                                                                                                                                              local.set 1
                                                                                                                                              br 6 (;@63;)
                                                                                                                                            end
                                                                                                                                            i32.const 3
                                                                                                                                            local.set 1
                                                                                                                                            br 5 (;@63;)
                                                                                                                                          end
                                                                                                                                          i32.const 4
                                                                                                                                          local.set 1
                                                                                                                                          br 4 (;@63;)
                                                                                                                                        end
                                                                                                                                        i32.const 5
                                                                                                                                        local.set 1
                                                                                                                                        br 3 (;@63;)
                                                                                                                                      end
                                                                                                                                      i32.const 6
                                                                                                                                      local.set 1
                                                                                                                                      br 2 (;@63;)
                                                                                                                                    end
                                                                                                                                    i32.const 7
                                                                                                                                    local.set 1
                                                                                                                                    br 1 (;@63;)
                                                                                                                                  end
                                                                                                                                  i32.const 8
                                                                                                                                  local.set 1
                                                                                                                                end
                                                                                                                                local.get 3
                                                                                                                                i32.const 712
                                                                                                                                i32.add
                                                                                                                                local.get 2
                                                                                                                                local.get 1
                                                                                                                                call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E
                                                                                                                                block  ;; label = @63
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=712
                                                                                                                                  i32.const 1
                                                                                                                                  i32.ne
                                                                                                                                  br_if 0 (;@63;)
                                                                                                                                  local.get 3
                                                                                                                                  i32.const 96
                                                                                                                                  i32.add
                                                                                                                                  i32.const 1
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=713
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=714
                                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=97
                                                                                                                                  local.set 2
                                                                                                                                  local.get 3
                                                                                                                                  i32.load8_u offset=96
                                                                                                                                  local.set 4
                                                                                                                                  br 2 (;@61;)
                                                                                                                                end
                                                                                                                                local.get 3
                                                                                                                                i32.const 768
                                                                                                                                i32.add
                                                                                                                                i32.const 8
                                                                                                                                i32.add
                                                                                                                                local.tee 2
                                                                                                                                local.get 3
                                                                                                                                i32.const 712
                                                                                                                                i32.add
                                                                                                                                i32.const 8
                                                                                                                                i32.add
                                                                                                                                i64.load
                                                                                                                                i64.store
                                                                                                                                local.get 3
                                                                                                                                local.get 3
                                                                                                                                i64.load offset=712
                                                                                                                                i64.store offset=768
                                                                                                                                local.get 3
                                                                                                                                i32.const 792
                                                                                                                                i32.add
                                                                                                                                local.get 3
                                                                                                                                i32.const 768
                                                                                                                                i32.add
                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
                                                                                                                                i64.store
                                                                                                                                local.get 3
                                                                                                                                i32.const 768
                                                                                                                                i32.add
                                                                                                                                i32.const 16
                                                                                                                                i32.add
                                                                                                                                i32.const 4
                                                                                                                                i32.store8
                                                                                                                                local.get 2
                                                                                                                                i32.const 0
                                                                                                                                i32.store
                                                                                                                                local.get 3
                                                                                                                                i32.const 0
                                                                                                                                i32.store8 offset=768
                                                                                                                                local.get 3
                                                                                                                                i32.const 736
                                                                                                                                i32.add
                                                                                                                                local.get 3
                                                                                                                                i32.const 768
                                                                                                                                i32.add
                                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                                                local.get 3
                                                                                                                                i32.const 648
                                                                                                                                i32.add
                                                                                                                                i32.const 8
                                                                                                                                i32.add
                                                                                                                                local.get 3
                                                                                                                                i32.const 736
                                                                                                                                i32.add
                                                                                                                                i32.const 8
                                                                                                                                i32.add
                                                                                                                                i64.load
                                                                                                                                i64.store
                                                                                                                                local.get 3
                                                                                                                                i32.const 648
                                                                                                                                i32.add
                                                                                                                                i32.const 16
                                                                                                                                i32.add
                                                                                                                                local.get 3
                                                                                                                                i32.const 736
                                                                                                                                i32.add
                                                                                                                                i32.const 16
                                                                                                                                i32.add
                                                                                                                                i64.load
                                                                                                                                i64.store
                                                                                                                                local.get 3
                                                                                                                                local.get 3
                                                                                                                                i64.load offset=736
                                                                                                                                i64.store offset=648
                                                                                                                                local.get 3
                                                                                                                                i32.const 648
                                                                                                                                i32.add
                                                                                                                                local.set 2
                                                                                                                                br 12 (;@50;)
                                                                                                                              end
                                                                                                                              local.get 3
                                                                                                                              i32.const 88
                                                                                                                              i32.add
                                                                                                                              local.get 4
                                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                              local.get 3
                                                                                                                              i32.load8_u offset=89
                                                                                                                              local.set 2
                                                                                                                              local.get 3
                                                                                                                              i32.load8_u offset=88
                                                                                                                              local.set 4
                                                                                                                            end
                                                                                                                            local.get 3
                                                                                                                            i32.const 80
                                                                                                                            i32.add
                                                                                                                            i32.const 1
                                                                                                                            local.get 4
                                                                                                                            local.get 2
                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                                            local.get 0
                                                                                                                            local.get 3
                                                                                                                            i32.load16_u offset=80
                                                                                                                            i32.store16 offset=1 align=1
                                                                                                                            local.get 0
                                                                                                                            i32.const 1
                                                                                                                            i32.store8
                                                                                                                            br 59 (;@1;)
                                                                                                                          end
                                                                                                                          block  ;; label = @60
                                                                                                                            local.get 2
                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h61d1a1dfe4440be9E
                                                                                                                            local.tee 2
                                                                                                                            i32.const 255
                                                                                                                            i32.and
                                                                                                                            i32.const 1
                                                                                                                            i32.eq
                                                                                                                            br_if 0 (;@60;)
                                                                                                                            local.get 3
                                                                                                                            i32.const 657
                                                                                                                            i32.add
                                                                                                                            local.get 2
                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17ha75a77b107b60e57E
                                                                                                                            i32.store8
                                                                                                                            local.get 3
                                                                                                                            i32.const 5
                                                                                                                            i32.store8 offset=656
                                                                                                                            local.get 3
                                                                                                                            i32.const 0
                                                                                                                            i32.store offset=648
                                                                                                                            local.get 3
                                                                                                                            i32.const 648
                                                                                                                            i32.add
                                                                                                                            local.set 2
                                                                                                                            br 10 (;@50;)
                                                                                                                          end
                                                                                                                          local.get 3
                                                                                                                          i32.const 104
                                                                                                                          i32.add
                                                                                                                          local.get 2
                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h329d37ca6b8370f0E
                                                                                                                          local.get 0
                                                                                                                          local.get 3
                                                                                                                          i32.load16_u offset=104
                                                                                                                          i32.store16 offset=1 align=1
                                                                                                                          local.get 0
                                                                                                                          i32.const 1
                                                                                                                          i32.store8
                                                                                                                          br 58 (;@1;)
                                                                                                                        end
                                                                                                                        i32.const 1
                                                                                                                        local.set 4
                                                                                                                        block  ;; label = @59
                                                                                                                          block  ;; label = @60
                                                                                                                            local.get 2
                                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h61d1a1dfe4440be9E
                                                                                                                            local.tee 2
                                                                                                                            i32.const 255
                                                                                                                            i32.and
                                                                                                                            i32.const 1
                                                                                                                            i32.eq
                                                                                                                            br_if 0 (;@60;)
                                                                                                                            i32.const 0
                                                                                                                            local.set 4
                                                                                                                            local.get 2
                                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17ha75a77b107b60e57E
                                                                                                                            local.set 1
                                                                                                                            br 1 (;@59;)
                                                                                                                          end
                                                                                                                          local.get 3
                                                                                                                          i32.const 120
                                                                                                                          i32.add
                                                                                                                          local.get 2
                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h329d37ca6b8370f0E
                                                                                                                          local.get 3
                                                                                                                          i32.load8_u offset=121
                                                                                                                          local.set 5
                                                                                                                          local.get 3
                                                                                                                          i32.load8_u offset=120
                                                                                                                          local.set 1
                                                                                                                        end
                                                                                                                        local.get 5
                                                                                                                        i32.const 8
                                                                                                                        i32.shl
                                                                                                                        local.get 1
                                                                                                                        i32.const 255
                                                                                                                        i32.and
                                                                                                                        i32.or
                                                                                                                        i32.const 8
                                                                                                                        i32.shl
                                                                                                                        local.get 4
                                                                                                                        i32.or
                                                                                                                        local.set 4
                                                                                                                        block  ;; label = @59
                                                                                                                          local.get 2
                                                                                                                          i32.const 255
                                                                                                                          i32.and
                                                                                                                          i32.const 1
                                                                                                                          i32.eq
                                                                                                                          br_if 0 (;@59;)
                                                                                                                          local.get 3
                                                                                                                          i32.const 657
                                                                                                                          i32.add
                                                                                                                          local.get 4
                                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                                                                                                                          i32.store8
                                                                                                                          local.get 3
                                                                                                                          i32.const 6
                                                                                                                          i32.store8 offset=656
                                                                                                                          local.get 3
                                                                                                                          i32.const 0
                                                                                                                          i32.store offset=648
                                                                                                                          local.get 3
                                                                                                                          i32.const 648
                                                                                                                          i32.add
                                                                                                                          local.set 2
                                                                                                                          br 9 (;@50;)
                                                                                                                        end
                                                                                                                        local.get 3
                                                                                                                        i32.const 112
                                                                                                                        i32.add
                                                                                                                        local.get 4
                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                                        local.get 0
                                                                                                                        local.get 3
                                                                                                                        i32.load16_u offset=112
                                                                                                                        i32.store16 offset=1 align=1
                                                                                                                        local.get 0
                                                                                                                        i32.const 1
                                                                                                                        i32.store8
                                                                                                                        br 57 (;@1;)
                                                                                                                      end
                                                                                                                      block  ;; label = @58
                                                                                                                        local.get 2
                                                                                                                        call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h543a552aefc49ae0E
                                                                                                                        local.tee 2
                                                                                                                        i32.const 255
                                                                                                                        i32.and
                                                                                                                        i32.const 1
                                                                                                                        i32.eq
                                                                                                                        br_if 0 (;@58;)
                                                                                                                        local.get 3
                                                                                                                        i32.const 658
                                                                                                                        i32.add
                                                                                                                        local.get 2
                                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9434170550c1e1daE
                                                                                                                        i32.store16
                                                                                                                        local.get 3
                                                                                                                        i32.const 7
                                                                                                                        i32.store8 offset=656
                                                                                                                        local.get 3
                                                                                                                        i32.const 0
                                                                                                                        i32.store offset=648
                                                                                                                        local.get 3
                                                                                                                        i32.const 648
                                                                                                                        i32.add
                                                                                                                        local.set 2
                                                                                                                        br 8 (;@50;)
                                                                                                                      end
                                                                                                                      local.get 3
                                                                                                                      i32.const 128
                                                                                                                      i32.add
                                                                                                                      local.get 2
                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h8022a205287793acE
                                                                                                                      local.get 0
                                                                                                                      local.get 3
                                                                                                                      i32.load16_u offset=128
                                                                                                                      i32.store16 offset=1 align=1
                                                                                                                      local.get 0
                                                                                                                      i32.const 1
                                                                                                                      i32.store8
                                                                                                                      br 56 (;@1;)
                                                                                                                    end
                                                                                                                    block  ;; label = @57
                                                                                                                      local.get 2
                                                                                                                      call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h543a552aefc49ae0E
                                                                                                                      local.tee 2
                                                                                                                      i32.const 255
                                                                                                                      i32.and
                                                                                                                      i32.const 1
                                                                                                                      i32.eq
                                                                                                                      br_if 0 (;@57;)
                                                                                                                      local.get 3
                                                                                                                      i32.const 658
                                                                                                                      i32.add
                                                                                                                      local.get 2
                                                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9434170550c1e1daE
                                                                                                                      i32.store16
                                                                                                                      local.get 3
                                                                                                                      i32.const 8
                                                                                                                      i32.store8 offset=656
                                                                                                                      local.get 3
                                                                                                                      i32.const 0
                                                                                                                      i32.store offset=648
                                                                                                                      local.get 3
                                                                                                                      i32.const 648
                                                                                                                      i32.add
                                                                                                                      local.set 2
                                                                                                                      br 7 (;@50;)
                                                                                                                    end
                                                                                                                    local.get 3
                                                                                                                    i32.const 136
                                                                                                                    i32.add
                                                                                                                    local.get 2
                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h8022a205287793acE
                                                                                                                    local.get 0
                                                                                                                    local.get 3
                                                                                                                    i32.load16_u offset=136
                                                                                                                    i32.store16 offset=1 align=1
                                                                                                                    local.get 0
                                                                                                                    i32.const 1
                                                                                                                    i32.store8
                                                                                                                    br 55 (;@1;)
                                                                                                                  end
                                                                                                                  block  ;; label = @56
                                                                                                                    local.get 2
                                                                                                                    call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h88974b4922d1e3f2E
                                                                                                                    local.tee 12
                                                                                                                    i32.wrap_i64
                                                                                                                    i32.const 255
                                                                                                                    i32.and
                                                                                                                    i32.const 1
                                                                                                                    i32.eq
                                                                                                                    br_if 0 (;@56;)
                                                                                                                    local.get 3
                                                                                                                    i32.const 660
                                                                                                                    i32.add
                                                                                                                    local.get 12
                                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h6762501a24f861ddE
                                                                                                                    i32.store
                                                                                                                    local.get 3
                                                                                                                    i32.const 9
                                                                                                                    i32.store8 offset=656
                                                                                                                    local.get 3
                                                                                                                    i32.const 0
                                                                                                                    i32.store offset=648
                                                                                                                    local.get 3
                                                                                                                    i32.const 648
                                                                                                                    i32.add
                                                                                                                    local.set 2
                                                                                                                    br 6 (;@50;)
                                                                                                                  end
                                                                                                                  local.get 3
                                                                                                                  i32.const 144
                                                                                                                  i32.add
                                                                                                                  local.get 12
                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hbd78294bab8fc663E
                                                                                                                  local.get 0
                                                                                                                  local.get 3
                                                                                                                  i32.load16_u offset=144
                                                                                                                  i32.store16 offset=1 align=1
                                                                                                                  local.get 0
                                                                                                                  i32.const 1
                                                                                                                  i32.store8
                                                                                                                  br 54 (;@1;)
                                                                                                                end
                                                                                                                block  ;; label = @55
                                                                                                                  local.get 2
                                                                                                                  call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h88974b4922d1e3f2E
                                                                                                                  local.tee 12
                                                                                                                  i32.wrap_i64
                                                                                                                  i32.const 255
                                                                                                                  i32.and
                                                                                                                  i32.const 1
                                                                                                                  i32.eq
                                                                                                                  br_if 0 (;@55;)
                                                                                                                  local.get 3
                                                                                                                  i32.const 660
                                                                                                                  i32.add
                                                                                                                  local.get 12
                                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h6762501a24f861ddE
                                                                                                                  i32.store
                                                                                                                  local.get 3
                                                                                                                  i32.const 10
                                                                                                                  i32.store8 offset=656
                                                                                                                  local.get 3
                                                                                                                  i32.const 0
                                                                                                                  i32.store offset=648
                                                                                                                  local.get 3
                                                                                                                  i32.const 648
                                                                                                                  i32.add
                                                                                                                  local.set 2
                                                                                                                  br 5 (;@50;)
                                                                                                                end
                                                                                                                local.get 3
                                                                                                                i32.const 152
                                                                                                                i32.add
                                                                                                                local.get 12
                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hbd78294bab8fc663E
                                                                                                                local.get 0
                                                                                                                local.get 3
                                                                                                                i32.load16_u offset=152
                                                                                                                i32.store16 offset=1 align=1
                                                                                                                local.get 0
                                                                                                                i32.const 1
                                                                                                                i32.store8
                                                                                                                br 53 (;@1;)
                                                                                                              end
                                                                                                              local.get 3
                                                                                                              i32.const 736
                                                                                                              i32.add
                                                                                                              local.get 2
                                                                                                              call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h6870fd9fd42a67aeE
                                                                                                              block  ;; label = @54
                                                                                                                local.get 3
                                                                                                                i32.load8_u offset=736
                                                                                                                i32.const 1
                                                                                                                i32.eq
                                                                                                                br_if 0 (;@54;)
                                                                                                                local.get 3
                                                                                                                i32.const 768
                                                                                                                i32.add
                                                                                                                i32.const 8
                                                                                                                i32.add
                                                                                                                local.get 3
                                                                                                                i32.const 736
                                                                                                                i32.add
                                                                                                                i32.const 8
                                                                                                                i32.add
                                                                                                                i64.load
                                                                                                                i64.store
                                                                                                                local.get 3
                                                                                                                local.get 3
                                                                                                                i64.load offset=736
                                                                                                                i64.store offset=768
                                                                                                                local.get 3
                                                                                                                i32.const 664
                                                                                                                i32.add
                                                                                                                local.get 3
                                                                                                                i32.const 768
                                                                                                                i32.add
                                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17hcd93f9c9a920f35eE
                                                                                                                i64.store
                                                                                                                local.get 3
                                                                                                                i32.const 11
                                                                                                                i32.store8 offset=656
                                                                                                                local.get 3
                                                                                                                i32.const 0
                                                                                                                i32.store offset=648
                                                                                                                local.get 3
                                                                                                                i32.const 648
                                                                                                                i32.add
                                                                                                                local.set 2
                                                                                                                br 4 (;@50;)
                                                                                                              end
                                                                                                              local.get 3
                                                                                                              i32.const 160
                                                                                                              i32.add
                                                                                                              i32.const 1
                                                                                                              local.get 3
                                                                                                              i32.load8_u offset=737
                                                                                                              local.get 3
                                                                                                              i32.load8_u offset=738
                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hd6856279f96aa83aE
                                                                                                              local.get 0
                                                                                                              local.get 3
                                                                                                              i32.load16_u offset=160
                                                                                                              i32.store16 offset=1 align=1
                                                                                                              local.get 0
                                                                                                              i32.const 1
                                                                                                              i32.store8
                                                                                                              br 52 (;@1;)
                                                                                                            end
                                                                                                            local.get 3
                                                                                                            i32.const 736
                                                                                                            i32.add
                                                                                                            local.get 2
                                                                                                            call $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h6870fd9fd42a67aeE
                                                                                                            block  ;; label = @53
                                                                                                              local.get 3
                                                                                                              i32.load8_u offset=736
                                                                                                              i32.const 1
                                                                                                              i32.eq
                                                                                                              br_if 0 (;@53;)
                                                                                                              local.get 3
                                                                                                              i32.const 768
                                                                                                              i32.add
                                                                                                              i32.const 8
                                                                                                              i32.add
                                                                                                              local.tee 2
                                                                                                              local.get 3
                                                                                                              i32.const 736
                                                                                                              i32.add
                                                                                                              i32.const 8
                                                                                                              i32.add
                                                                                                              i64.load
                                                                                                              i64.store
                                                                                                              local.get 3
                                                                                                              local.get 3
                                                                                                              i64.load offset=736
                                                                                                              i64.store offset=768
                                                                                                              local.get 2
                                                                                                              local.get 3
                                                                                                              i32.const 768
                                                                                                              i32.add
                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17hcd93f9c9a920f35eE
                                                                                                              i64.store
                                                                                                              local.get 3
                                                                                                              i32.const 0
                                                                                                              i32.store8 offset=768
                                                                                                              local.get 3
                                                                                                              i32.const 664
                                                                                                              i32.add
                                                                                                              local.get 3
                                                                                                              i32.const 768
                                                                                                              i32.add
                                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
                                                                                                              i64.store
                                                                                                              local.get 3
                                                                                                              i32.const 12
                                                                                                              i32.store8 offset=656
                                                                                                              local.get 3
                                                                                                              i32.const 0
                                                                                                              i32.store offset=648
                                                                                                              local.get 3
                                                                                                              i32.const 648
                                                                                                              i32.add
                                                                                                              local.set 2
                                                                                                              br 3 (;@50;)
                                                                                                            end
                                                                                                            local.get 3
                                                                                                            i32.const 176
                                                                                                            i32.add
                                                                                                            i32.const 1
                                                                                                            local.get 3
                                                                                                            i32.load8_u offset=737
                                                                                                            local.get 3
                                                                                                            i32.load8_u offset=738
                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hd6856279f96aa83aE
                                                                                                            local.get 3
                                                                                                            i32.const 168
                                                                                                            i32.add
                                                                                                            i32.const 1
                                                                                                            local.get 3
                                                                                                            i32.load8_u offset=176
                                                                                                            local.get 3
                                                                                                            i32.load8_u offset=177
                                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
                                                                                                            local.get 0
                                                                                                            local.get 3
                                                                                                            i32.load16_u offset=168
                                                                                                            i32.store16 offset=1 align=1
                                                                                                            local.get 0
                                                                                                            i32.const 1
                                                                                                            i32.store8
                                                                                                            br 51 (;@1;)
                                                                                                          end
                                                                                                          local.get 3
                                                                                                          i32.const 192
                                                                                                          i32.add
                                                                                                          local.get 4
                                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                          local.get 3
                                                                                                          i32.load8_u offset=193
                                                                                                          local.set 4
                                                                                                          local.get 3
                                                                                                          i32.load8_u offset=192
                                                                                                          local.set 2
                                                                                                          br 49 (;@2;)
                                                                                                        end
                                                                                                        local.get 3
                                                                                                        i32.const 768
                                                                                                        i32.add
                                                                                                        i32.const 20
                                                                                                        i32.add
                                                                                                        local.get 12
                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h037337940127482eE
                                                                                                        i32.store
                                                                                                        local.get 3
                                                                                                        i32.const 768
                                                                                                        i32.add
                                                                                                        i32.const 16
                                                                                                        i32.add
                                                                                                        i32.const 3
                                                                                                        i32.store8
                                                                                                        local.get 3
                                                                                                        i32.const 768
                                                                                                        i32.add
                                                                                                        i32.const 8
                                                                                                        i32.add
                                                                                                        i32.const 0
                                                                                                        i32.store
                                                                                                        local.get 3
                                                                                                        i32.const 0
                                                                                                        i32.store8 offset=768
                                                                                                        local.get 3
                                                                                                        i32.const 736
                                                                                                        i32.add
                                                                                                        local.get 3
                                                                                                        i32.const 768
                                                                                                        i32.add
                                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E
                                                                                                        local.get 3
                                                                                                        i32.const 648
                                                                                                        i32.add
                                                                                                        i32.const 8
                                                                                                        i32.add
                                                                                                        local.get 3
                                                                                                        i32.const 736
                                                                                                        i32.add
                                                                                                        i32.const 8
                                                                                                        i32.add
                                                                                                        i64.load
                                                                                                        i64.store
                                                                                                        local.get 3
                                                                                                        i32.const 648
                                                                                                        i32.add
                                                                                                        i32.const 16
                                                                                                        i32.add
                                                                                                        local.get 3
                                                                                                        i32.const 736
                                                                                                        i32.add
                                                                                                        i32.const 16
                                                                                                        i32.add
                                                                                                        i64.load
                                                                                                        i64.store
                                                                                                        local.get 3
                                                                                                        local.get 3
                                                                                                        i64.load offset=736
                                                                                                        i64.store offset=648
                                                                                                        local.get 3
                                                                                                        i32.const 648
                                                                                                        i32.add
                                                                                                        local.set 2
                                                                                                      end
                                                                                                      local.get 0
                                                                                                      i32.const 0
                                                                                                      i32.store8
                                                                                                      local.get 0
                                                                                                      i32.const 8
                                                                                                      i32.add
                                                                                                      local.get 2
                                                                                                      i64.load
                                                                                                      i64.store
                                                                                                      local.get 0
                                                                                                      i32.const 24
                                                                                                      i32.add
                                                                                                      local.get 2
                                                                                                      i32.const 16
                                                                                                      i32.add
                                                                                                      i64.load
                                                                                                      i64.store
                                                                                                      local.get 0
                                                                                                      i32.const 16
                                                                                                      i32.add
                                                                                                      local.get 2
                                                                                                      i32.const 8
                                                                                                      i32.add
                                                                                                      i64.load
                                                                                                      i64.store
                                                                                                      br 48 (;@1;)
                                                                                                    end
                                                                                                    local.get 3
                                                                                                    i32.const 16
                                                                                                    i32.add
                                                                                                    local.get 2
                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                                                                                                    local.get 3
                                                                                                    i32.const 8
                                                                                                    i32.add
                                                                                                    i32.const 1
                                                                                                    local.get 3
                                                                                                    i32.load8_u offset=16
                                                                                                    local.get 3
                                                                                                    i32.load8_u offset=17
                                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                    local.get 0
                                                                                                    local.get 3
                                                                                                    i32.load16_u offset=8
                                                                                                    i32.store16 offset=1 align=1
                                                                                                    local.get 0
                                                                                                    i32.const 1
                                                                                                    i32.store8
                                                                                                    br 47 (;@1;)
                                                                                                  end
                                                                                                  local.get 3
                                                                                                  i32.const 208
                                                                                                  i32.add
                                                                                                  i32.const 1
                                                                                                  local.get 3
                                                                                                  i32.load8_u offset=737
                                                                                                  local.get 3
                                                                                                  i32.load8_u offset=738
                                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                  local.get 3
                                                                                                  i32.load8_u offset=209
                                                                                                  local.set 4
                                                                                                  local.get 3
                                                                                                  i32.load8_u offset=208
                                                                                                  local.set 2
                                                                                                  br 45 (;@2;)
                                                                                                end
                                                                                                local.get 3
                                                                                                i32.const 224
                                                                                                i32.add
                                                                                                i32.const 1
                                                                                                local.get 3
                                                                                                i32.load8_u offset=737
                                                                                                local.get 3
                                                                                                i32.load8_u offset=738
                                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                                local.get 3
                                                                                                i32.load8_u offset=225
                                                                                                local.set 4
                                                                                                local.get 3
                                                                                                i32.load8_u offset=224
                                                                                                local.set 2
                                                                                                br 44 (;@2;)
                                                                                              end
                                                                                              local.get 3
                                                                                              i32.const 232
                                                                                              i32.add
                                                                                              i32.const 1
                                                                                              local.get 3
                                                                                              i32.load8_u offset=737
                                                                                              local.get 3
                                                                                              i32.load8_u offset=738
                                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                              local.get 3
                                                                                              i32.load8_u offset=233
                                                                                              local.set 4
                                                                                              local.get 3
                                                                                              i32.load8_u offset=232
                                                                                              local.set 2
                                                                                              br 43 (;@2;)
                                                                                            end
                                                                                            local.get 3
                                                                                            i32.const 248
                                                                                            i32.add
                                                                                            i32.const 1
                                                                                            local.get 3
                                                                                            i32.load8_u offset=737
                                                                                            local.get 3
                                                                                            i32.load8_u offset=738
                                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                            local.get 3
                                                                                            i32.load8_u offset=249
                                                                                            local.set 4
                                                                                            local.get 3
                                                                                            i32.load8_u offset=248
                                                                                            local.set 2
                                                                                            br 42 (;@2;)
                                                                                          end
                                                                                          local.get 3
                                                                                          i32.const 256
                                                                                          i32.add
                                                                                          i32.const 1
                                                                                          local.get 3
                                                                                          i32.load8_u offset=737
                                                                                          local.get 3
                                                                                          i32.load8_u offset=738
                                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                          local.get 3
                                                                                          i32.load8_u offset=257
                                                                                          local.set 4
                                                                                          local.get 3
                                                                                          i32.load8_u offset=256
                                                                                          local.set 2
                                                                                          br 41 (;@2;)
                                                                                        end
                                                                                        local.get 3
                                                                                        i32.const 264
                                                                                        i32.add
                                                                                        i32.const 1
                                                                                        local.get 3
                                                                                        i32.load8_u offset=737
                                                                                        local.get 3
                                                                                        i32.load8_u offset=738
                                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                        local.get 3
                                                                                        i32.load8_u offset=265
                                                                                        local.set 4
                                                                                        local.get 3
                                                                                        i32.load8_u offset=264
                                                                                        local.set 2
                                                                                        br 40 (;@2;)
                                                                                      end
                                                                                      local.get 3
                                                                                      i32.const 280
                                                                                      i32.add
                                                                                      i32.const 1
                                                                                      local.get 3
                                                                                      i32.load8_u offset=737
                                                                                      local.get 3
                                                                                      i32.load8_u offset=738
                                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                      local.get 3
                                                                                      i32.load8_u offset=281
                                                                                      local.set 4
                                                                                      local.get 3
                                                                                      i32.load8_u offset=280
                                                                                      local.set 2
                                                                                      br 39 (;@2;)
                                                                                    end
                                                                                    local.get 3
                                                                                    i32.const 288
                                                                                    i32.add
                                                                                    i32.const 1
                                                                                    local.get 3
                                                                                    i32.load8_u offset=737
                                                                                    local.get 3
                                                                                    i32.load8_u offset=738
                                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                    local.get 3
                                                                                    i32.load8_u offset=289
                                                                                    local.set 4
                                                                                    local.get 3
                                                                                    i32.load8_u offset=288
                                                                                    local.set 2
                                                                                    br 38 (;@2;)
                                                                                  end
                                                                                  local.get 3
                                                                                  i32.const 296
                                                                                  i32.add
                                                                                  i32.const 1
                                                                                  local.get 3
                                                                                  i32.load8_u offset=737
                                                                                  local.get 3
                                                                                  i32.load8_u offset=738
                                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                  local.get 3
                                                                                  i32.load8_u offset=297
                                                                                  local.set 4
                                                                                  local.get 3
                                                                                  i32.load8_u offset=296
                                                                                  local.set 2
                                                                                  br 37 (;@2;)
                                                                                end
                                                                                local.get 3
                                                                                i32.const 304
                                                                                i32.add
                                                                                i32.const 1
                                                                                local.get 3
                                                                                i32.load8_u offset=737
                                                                                local.get 3
                                                                                i32.load8_u offset=738
                                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                                local.get 3
                                                                                i32.load8_u offset=305
                                                                                local.set 4
                                                                                local.get 3
                                                                                i32.load8_u offset=304
                                                                                local.set 2
                                                                                br 36 (;@2;)
                                                                              end
                                                                              local.get 3
                                                                              i32.const 320
                                                                              i32.add
                                                                              i32.const 1
                                                                              local.get 3
                                                                              i32.load8_u offset=737
                                                                              local.get 3
                                                                              i32.load8_u offset=738
                                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                              local.get 3
                                                                              i32.load8_u offset=321
                                                                              local.set 4
                                                                              local.get 3
                                                                              i32.load8_u offset=320
                                                                              local.set 2
                                                                              br 35 (;@2;)
                                                                            end
                                                                            local.get 3
                                                                            i32.const 328
                                                                            i32.add
                                                                            i32.const 1
                                                                            local.get 3
                                                                            i32.load8_u offset=737
                                                                            local.get 3
                                                                            i32.load8_u offset=738
                                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                            local.get 3
                                                                            i32.load8_u offset=329
                                                                            local.set 4
                                                                            local.get 3
                                                                            i32.load8_u offset=328
                                                                            local.set 2
                                                                            br 34 (;@2;)
                                                                          end
                                                                          local.get 3
                                                                          i32.const 336
                                                                          i32.add
                                                                          i32.const 1
                                                                          local.get 3
                                                                          i32.load8_u offset=737
                                                                          local.get 3
                                                                          i32.load8_u offset=738
                                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                          local.get 3
                                                                          i32.load8_u offset=337
                                                                          local.set 4
                                                                          local.get 3
                                                                          i32.load8_u offset=336
                                                                          local.set 2
                                                                          br 33 (;@2;)
                                                                        end
                                                                        local.get 3
                                                                        i32.const 344
                                                                        i32.add
                                                                        i32.const 1
                                                                        local.get 3
                                                                        i32.load8_u offset=737
                                                                        local.get 3
                                                                        i32.load8_u offset=738
                                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                        local.get 3
                                                                        i32.load8_u offset=345
                                                                        local.set 4
                                                                        local.get 3
                                                                        i32.load8_u offset=344
                                                                        local.set 2
                                                                        br 32 (;@2;)
                                                                      end
                                                                      local.get 3
                                                                      i32.const 352
                                                                      i32.add
                                                                      i32.const 1
                                                                      local.get 3
                                                                      i32.load8_u offset=737
                                                                      local.get 3
                                                                      i32.load8_u offset=738
                                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                      local.get 3
                                                                      i32.load8_u offset=353
                                                                      local.set 4
                                                                      local.get 3
                                                                      i32.load8_u offset=352
                                                                      local.set 2
                                                                      br 31 (;@2;)
                                                                    end
                                                                    local.get 3
                                                                    i32.const 368
                                                                    i32.add
                                                                    i32.const 1
                                                                    local.get 3
                                                                    i32.load8_u offset=737
                                                                    local.get 3
                                                                    i32.load8_u offset=738
                                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                    local.get 3
                                                                    i32.load8_u offset=369
                                                                    local.set 4
                                                                    local.get 3
                                                                    i32.load8_u offset=368
                                                                    local.set 2
                                                                    br 30 (;@2;)
                                                                  end
                                                                  local.get 3
                                                                  i32.const 376
                                                                  i32.add
                                                                  i32.const 1
                                                                  local.get 3
                                                                  i32.load8_u offset=737
                                                                  local.get 3
                                                                  i32.load8_u offset=738
                                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                  local.get 3
                                                                  i32.load8_u offset=377
                                                                  local.set 4
                                                                  local.get 3
                                                                  i32.load8_u offset=376
                                                                  local.set 2
                                                                  br 29 (;@2;)
                                                                end
                                                                local.get 3
                                                                i32.const 384
                                                                i32.add
                                                                i32.const 1
                                                                local.get 3
                                                                i32.load8_u offset=737
                                                                local.get 3
                                                                i32.load8_u offset=738
                                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                                local.get 3
                                                                i32.load8_u offset=385
                                                                local.set 4
                                                                local.get 3
                                                                i32.load8_u offset=384
                                                                local.set 2
                                                                br 28 (;@2;)
                                                              end
                                                              local.get 3
                                                              i32.const 392
                                                              i32.add
                                                              i32.const 1
                                                              local.get 3
                                                              i32.load8_u offset=737
                                                              local.get 3
                                                              i32.load8_u offset=738
                                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                              local.get 3
                                                              i32.load8_u offset=393
                                                              local.set 4
                                                              local.get 3
                                                              i32.load8_u offset=392
                                                              local.set 2
                                                              br 27 (;@2;)
                                                            end
                                                            local.get 3
                                                            i32.const 400
                                                            i32.add
                                                            i32.const 1
                                                            local.get 3
                                                            i32.load8_u offset=737
                                                            local.get 3
                                                            i32.load8_u offset=738
                                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                            local.get 3
                                                            i32.load8_u offset=401
                                                            local.set 4
                                                            local.get 3
                                                            i32.load8_u offset=400
                                                            local.set 2
                                                            br 26 (;@2;)
                                                          end
                                                          local.get 3
                                                          i32.const 408
                                                          i32.add
                                                          i32.const 1
                                                          local.get 3
                                                          i32.load8_u offset=737
                                                          local.get 3
                                                          i32.load8_u offset=738
                                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                          local.get 3
                                                          i32.load8_u offset=409
                                                          local.set 4
                                                          local.get 3
                                                          i32.load8_u offset=408
                                                          local.set 2
                                                          br 25 (;@2;)
                                                        end
                                                        local.get 3
                                                        i32.const 424
                                                        i32.add
                                                        i32.const 1
                                                        local.get 3
                                                        i32.load8_u offset=737
                                                        local.get 3
                                                        i32.load8_u offset=738
                                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                        local.get 3
                                                        i32.load8_u offset=425
                                                        local.set 4
                                                        local.get 3
                                                        i32.load8_u offset=424
                                                        local.set 2
                                                        br 24 (;@2;)
                                                      end
                                                      local.get 3
                                                      i32.const 432
                                                      i32.add
                                                      i32.const 1
                                                      local.get 3
                                                      i32.load8_u offset=737
                                                      local.get 3
                                                      i32.load8_u offset=738
                                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                      local.get 3
                                                      i32.load8_u offset=433
                                                      local.set 4
                                                      local.get 3
                                                      i32.load8_u offset=432
                                                      local.set 2
                                                      br 23 (;@2;)
                                                    end
                                                    local.get 3
                                                    i32.const 440
                                                    i32.add
                                                    i32.const 1
                                                    local.get 3
                                                    i32.load8_u offset=737
                                                    local.get 3
                                                    i32.load8_u offset=738
                                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                    local.get 3
                                                    i32.load8_u offset=441
                                                    local.set 4
                                                    local.get 3
                                                    i32.load8_u offset=440
                                                    local.set 2
                                                    br 22 (;@2;)
                                                  end
                                                  local.get 3
                                                  i32.const 448
                                                  i32.add
                                                  i32.const 1
                                                  local.get 3
                                                  i32.load8_u offset=737
                                                  local.get 3
                                                  i32.load8_u offset=738
                                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                  local.get 3
                                                  i32.load8_u offset=449
                                                  local.set 4
                                                  local.get 3
                                                  i32.load8_u offset=448
                                                  local.set 2
                                                  br 21 (;@2;)
                                                end
                                                local.get 3
                                                i32.const 456
                                                i32.add
                                                i32.const 1
                                                local.get 3
                                                i32.load8_u offset=737
                                                local.get 3
                                                i32.load8_u offset=738
                                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                                local.get 3
                                                i32.load8_u offset=457
                                                local.set 4
                                                local.get 3
                                                i32.load8_u offset=456
                                                local.set 2
                                                br 20 (;@2;)
                                              end
                                              local.get 3
                                              i32.const 464
                                              i32.add
                                              i32.const 1
                                              local.get 3
                                              i32.load8_u offset=737
                                              local.get 3
                                              i32.load8_u offset=738
                                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                              local.get 3
                                              i32.load8_u offset=465
                                              local.set 4
                                              local.get 3
                                              i32.load8_u offset=464
                                              local.set 2
                                              br 19 (;@2;)
                                            end
                                            local.get 3
                                            i32.const 472
                                            i32.add
                                            i32.const 1
                                            local.get 3
                                            i32.load8_u offset=737
                                            local.get 3
                                            i32.load8_u offset=738
                                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                            local.get 3
                                            i32.load8_u offset=473
                                            local.set 4
                                            local.get 3
                                            i32.load8_u offset=472
                                            local.set 2
                                            br 18 (;@2;)
                                          end
                                          local.get 3
                                          i32.const 488
                                          i32.add
                                          i32.const 1
                                          local.get 3
                                          i32.load8_u offset=737
                                          local.get 3
                                          i32.load8_u offset=738
                                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                          local.get 3
                                          i32.load8_u offset=489
                                          local.set 4
                                          local.get 3
                                          i32.load8_u offset=488
                                          local.set 2
                                          br 17 (;@2;)
                                        end
                                        local.get 3
                                        i32.const 496
                                        i32.add
                                        i32.const 1
                                        local.get 3
                                        i32.load8_u offset=737
                                        local.get 3
                                        i32.load8_u offset=738
                                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                        local.get 3
                                        i32.load8_u offset=497
                                        local.set 4
                                        local.get 3
                                        i32.load8_u offset=496
                                        local.set 2
                                        br 16 (;@2;)
                                      end
                                      local.get 3
                                      i32.const 504
                                      i32.add
                                      i32.const 1
                                      local.get 3
                                      i32.load8_u offset=737
                                      local.get 3
                                      i32.load8_u offset=738
                                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                      local.get 3
                                      i32.load8_u offset=505
                                      local.set 4
                                      local.get 3
                                      i32.load8_u offset=504
                                      local.set 2
                                      br 15 (;@2;)
                                    end
                                    local.get 3
                                    i32.const 512
                                    i32.add
                                    i32.const 1
                                    local.get 3
                                    i32.load8_u offset=737
                                    local.get 3
                                    i32.load8_u offset=738
                                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                    local.get 3
                                    i32.load8_u offset=513
                                    local.set 4
                                    local.get 3
                                    i32.load8_u offset=512
                                    local.set 2
                                    br 14 (;@2;)
                                  end
                                  local.get 3
                                  i32.const 520
                                  i32.add
                                  i32.const 1
                                  local.get 3
                                  i32.load8_u offset=737
                                  local.get 3
                                  i32.load8_u offset=738
                                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                  local.get 3
                                  i32.load8_u offset=521
                                  local.set 4
                                  local.get 3
                                  i32.load8_u offset=520
                                  local.set 2
                                  br 13 (;@2;)
                                end
                                local.get 3
                                i32.const 528
                                i32.add
                                i32.const 1
                                local.get 3
                                i32.load8_u offset=737
                                local.get 3
                                i32.load8_u offset=738
                                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                                local.get 3
                                i32.load8_u offset=529
                                local.set 4
                                local.get 3
                                i32.load8_u offset=528
                                local.set 2
                                br 12 (;@2;)
                              end
                              local.get 3
                              i32.const 536
                              i32.add
                              i32.const 1
                              local.get 3
                              i32.load8_u offset=737
                              local.get 3
                              i32.load8_u offset=738
                              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                              local.get 3
                              i32.load8_u offset=537
                              local.set 4
                              local.get 3
                              i32.load8_u offset=536
                              local.set 2
                              br 11 (;@2;)
                            end
                            local.get 3
                            i32.const 544
                            i32.add
                            i32.const 1
                            local.get 3
                            i32.load8_u offset=737
                            local.get 3
                            i32.load8_u offset=738
                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                            local.get 3
                            i32.load8_u offset=545
                            local.set 4
                            local.get 3
                            i32.load8_u offset=544
                            local.set 2
                            br 10 (;@2;)
                          end
                          local.get 3
                          i32.const 560
                          i32.add
                          i32.const 1
                          local.get 3
                          i32.load8_u offset=737
                          local.get 3
                          i32.load8_u offset=738
                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                          local.get 3
                          i32.load8_u offset=561
                          local.set 4
                          local.get 3
                          i32.load8_u offset=560
                          local.set 2
                          br 9 (;@2;)
                        end
                        local.get 3
                        i32.const 568
                        i32.add
                        i32.const 1
                        local.get 3
                        i32.load8_u offset=737
                        local.get 3
                        i32.load8_u offset=738
                        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                        local.get 3
                        i32.load8_u offset=569
                        local.set 4
                        local.get 3
                        i32.load8_u offset=568
                        local.set 2
                        br 8 (;@2;)
                      end
                      local.get 3
                      i32.const 576
                      i32.add
                      i32.const 1
                      local.get 3
                      i32.load8_u offset=737
                      local.get 3
                      i32.load8_u offset=738
                      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                      local.get 3
                      i32.load8_u offset=577
                      local.set 4
                      local.get 3
                      i32.load8_u offset=576
                      local.set 2
                      br 7 (;@2;)
                    end
                    local.get 3
                    i32.const 584
                    i32.add
                    i32.const 1
                    local.get 3
                    i32.load8_u offset=737
                    local.get 3
                    i32.load8_u offset=738
                    call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                    local.get 3
                    i32.load8_u offset=585
                    local.set 4
                    local.get 3
                    i32.load8_u offset=584
                    local.set 2
                    br 6 (;@2;)
                  end
                  local.get 3
                  i32.const 592
                  i32.add
                  i32.const 1
                  local.get 3
                  i32.load8_u offset=737
                  local.get 3
                  i32.load8_u offset=738
                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                  local.get 3
                  i32.load8_u offset=593
                  local.set 4
                  local.get 3
                  i32.load8_u offset=592
                  local.set 2
                  br 5 (;@2;)
                end
                local.get 3
                i32.const 600
                i32.add
                i32.const 1
                local.get 3
                i32.load8_u offset=737
                local.get 3
                i32.load8_u offset=738
                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
                local.get 3
                i32.load8_u offset=601
                local.set 4
                local.get 3
                i32.load8_u offset=600
                local.set 2
                br 4 (;@2;)
              end
              local.get 3
              i32.const 608
              i32.add
              i32.const 1
              local.get 3
              i32.load8_u offset=737
              local.get 3
              i32.load8_u offset=738
              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
              local.get 3
              i32.load8_u offset=609
              local.set 4
              local.get 3
              i32.load8_u offset=608
              local.set 2
              br 3 (;@2;)
            end
            local.get 3
            i32.const 616
            i32.add
            i32.const 1
            local.get 3
            i32.load8_u offset=737
            local.get 3
            i32.load8_u offset=738
            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
            local.get 3
            i32.load8_u offset=617
            local.set 4
            local.get 3
            i32.load8_u offset=616
            local.set 2
            br 2 (;@2;)
          end
          local.get 3
          i32.const 624
          i32.add
          i32.const 1
          local.get 3
          i32.load8_u offset=737
          local.get 3
          i32.load8_u offset=738
          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
          local.get 3
          i32.load8_u offset=625
          local.set 4
          local.get 3
          i32.load8_u offset=624
          local.set 2
          br 1 (;@2;)
        end
        i32.const 1049343
        i32.const 40
        i32.const 1048712
        call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
        unreachable
      end
      local.get 3
      i32.const 184
      i32.add
      i32.const 1
      local.get 2
      local.get 4
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE
      local.get 0
      local.get 3
      i32.load16_u offset=184
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 1
      i32.store8
    end
    local.get 3
    i32.const 800
    i32.add
    global.set 0)
  (func $_ZN4core9panicking5panic17hc64f9753b8a1be76E (type 5) (param i32 i32 i32)
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
    i32.const 1048592
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
    call $_ZN4core9panicking9panic_fmt17hfa15f5472ef5e557E
    unreachable)
  (func $load_addr (type 4)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 8
    i32.add
    call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$12get_calldata17h9c3a1c3f928c46f6E
    i32.const 0
    i32.const 20
    call $_ZN13svm_sdk_alloc12svm_allocate17h12629aa7a0722256E
    local.tee 1
    call $_ZN15svm_sdk_storage3ext11svm_load16017hb02d905ebf79dc3bE
    local.get 0
    i32.const 16
    i32.add
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$13with_capacity17h5c5f0a792dfa361bE
    local.get 0
    i32.const 16
    i32.add
    i32.const 64
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=1
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=2
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=3
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=4
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=5
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=6
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=7
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=8
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=9
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=10
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=11
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=12
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=13
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=14
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=15
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=16
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=17
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=18
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.load8_u offset=19
    call $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE
    local.get 0
    i32.load offset=24
    local.get 0
    i32.load offset=16
    call $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$14set_returndata17h50222e5da971306cE
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h0af1a76b2490332dE (type 1) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1049172
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 2
    i32.const 1
    i32.add
    i32.store
    local.get 0
    i32.load offset=8
    local.get 2
    i32.add
    local.get 1
    i32.store8)
  (func $svm_fund (type 4))
  (func $_ZN4core9panicking9panic_fmt17hfa15f5472ef5e557E (type 1) (param i32 i32)
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
    i32.const 1048576
    i32.store offset=4
    local.get 2
    i32.const 1048592
    i32.store
    local.get 2
    call $rust_begin_unwind
    unreachable)
  (func $rust_begin_unwind (type 3) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=12
    local.set 2
    local.get 0
    i32.load offset=8
    call $_ZN4core6option15Option$LT$T$GT$6unwrap17h29d0a0f74d23cc07E
    local.set 3
    local.get 1
    local.get 2
    i32.store offset=8
    local.get 1
    local.get 0
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 1
    call $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h404ad66b7b407dddE
    unreachable)
  (func $_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17h0375e219d24c8f94E (type 3) (param i32))
  (func $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h103f0123c0f85d75E (type 6) (param i32) (result i64)
    i64.const -8904177938637813917)
  (func $_ZN3std9panicking20rust_panic_with_hook17h4e1267e42c34e062E (type 4)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=1049464
    i32.const 1
    i32.add
    i32.store offset=1049464
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1049472
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.const 0
          i32.load offset=1049476
          i32.const 1
          i32.add
          local.tee 0
          i32.store offset=1049476
          local.get 0
          i32.const 3
          i32.ge_u
          br_if 2 (;@1;)
          i32.const 0
          i32.load offset=1049480
          i32.const -1
          i32.le_s
          br_if 2 (;@1;)
          local.get 0
          i32.const 2
          i32.lt_u
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        i64.const 4294967297
        i64.store offset=1049472
        i32.const 0
        i32.load offset=1049480
        i32.const -1
        i32.le_s
        br_if 1 (;@1;)
      end
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 4)
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h29d0a0f74d23cc07E (type 2) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1048592
      i32.const 43
      i32.const 1048636
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0)
  (func $_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h404ad66b7b407dddE (type 3) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 1
    local.get 0
    i64.load align=4
    i64.store
    local.get 1
    call $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h55e6bb589840bdacE
    unreachable)
  (func $_ZN3std9panicking19begin_panic_handler28_$u7b$$u7b$closure$u7d$$u7d$17h55e6bb589840bdacE (type 3) (param i32)
    local.get 0
    i32.load
    local.tee 0
    i32.const 20
    i32.add
    i32.load
    drop
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      br_table 0 (;@1;) 0 (;@1;) 0 (;@1;)
    end
    call $_ZN3std9panicking20rust_panic_with_hook17h4e1267e42c34e062E
    unreachable)
  (func $_ZN15svm_abi_decoder6cursor6Cursor4peek17h67be1e4f303ac267E (type 1) (param i32 i32)
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E (type 2) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 16776960
    i32.and
    i32.const 8
    i32.shr_u)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE (type 1) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E (type 2) (param i32) (result i32)
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
    call $_ZN15svm_abi_decoder6cursor6Cursor4peek17h67be1e4f303ac267E
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.const 1
      i32.add
      local.tee 3
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 1048976
      i32.const 28
      i32.const 1049004
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 1
    i32.load8_u offset=9
    local.set 2
    local.get 1
    i32.load8_u offset=8
    local.set 4
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 2
    i32.const 255
    i32.and
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9918fe3a184f25c5E (type 1) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.load8_u
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store)
  (func $_ZN11svm_sdk_std3vec12Vec$LT$T$GT$4push17h35e4f67b148ff0e6E (type 1) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1049172
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 2
    i32.const 1
    i32.add
    i32.store
    local.get 0
    i32.load offset=8
    local.get 2
    i32.const 24
    i32.mul
    i32.add
    local.tee 0
    local.get 1
    i64.load
    i64.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hec4d04f204cac3cdE (type 7) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 3
    i32.store8 offset=1
    local.get 0
    local.get 2
    i32.store8)
  (func $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h60a0e41c7e5770a4E (type 8) (param i32 i32) (result i64)
    (local i32 i32 i64)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 2
        local.get 1
        i32.add
        local.tee 1
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -1
        i32.add
        local.tee 3
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        i64.const 0
        local.set 4
        block  ;; label = @3
          local.get 3
          local.get 0
          i32.load offset=8
          i32.ge_u
          local.tee 3
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          i32.load
          local.get 2
          i32.add
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.set 4
        end
        local.get 4
        local.get 3
        i64.extend_i32_u
        i64.or
        i64.const 512
        i64.or
        return
      end
      i32.const 1049024
      i32.const 28
      i32.const 1049052
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    i32.const 1049072
    i32.const 33
    i32.const 1049052
    call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
    unreachable)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h7658ac3f58c8eb3aE (type 9) (param i32 i64)
    block  ;; label = @1
      local.get 1
      i32.wrap_i64
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E (type 5) (param i32 i32 i32)
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
                            call $_ZN15svm_abi_decoder7decoder7Decoder10read_bytes17h60a0e41c7e5770a4E
                            local.tee 4
                            i32.wrap_i64
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 4
                            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h037337940127482eE
                            local.set 1
                            local.get 2
                            i32.const -1
                            i32.add
                            br_table 2 (;@10;) 3 (;@9;) 4 (;@8;) 5 (;@7;) 6 (;@6;) 7 (;@5;) 8 (;@4;) 9 (;@3;) 1 (;@11;)
                          end
                          local.get 3
                          i32.const 8
                          i32.add
                          local.get 4
                          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h7658ac3f58c8eb3aE
                          local.get 0
                          local.get 3
                          i32.load16_u offset=8
                          i32.store16 offset=1 align=1
                          local.get 0
                          i32.const 1
                          i32.store8
                          br 10 (;@1;)
                        end
                        i32.const 1049343
                        i32.const 40
                        i32.const 1048956
                        call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
                        unreachable
                      end
                      local.get 1
                      i64.load8_u
                      local.set 4
                      br 7 (;@2;)
                    end
                    local.get 1
                    i64.load8_u
                    i64.const 8
                    i64.shl
                    local.get 1
                    i64.load8_u offset=1
                    i64.or
                    local.set 4
                    br 6 (;@2;)
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
                  br 5 (;@2;)
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
                br 4 (;@2;)
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
              br 3 (;@2;)
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
            br 2 (;@2;)
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
          br 1 (;@2;)
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
    global.set 0)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE (type 7) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 3
    i32.store8 offset=1
    local.get 0
    local.get 2
    i32.store8)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E (type 6) (param i32) (result i64)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 8
    i32.add
    i64.load)
  (func $_ZN15svm_abi_decoder7decoder7Decoder9decode_i817h61d1a1dfe4440be9E (type 2) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 1
    global.set 0
    i32.const 1
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
        local.tee 3
        i32.const 255
        i32.and
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 3
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
        drop
        local.get 1
        i32.const 16
        i32.add
        local.get 0
        i32.const 1
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E
        block  ;; label = @3
          local.get 1
          i32.load8_u offset=16
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 32
          i32.add
          i32.const 8
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 1
          local.get 1
          i64.load offset=16
          i64.store offset=32
          local.get 1
          i32.const 32
          i32.add
          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
          i32.wrap_i64
          local.set 0
          i32.const 0
          local.set 2
          br 2 (;@1;)
        end
        i32.const 1
        local.set 2
        local.get 1
        i32.const 8
        i32.add
        i32.const 1
        local.get 1
        i32.load8_u offset=17
        local.get 1
        i32.load8_u offset=18
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
        local.get 1
        i32.load8_u offset=9
        local.set 3
        local.get 1
        i32.load8_u offset=8
        local.set 0
        br 1 (;@1;)
      end
      local.get 1
      local.get 3
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
      local.get 1
      i32.load8_u offset=1
      local.set 3
      local.get 1
      i32.load8_u
      local.set 0
    end
    local.get 1
    i32.const 48
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
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17ha75a77b107b60e57E (type 2) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 16776960
    i32.and
    i32.const 8
    i32.shr_u)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h329d37ca6b8370f0E (type 1) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i1617h543a552aefc49ae0E (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 48
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
              call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
              local.tee 3
              i32.const 255
              i32.and
              i32.const 1
              i32.eq
              br_if 0 (;@5;)
              local.get 3
              call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
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
              br_table 3 (;@2;) 2 (;@3;) 3 (;@2;) 2 (;@3;) 1 (;@4;)
            end
            local.get 1
            local.get 3
            call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
            i32.const 1
            local.set 0
            i32.const 0
            local.set 3
            local.get 1
            i32.load8_u offset=1
            local.set 2
            local.get 1
            i32.load8_u
            local.set 4
            br 3 (;@1;)
          end
          i32.const 1049343
          i32.const 40
          i32.const 1048940
          call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
          unreachable
        end
        i32.const 2
        local.set 2
      end
      local.get 1
      i32.const 16
      i32.add
      local.get 0
      local.get 2
      call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E
      i32.const 1
      local.set 0
      block  ;; label = @2
        local.get 1
        i32.load8_u offset=16
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.const 32
        i32.add
        i32.const 8
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 1
        local.get 1
        i64.load offset=16
        i64.store offset=32
        local.get 1
        i32.const 32
        i32.add
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
        i32.wrap_i64
        local.tee 2
        i32.const 65280
        i32.and
        local.set 3
        i32.const 0
        local.set 0
        br 1 (;@1;)
      end
      local.get 1
      i32.const 8
      i32.add
      i32.const 1
      local.get 1
      i32.load8_u offset=17
      local.get 1
      i32.load8_u offset=18
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
      local.get 1
      i32.load8_u offset=9
      local.set 2
      local.get 1
      i32.load8_u offset=8
      local.set 4
      i32.const 0
      local.set 3
    end
    local.get 1
    i32.const 48
    i32.add
    global.set 0
    local.get 4
    i32.const 255
    i32.and
    i32.const 8
    i32.shl
    local.get 0
    i32.or
    local.get 3
    local.get 2
    i32.const 255
    i32.and
    i32.or
    i32.const 16
    i32.shl
    i32.or)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h9434170550c1e1daE (type 2) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 16
    i32.shr_u)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h8022a205287793acE (type 1) (param i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i3217h88974b4922d1e3f2E (type 6) (param i32) (result i64)
    (local i32 i32 i32 i64 i64)
    global.get 0
    i32.const 48
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
              block  ;; label = @6
                block  ;; label = @7
                  local.get 0
                  call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
                  local.tee 3
                  i32.const 255
                  i32.and
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
                  i32.const -3
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
                  br_table 5 (;@2;) 2 (;@5;) 3 (;@4;) 4 (;@3;) 5 (;@2;) 2 (;@5;) 3 (;@4;) 4 (;@3;) 1 (;@6;)
                end
                local.get 1
                local.get 3
                call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
                i64.const 1
                local.set 4
                i64.const 0
                local.set 5
                local.get 1
                i32.load8_u offset=1
                local.set 0
                local.get 1
                i32.load8_u
                local.set 3
                br 5 (;@1;)
              end
              i32.const 1049343
              i32.const 40
              i32.const 1048924
              call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
              unreachable
            end
            i32.const 2
            local.set 2
            br 2 (;@2;)
          end
          i32.const 3
          local.set 2
          br 1 (;@2;)
        end
        i32.const 4
        local.set 2
      end
      local.get 1
      i32.const 16
      i32.add
      local.get 0
      local.get 2
      call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E
      block  ;; label = @2
        local.get 1
        i32.load8_u offset=16
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.const 32
        i32.add
        i32.const 8
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 1
        local.get 1
        i64.load offset=16
        i64.store offset=32
        local.get 1
        i32.const 32
        i32.add
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
        i64.const 32
        i64.shl
        local.set 5
        i64.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      i32.const 8
      i32.add
      i32.const 1
      local.get 1
      i32.load8_u offset=17
      local.get 1
      i32.load8_u offset=18
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
      local.get 1
      i32.load8_u offset=9
      local.set 0
      local.get 1
      i32.load8_u offset=8
      local.set 3
      i64.const 1
      local.set 4
      i64.const 0
      local.set 5
    end
    local.get 1
    i32.const 48
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
    local.get 3
    i64.extend_i32_u
    i64.const 255
    i64.and
    i64.const 8
    i64.shl
    i64.or)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h6762501a24f861ddE (type 10) (param i64) (result i32)
    block  ;; label = @1
      local.get 0
      i64.const 1
      i64.and
      i64.eqz
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i64.const 32
    i64.shr_u
    i32.wrap_i64)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hbd78294bab8fc663E (type 9) (param i32 i64)
    block  ;; label = @1
      local.get 1
      i32.wrap_i64
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
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
  (func $_ZN15svm_abi_decoder7decoder7Decoder10decode_i6417h6870fd9fd42a67aeE (type 1) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN15svm_abi_decoder7decoder7Decoder9read_byte17h21ec604596f69413E
        local.tee 4
        i32.const 255
        i32.and
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h4dfab9955cc6c517E
          i32.const 255
          i32.and
          local.tee 4
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
                        local.get 4
                        i32.const -20
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 4
                        i32.const -36
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 1 (;@9;)
                        local.get 4
                        i32.const -52
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 2 (;@8;)
                        local.get 4
                        i32.const -68
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 3 (;@7;)
                        local.get 4
                        i32.const -84
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 4 (;@6;)
                        local.get 4
                        i32.const -100
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 5 (;@5;)
                        local.get 4
                        i32.const -116
                        i32.add
                        i32.const 2
                        i32.lt_u
                        br_if 6 (;@4;)
                        i32.const 1049343
                        i32.const 40
                        i32.const 1048840
                        call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
                        unreachable
                      end
                      i32.const 2
                      local.set 3
                      br 6 (;@3;)
                    end
                    i32.const 3
                    local.set 3
                    br 5 (;@3;)
                  end
                  i32.const 4
                  local.set 3
                  br 4 (;@3;)
                end
                i32.const 5
                local.set 3
                br 3 (;@3;)
              end
              i32.const 6
              local.set 3
              br 2 (;@3;)
            end
            i32.const 7
            local.set 3
            br 1 (;@3;)
          end
          i32.const 8
          local.set 3
        end
        local.get 2
        i32.const 16
        i32.add
        local.get 1
        local.get 3
        call $_ZN15svm_abi_decoder7decoder7Decoder8read_num17hd57ef9494a153fd8E
        block  ;; label = @3
          local.get 2
          i32.load8_u offset=16
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 32
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.const 16
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=16
          i64.store offset=32
          local.get 2
          i32.const 32
          i32.add
          call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h94af4025f3c7dd56E
          local.set 5
          local.get 0
          i32.const 0
          i32.store8
          local.get 0
          i32.const 8
          i32.add
          local.get 5
          i64.store
          br 2 (;@1;)
        end
        local.get 2
        i32.const 8
        i32.add
        i32.const 1
        local.get 2
        i32.load8_u offset=17
        local.get 2
        i32.load8_u offset=18
        call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hc0bbb41f48ff998cE
        local.get 0
        local.get 2
        i32.load16_u offset=8
        i32.store16 offset=1 align=1
        local.get 0
        i32.const 1
        i32.store8
        br 1 (;@1;)
      end
      local.get 2
      local.get 4
      call $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17h614f58e3afa2a7eaE
      local.get 0
      local.get 2
      i32.load16_u
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 1
      i32.store8
    end
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17hcd93f9c9a920f35eE (type 6) (param i32) (result i64)
    block  ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i32.const 8
    i32.add
    i64.load)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$10unwrap_err17hd6856279f96aa83aE (type 7) (param i32 i32 i32 i32)
    block  ;; label = @1
      local.get 1
      i32.const 255
      i32.and
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048744
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 3
    i32.store8 offset=1
    local.get 0
    local.get 2
    i32.store8)
  (func $_ZN11svm_sdk_std6result19Result$LT$T$C$E$GT$6unwrap17h037337940127482eE (type 10) (param i64) (result i32)
    block  ;; label = @1
      local.get 0
      i64.const 1
      i64.and
      i64.eqz
      br_if 0 (;@1;)
      i32.const 1049248
      i32.const 14
      i32.const 1048824
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    i64.const 32
    i64.shr_u
    i32.wrap_i64)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049485))
  (global (;2;) i32 (i32.const 1049485))
  (export "memory" (memory 0))
  (export "svm_alloc" (func $svm_alloc))
  (export "initialize" (func $initialize))
  (export "store_addr" (func $store_addr))
  (export "load_addr" (func $load_addr))
  (export "svm_fund" (func $svm_fund))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17h0375e219d24c8f94E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h103f0123c0f85d75E)
  (data (;0;) (i32.const 1048576) "\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00L\00\10\00\1c\00\00\00\ec\01\00\00\1e\00\00\00library/std/src/panicking.rs\18\01\10\00D\00\00\00N\02\00\00\11\00\00\00\18\01\10\00D\00\00\00>\01\00\00\12\00\00\00\18\01\10\00D\00\00\00\81\00\00\00\12\00\00\00\18\01\10\00D\00\00\00\9b\00\00\00\12\00\00\00\b8\00\10\00?\00\00\00!\00\00\00\1d\00\00\00/home/yaronwittenstein/work/sm/svm/crates/sdk/std/src/result.rs\00\b8\00\10\00?\00\00\00\19\00\00\00\1e\00\00\00\18\01\10\00D\00\00\00\ea\00\00\00\12\00\00\00/home/yaronwittenstein/work/sm/svm/crates/abi/decoder/src/decoder.rs\18\01\10\00D\00\00\00\d0\00\00\00\12\00\00\00\18\01\10\00D\00\00\00\ba\00\00\00\12\00\00\00\18\01\10\00D\00\00\00\fb\01\00\00\12\00\00\00\00\00\00\00attempt to add with overflow\11\02\10\00C\00\00\00C\00\00\00\09\00\00\00\00\00\00\00attempt to add with overflow\11\02\10\00C\00\00\00N\00\00\00\1d\00\00\00\00\00\00\00attempt to subtract with overflow/home/yaronwittenstein/work/sm/svm/crates/abi/decoder/src/cursor.rsd\02\10\00<\00\00\00\19\00\00\00\09\00\00\00/home/yaronwittenstein/work/sm/svm/crates/sdk/std/src/vec.rsexplicit panic\00\00\c0\02\10\00?\00\00\00\0e\00\00\00\1b\00\00\00/home/yaronwittenstein/work/sm/svm/crates/sdk/std/src/option.rsinternal error: entered unreachable code\008\03\10\00@\00\00\00\ac\00\00\00\01\00\00\00/home/yaronwittenstein/work/sm/svm/crates/sdk/types/src/value.rs"))
