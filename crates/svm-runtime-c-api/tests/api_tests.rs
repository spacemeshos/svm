extern crate svm_runtime_c_api;

use svm_runtime_c_api as c_api;

use c_api::{helpers, testing, RuntimePtr};

use std::collections::HashMap;
use std::ffi::c_void;

use svm_common::{Address, State};
use svm_contract::wasm::WasmArgValue;
use svm_runtime::traits::Runtime;
use svm_storage::{
    page::{PageIndex, PageOffset, PageSliceLayout},
    ContractStorage,
};

use wasmer_runtime_c_api::instance::wasmer_instance_context_t;
use wasmer_runtime_core::types::Type;

/// Represents a fake `Host`
#[repr(C)]
struct Host {
    balance: HashMap<Address, i64>,
}

impl Host {
    pub fn set_balance(&mut self, addr: &Address, balance: i64) {
        self.balance.insert(addr.clone(), balance);
    }

    pub fn get_balance(&self, addr: &Address) -> i64 {
        if let Some(balance) = self.balance.get(addr) {
            *balance
        } else {
            0
        }
    }
}

impl Default for Host {
    fn default() -> Self {
        Self {
            balance: Default::default(),
        }
    }
}

/// Represents a fake host vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_node_get_set_balance`
#[no_mangle]
unsafe fn vmcall_get_balance(
    raw_ctx: *mut wasmer_instance_context_t,
    reg_bits: i32,
    reg_idx: i32,
) -> i64 {
    assert_eq!(Address::len() * 8, reg_bits as usize);

    let reg = testing::svm_register_get(raw_ctx, reg_bits, reg_idx);
    let addr = Address::from(reg);

    let host = testing::svm_node_data_get::<Host>(raw_ctx);
    host.get_balance(&addr)
}

/// Represents a fake host vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_node_get_set_balance`
#[no_mangle]
unsafe fn vmcall_set_balance(
    raw_ctx: *mut wasmer_instance_context_t,
    balance: i64,
    reg_bits: i32,
    reg_idx: i32,
) {
    let reg = testing::svm_register_get(raw_ctx, reg_bits, reg_idx);
    let addr = Address::from(reg);

    let host = testing::svm_node_data_get::<Host>(raw_ctx);
    host.set_balance(&addr, balance);
}

#[test]
fn runtime_c_transaction_exec_changing_state() {
    unsafe { transaction_exec_changing_state() }
}

unsafe fn transaction_exec_changing_state() {
    let host = Host::default();
    let mut raw_runtime: *mut *mut c_void = testing::alloc_ptr();
    let mut raw_contract = testing::alloc_ptr();
    // let raw_import_object = testing::alloc_ptr();

    let author_addr = 0_10_20_30_40;
    let wasm = include_str!("wasm/store.wast");

    // 1) deploy
    let kv = svm_runtime::testing::memory_kv_store_init();
    let runtime = svm_runtime::testing::create_memory_runtime(&kv);
    let boxed_runtime: Box<dyn Runtime> = Box::new(runtime);
    let runtime_ptr: RuntimePtr = RuntimePtr::new(boxed_runtime);
    let raw_runtime_ptr: *mut c_void = helpers::into_raw_mut(runtime_ptr);

    let runtime: &mut Box<dyn Runtime> = &mut *(raw_runtime_ptr as *mut RuntimePtr);
    runtime.contract_build(&[]);

    //
    //
    // use svm_contract::memory::MemoryEnv;
    // let raw_runtime: *mut c_void =
    //     helpers::into_raw_mut::<svm_runtime::DefaultRuntime<MemoryEnv>>(runtime);

    // let runtime = &mut *(raw_runtime as *mut svm_runtime::DefaultRuntime<MemoryEnv>);
    // runtime.contract_build(&[]);
    //
    //
    //
    //

    // let raw_runtime: *mut c_void =
    //     helpers::into_raw_mut::<svm_runtime::DefaultRuntime<MemoryEnv>>(runtime);
    // let runtime = &mut *(raw_runtime as *mut svm_runtime::DefaultRuntime<MemoryEnv>);
    // runtime.contract_build(&[]);

    //
    //
    //
    //
    // let _ = c_api::svm_runtime_create(raw_runtime);
    // TODO: assert runtime has been created successfully

    // let bytes = svm_runtime::testing::build_raw_contract(0, "Sample Contract", author_addr, wasm);
    // let runtime = helpers::cast_to_runtime_mut(*raw_runtime);

    // let _ = c_api::svm_contract_build(
    //     *raw_runtime,
    //     raw_contract,
    //     bytes.as_ptr() as *const c_void,
    //     bytes.len() as u64,
    // );
    // TODO: assert `Contract` instance has been build successfully

    //     let raw_addr = svm_contract_compute_address(*raw_contract as _);
    //     let _ = svm_contract_store(*raw_contract as _, raw_addr);
    //
    //     // 2) execute
    //     let _res = svm_import_object(
    //         raw_import_object as _,
    //         raw_addr,                     // `raw_addr:  *const c_void`
    //         State::from(0).as_ptr() as _, // `raw_state: *const c_void`
    //         5,                            // `pages_count: libc::c_int`
    //         host_as_ptr(&host),           // `node_data_ptr:: *const c_void`
    //         std::ptr::null_mut(),         // `imports: *mut wasmer_import_t`
    //         0,                            // `imports_len: libc::c_int`
    //     );
    //
    //     let addr = Address::from(raw_addr);
    //     let sender = Address::from([0xAB; 20].as_ref());
    //
    //     let bytes = build_raw_tx!(
    //         addr.clone(),
    //         sender,
    //         "run",
    //         &[WasmArgValue::I64(0x10_20_30_40_50_60_70_80)]
    //     );
    //
    //     let raw_receipt = alloc_raw_receipt!();
    //     let raw_tx = alloc_raw_transaction!();
    //     let _ = svm_transaction_build(
    //         raw_tx as _,
    //         bytes.as_ptr() as *const c_void,
    //         bytes.len() as u64,
    //     );
    //     let _ = svm_transaction_exec(raw_receipt as _, *raw_tx as _, *raw_import_object as _);
    //
    //     assert_eq!(true, svm_receipt_status(*raw_receipt as _));
    //
    //     let new_state = svm_receipt_new_state(*raw_receipt as _);
    //     let new_state = State::from(new_state);
    //
    //     // 3) asserting data has been persisted as expected
    //     let pages_storage =
    //         svm_runtime::gen_rocksdb_pages_storage!(addr, new_state, 5, "tests-contract-storage");
    //     let page_cache = svm_runtime::gen_rocksdb_page_cache!(pages_storage, 5);
    //     let mut storage = ContractStorage::new(page_cache);
    //
    //     let slice_pos = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);
    //
    //     let slice = storage.read_page_slice(&slice_pos).unwrap();
    //     assert_eq!(
    //         &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
    //         &slice[..]
    //     );

    // TODO: clean:
    // * addr
    // * contract ??
    // * receipt
    // }
}

// #[test]
// fn runtime_node_vmcalls() {
//     unsafe {
//         let mut host = Host::default();
//         let raw_contract = alloc_raw_contract!();
//         let raw_import_object = alloc_raw_import_object!();
//         let author_addr = Address::from([0xFF; 20].as_ref());
//
//         // 1) deploy
//         let bytes = build_raw_contract!("wasm/mul_balance.wast", &author_addr);
//         let _ = svm_contract_build(
//             raw_contract as _,
//             bytes.as_ptr() as *const c_void,
//             bytes.len() as u64,
//         );
//         let raw_addr = svm_contract_compute_address(*raw_contract as _);
//         let _ = svm_contract_store(*raw_contract as _, raw_addr);
//
//         // 2) execute
//         let gb_ptr = cast_vmcall_to_import_func_t!(
//             vmcall_get_balance,
//             vec![Type::I32, Type::I32],
//             vec![Type::I64]
//         );
//
//         let sb_ptr = cast_vmcall_to_import_func_t!(
//             vmcall_set_balance,
//             vec![Type::I64, Type::I32, Type::I32],
//             vec![]
//         );
//
//         let gb_import = build_wasmer_import_t("env", "vmcall_get_balance", gb_ptr);
//         let sb_import = build_wasmer_import_t("env", "vmcall_set_balance", sb_ptr);
//         let mut imports = [gb_import, sb_import];
//
//         let _res = svm_import_object(
//             raw_import_object as _,
//             raw_addr,                     // `raw_addr: *const u8`
//             State::from(0).as_ptr() as _, // `raw_state: *const u8`,
//             5,                            // `pages_count: libc::c_int`
//             host_as_ptr(&host),      // `node_data_ptr:: *const c_void`
//             imports.as_mut_ptr() as _,    // `imports: *mut wasmer_import_t`
//             imports.len() as _,           // `imports_len: libc::c_int`
//         );
//
//         let addr = Address::from(raw_addr);
//         let sender = Address::from([0xAB; 20].as_ref());
//
//         let bytes = build_raw_tx!(
//             addr.clone(),
//             sender,
//             "mul_balance",
//             &[WasmArgValue::I64(2)] // `balance` multiply-by factor
//         );
//
//         // we initialize account `0x00...10_20_30` with `balance = 100`
//         let balance_addr = Address::from(0x10_20_30);
//         host.set_balance(&balance_addr, 100);
//
//         let raw_receipt = alloc_raw_receipt!();
//         let raw_tx = alloc_raw_transaction!();
//         let _ = svm_transaction_build(
//             raw_tx as _,
//             bytes.as_ptr() as *const c_void,
//             bytes.len() as u64,
//         );
//         let _ = svm_transaction_exec(raw_receipt as _, *raw_tx as _, *raw_import_object as _);
//
//         // asserting account `0x00...10_20_30` new balance is `200 (= 100 x 2)`
//         assert_eq!(200, host.get_balance(&balance_addr));
//     }
// }
