(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (param i32) (result i64)))
  (import "svm" "svm_calldata_offset" (func $_ZN12svm_sdk_host3ext19svm_calldata_offset17h07d24f45c5ac567fE (type 0)))
  (import "svm" "svm_calldata_len" (func $_ZN12svm_sdk_host3ext16svm_calldata_len17h6d77304c58f043adE (type 0)))
  (import "svm" "svm_set_returndata" (func $_ZN12svm_sdk_host3ext18svm_set_returndata17hc3f114854661f072E (type 1)))
  (import "host" "host_fail" (func $_ZN28svm_runtime_examples_failure9host_fail17hc1706f871e2b5889E (type 2)))
  (func $_ZN73_$LT$svm_sdk_host..ext..ExtHost$u20$as$u20$svm_sdk_host..traits..Host$GT$12get_calldata17h9c3a1c3f928c46f6E (type 3) (param i32)
    (local i32)
    block  ;; label = @1
      i32.const 0
      i32.load8_u offset=1048796
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048796
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
      i32.load8_u offset=1048796
      br_if 0 (;@1;)
      i32.const 0
      i32.const 1
      i32.store8 offset=1048796
    end
    local.get 0
    local.get 1
    call $_ZN12svm_sdk_host3ext18svm_set_returndata17hc3f114854661f072E)
  (func $svm_alloc (type 4) (param i32) (result i32)
    i32.const 0)
  (func $initialize (type 2)
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
    i32.const 0
    i32.store offset=24
    local.get 0
    i64.const 42949672960000
    i64.store offset=16
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
  (func $_ZN15svm_abi_encoder5types4unit110_$LT$impl$u20$svm_abi_encoder..traits..Encoder$LT$svm_sdk_std..vec..Vec$LT$u8$GT$$GT$$u20$for$u20$$LP$$RP$$GT$6encode17hbc07c9d4026a86d9E (type 3) (param i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      local.get 0
      i32.load offset=4
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1048680
      i32.const 14
      i32.const 1048696
      call $_ZN4core9panicking5panic17hc64f9753b8a1be76E
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 1
    i32.add
    i32.store
    local.get 0
    i32.load offset=8
    local.get 1
    i32.add
    i32.const 48
    i32.store8)
  (func $fail (type 2)
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
    call $_ZN28svm_runtime_examples_failure9host_fail17hc1706f871e2b5889E
    local.get 0
    i32.const 0
    i32.store offset=24
    local.get 0
    i64.const 42949672960000
    i64.store offset=16
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
  (func $svm_fund (type 2))
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
  (func $_ZN3std9panicking20rust_panic_with_hook17h4e1267e42c34e062E (type 2)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=1048776
    i32.const 1
    i32.add
    i32.store offset=1048776
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          i32.const 0
          i32.load offset=1048784
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.const 0
          i32.load offset=1048788
          i32.const 1
          i32.add
          local.tee 0
          i32.store offset=1048788
          local.get 0
          i32.const 3
          i32.ge_u
          br_if 2 (;@1;)
          i32.const 0
          i32.load offset=1048792
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
        i64.store offset=1048784
        i32.const 0
        i32.load offset=1048792
        i32.const -1
        i32.le_s
        br_if 1 (;@1;)
      end
      call $rust_panic
      unreachable
    end
    unreachable
    unreachable)
  (func $rust_panic (type 2)
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6unwrap17h29d0a0f74d23cc07E (type 4) (param i32) (result i32)
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
  (table (;0;) 3 3 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048797))
  (global (;2;) i32 (i32.const 1048797))
  (export "memory" (memory 0))
  (export "svm_alloc" (func $svm_alloc))
  (export "initialize" (func $initialize))
  (export "fail" (func $fail))
  (export "svm_fund" (func $svm_fund))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN4core3ptr102drop_in_place$LT$$RF$core..iter..adapters..copied..Copied$LT$core..slice..iter..Iter$LT$u8$GT$$GT$$GT$17h0375e219d24c8f94E $_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h103f0123c0f85d75E)
  (data (;0;) (i32.const 1048576) "\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00called `Option::unwrap()` on a `None` value\00L\00\10\00\1c\00\00\00\ec\01\00\00\1e\00\00\00library/std/src/panicking.rsexplicit panic\00\00\88\00\10\00<\00\00\00\19\00\00\00\09\00\00\00/home/yaronwittenstein/work/sm/svm/crates/sdk/std/src/vec.rs"))
