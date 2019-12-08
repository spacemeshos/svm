extern crate svm_runtime_c_api;

use std::collections::HashMap;
use std::ffi::c_void;

use svm_common::{Address, State};
use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};
use svm_storage::PageSliceCache;

use svm_contract::wasm::WasmArgValue;

use svm_runtime_c_api::*;

use svm_runtime_c_api::c_utils::*;
use svm_runtime_c_api::rocksdb_c_api::*;

use wasmer_runtime_c_api::instance::wasmer_instance_context_t;
use wasmer_runtime_core::types::Type;

/// Represents a fake `FullNode`
#[repr(C)]
struct FullNode {
    pub balance: HashMap<Address, i64>,
}

impl FullNode {
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

impl Default for FullNode {
    fn default() -> Self {
        Self {
            balance: Default::default(),
        }
    }
}

fn full_node_as_ptr(node: &FullNode) -> *const c_void {
    node as *const FullNode as *const _
}

/// Represents a fake node vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_node_get_set_balance`
#[no_mangle]
fn vmcall_get_balance(ctx: *mut wasmer_instance_context_t, reg_bits: i32, reg_idx: i32) -> i64 {
    unsafe {
        assert_eq!(Address::len() * 8, reg_bits as usize);

        let ptr: *const u8 = svm_register_get(ctx as _, reg_bits, reg_idx) as _;
        let addr = Address::from(ptr);

        let node: *const c_void = svm_instance_context_node_data_get(ctx as _);
        let node: &FullNode = &*(node as *const FullNode);

        node.get_balance(&addr)
    }
}

/// Represents a fake node vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_node_get_set_balance`
#[no_mangle]
fn vmcall_set_balance(
    ctx: *mut wasmer_instance_context_t,
    balance: i64,
    reg_bits: i32,
    reg_idx: i32,
) {
    unsafe {
        let ptr: *const u8 = svm_register_get(ctx as _, reg_bits, reg_idx) as _;
        let addr = Address::from(ptr);

        let node: *mut c_void = svm_instance_context_node_data_get(ctx as _) as _;
        let node: &mut FullNode = &mut *(node as *mut FullNode);

        node.set_balance(&addr, balance);
    }
}

macro_rules! build_raw_contract {
    ($file: expr, $author_addr: expr) => {{
        let wast = include_bytes!($file);
        let wasm = wabt::wat2wasm(wast.as_ref()).unwrap();

        svm_contract::build::WireContractBuilder::new()
            .with_version(0)
            .with_author($author_addr.clone())
            .with_code(wasm.as_slice())
            .with_name($file)
            .build()
    }};
}

macro_rules! build_raw_tx {
    ($contract_addr: expr, $sender_addr: expr, $func_name: expr, $func_args: expr) => {{
        svm_contract::build::WireTxBuilder::new()
            .with_version(0)
            .with_contract($contract_addr)
            .with_sender($sender_addr)
            .with_func_name($func_name)
            .with_func_args($func_args)
            .build()
    }};
}

#[test]
fn runtime_tx_exec_changing_state() {
    unsafe {
        let node = FullNode::default();
        let raw_contract = alloc_raw_contract!();
        let raw_import_object = alloc_raw_import_object!();
        let author_addr = Address::from([0xFF; 20].as_ref());

        // 1) deploy
        let bytes = build_raw_contract!("wasm/store.wast", &author_addr);
        let _ = svm_contract_build(
            raw_contract as _,
            bytes.as_ptr() as *const c_void,
            bytes.len() as u64,
        );
        let raw_addr = svm_contract_compute_address(*raw_contract as _);
        let _ = svm_contract_store(*raw_contract as _, raw_addr);

        // 2) execute
        let _res = svm_import_object(
            raw_import_object as _,
            raw_addr,                     // `raw_addr:  *const c_void`
            State::from(0).as_ptr() as _, // `raw_state: *const c_void`
            5,                            // `max_pages:  libc::c_int`
            full_node_as_ptr(&node),      // `node_data_ptr:: *const c_void`
            std::ptr::null_mut(),         // `imports: *mut wasmer_import_t`
            0,                            // `imports_len: libc::c_int`
        );

        let addr = Address::from(raw_addr);
        let sender = Address::from([0xAB; 20].as_ref());

        let bytes = build_raw_tx!(
            addr.clone(),
            sender,
            "run",
            &[WasmArgValue::I64(0x10_20_30_40_50_60_70_80)]
        );

        let raw_receipt = alloc_raw_receipt!();
        let raw_tx = alloc_raw_transaction!();
        let _ = svm_transaction_build(
            raw_tx as _,
            bytes.as_ptr() as *const c_void,
            bytes.len() as u64,
        );
        let _ = svm_transaction_exec(raw_receipt as _, *raw_tx as _, *raw_import_object as _);

        assert_eq!(true, svm_receipt_status(*raw_receipt as _));

        let new_state = svm_receipt_new_state(*raw_receipt as _);
        let new_state = State::from(new_state);

        // 3) asserting data has been persisted as expected
        let pages_storage =
            svm_runtime::gen_rocksdb_pages_storage!(addr, new_state, 5, "tests-contract-storage");
        let page_cache = svm_runtime::gen_rocksdb_page_cache!(pages_storage, 5);
        let mut storage = PageSliceCache::new(page_cache);

        let slice_pos = PageSliceLayout::new(PageIndex(0), PageOffset(0), 8);

        let slice = storage.read_page_slice(&slice_pos).unwrap();
        assert_eq!(
            &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
            &slice[..]
        );

        // TODO: clean:
        // * addr
        // * contract ??
        // * receipt
    }
}

#[test]
fn runtime_node_vmcalls() {
    unsafe {
        let mut node = FullNode::default();
        let raw_contract = alloc_raw_contract!();
        let raw_import_object = alloc_raw_import_object!();
        let author_addr = Address::from([0xFF; 20].as_ref());

        // 1) deploy
        let bytes = build_raw_contract!("wasm/mul_balance.wast", &author_addr);
        let _ = svm_contract_build(
            raw_contract as _,
            bytes.as_ptr() as *const c_void,
            bytes.len() as u64,
        );
        let raw_addr = svm_contract_compute_address(*raw_contract as _);
        let _ = svm_contract_store(*raw_contract as _, raw_addr);

        // 2) execute
        let gb_ptr = cast_vmcall_to_import_func_t!(
            vmcall_get_balance,
            vec![Type::I32, Type::I32],
            vec![Type::I64]
        );

        let sb_ptr = cast_vmcall_to_import_func_t!(
            vmcall_set_balance,
            vec![Type::I64, Type::I32, Type::I32],
            vec![]
        );

        let gb_import = build_wasmer_import_t("node", "vmcall_get_balance", gb_ptr);
        let sb_import = build_wasmer_import_t("node", "vmcall_set_balance", sb_ptr);
        let mut imports = [gb_import, sb_import];

        let _res = svm_import_object(
            raw_import_object as _,
            raw_addr,                     // `raw_addr: *const u8`
            State::from(0).as_ptr() as _, // `raw_state: *const u8`,
            5,                            // `max_pages: libc::c_int`
            full_node_as_ptr(&node),      // `node_data_ptr:: *const c_void`
            imports.as_mut_ptr() as _,    // `imports: *mut wasmer_import_t`
            imports.len() as _,           // `imports_len: libc::c_int`
        );

        let addr = Address::from(raw_addr);
        let sender = Address::from([0xAB; 20].as_ref());

        let bytes = build_raw_tx!(
            addr.clone(),
            sender,
            "mul_balance",
            &[WasmArgValue::I64(2)] // `balance` multiply-by factor
        );

        // we initialize account `0x00...10_20_30` with `balance = 100`
        let balance_addr = Address::from(0x10_20_30);
        node.set_balance(&balance_addr, 100);

        let raw_receipt = alloc_raw_receipt!();
        let raw_tx = alloc_raw_transaction!();
        let _ = svm_transaction_build(
            raw_tx as _,
            bytes.as_ptr() as *const c_void,
            bytes.len() as u64,
        );
        let _ = svm_transaction_exec(raw_receipt as _, *raw_tx as _, *raw_import_object as _);

        // asserting account `0x00...10_20_30` new balance is `200 (= 100 x 2)`
        assert_eq!(200, node.get_balance(&balance_addr));
    }
}
