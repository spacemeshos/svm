extern crate svm_runtime_c_api;

use std::ffi::c_void;

use svm_common::{Address, State};
use svm_storage::rocksdb::RocksMerklePageCache;

use svm_contract::wasm::WasmArgValue;

use svm_runtime::*;
use svm_runtime_c_api::*;

use svm_runtime_c_api::c_utils::*;
use svm_runtime_c_api::rocks_c_api::*;

use wasmer_runtime::{Ctx, Func, Instance};
use wasmer_runtime_c_api::{
    instance::{wasmer_instance_context_t, wasmer_module_import_instantiate},
    module::wasmer_module_t,
};
use wasmer_runtime_core::types::Type;

/// Represents a fake `Node`
#[repr(C)]
struct NodeData {
    pub(self) ip: [u8; 4],
    pub(self) os: String,
}

impl NodeData {
    fn set_ip(&mut self, ip: i32) {
        let ip = ip as u32;

        let d = ((ip >> 00) & 0xFF) as u8;
        let c = ((ip >> 08) & 0xFF) as u8;
        let b = ((ip >> 16) & 0xFF) as u8;
        let a = ((ip >> 24) & 0xFF) as u8;

        self.ip = [a, b, c, d];
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            ip: [0; 4],
            os: "max".to_string(),
        }
    }
}

/// Represents a fake node vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_node_get_balance`
#[no_mangle]
unsafe extern "C" fn get_balance(_ctx: *mut wasmer_instance_context_t, addr: i32) -> i64 {
    return (addr + 100) as i64;
}

/// Represents a fake node vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_wasmer_svm_instance_context_node_data_get`
#[no_mangle]
unsafe extern "C" fn set_ip(ctx: *mut wasmer_instance_context_t, new_ip: i32) {
    let node_data: *mut c_void = svm_instance_context_node_data_get(ctx) as *mut _;
    let node_data: &mut NodeData = &mut *(node_data as *mut _);
    node_data.set_ip(new_ip);
}

/// Represents a fake node vmcall implemented in another programming-language using the FFI interface.
/// See test: `call_wasmer_svm_register_ptr`
#[no_mangle]
unsafe extern "C" fn copy_reg_to_reg(
    ctx: *const wasmer_instance_context_t,
    src_reg_idx: i32,
    dst_reg_idx: i32,
) {
    let src_reg_ptr: *const u8 = svm_register_get(ctx, 64, src_reg_idx) as *const _;
    let dst_reg_ptr: *mut u8 = svm_register_get(ctx, 64, dst_reg_idx) as *mut _;

    std::ptr::copy_nonoverlapping(src_reg_ptr, dst_reg_ptr, 8);
}

#[cfg(test)]
fn u32_addr_as_ptr(addr: u32) -> *const c_void {
    use svm_common::Address;
    let addr = Box::new(Address::from(addr));
    let addr = Box::leak(addr);

    addr.as_ptr() as _
}

#[cfg(test)]
fn u32_state_as_ptr(state: u32) -> *const c_void {
    use svm_common::State;
    let state = Box::new(State::from(state));
    let state = Box::leak(state);

    state.as_ptr() as _
}

fn node_data_as_ptr(node_data: &NodeData) -> *const c_void {
    node_data as *const NodeData as *const _
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
fn call_storage_mem_to_reg_copy() {
    unsafe {
        let node_data = NodeData::default();
        let raw_contract = alloc_raw_contract();
        let raw_import_object = alloc_raw_import_object();
        let author_addr = Address::from([0xFF; 20].as_ref());

        let bytes = build_raw_contract!("wasm/mem_to_reg_copy.wast", &author_addr);
        let _ = svm_contract_build(raw_contract, bytes.as_ptr(), bytes.len() as u64);
        let raw_addr = svm_contract_compute_address(*raw_contract);
        let _ = svm_contract_store(*raw_contract, raw_addr);

        let _ = svm_import_object(
            raw_import_object,
            raw_addr,                     // `raw_addr: *const c_void`
            State::from(0).as_ptr() as _, // `raw_state: *const c_void`
            5,                            // `max_pages: libc::c_int`
            100,                          // `max_pages_slices: libc::c_int`
            node_data_as_ptr(&node_data), // `node_data_ptr:: *const c_void`
            std::ptr::null_mut(),         // `imports: *mut wasmer_import_t`
            0,                            // `imports_len: libc::c_int`
        );

        let addr = Address::from(raw_addr);
        let sender_addr = Address::from([0xAB; 20].as_ref());

        let bytes = build_raw_tx!(
            addr,
            sender_addr,
            "do_copy_to_reg",
            &[
                WasmArgValue::I32(0),
                WasmArgValue::I32(0),
                WasmArgValue::I32(4)
            ]
        );

        let raw_receipt = alloc_raw_receipt();
        let raw_tx = alloc_raw_transaction();
        let _ = svm_transaction_build(raw_tx, bytes.as_ptr(), bytes.len() as u64);
        let _ = svm_transaction_exec(raw_receipt, *raw_tx, *raw_import_object);

        // TODO: clean:
        // * addr
        // * contract ??
        // * receipt
    }
}

// #[test]
// fn call_node_get_balance() {
//     unsafe {
//         let node_data = NodeData::default();
//         let gb_ptr = cast_vmcall_to_import_func_t!(get_balance, vec![Type::I32], vec![Type::I64]);
//         let mut gb_import = build_wasmer_import_t("node", "get_balance", gb_ptr);
//         let raw_import_object = alloc_raw_import_object();
//
//         svm_import_object(
//             raw_import_object,
//             u32_addr_as_ptr(0x11_22_33_44),  // `raw_addr: *const u8`
//             u32_state_as_ptr(0x00_00_00_00), // `raw_state: *const u8`,
//             5,                               // `max_pages: libc::c_int`
//             100,                             // `max_pages_slices: libc::c_int`
//             node_data_as_ptr(&node_data),    // `node_data_ptr:: *const c_void`
//             &mut gb_import as *mut _,        // `imports: *mut wasmer_import_t`
//             1,                               // `imports_len: libc::c_int`
//         );
//
//         let import_object = deref_import_obj!(raw_import_object);
//         let raw_instance = alloc_raw_instance();
//         let module = wasmer_compile_module_file!("wasm/get_balance.wast");
//         let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
//         let instance: &Instance = deref_instance!(raw_instance);
//
//         let func: Func<i32, i64> = instance.func("get_balance_proxy").unwrap();
//         let res = func.call(20).unwrap();
//         assert_eq!(100 + 20, res);
//
//         let instance = instance as *const Instance;
//         Box::from_raw(instance as *mut Instance);
//     }
// }

// #[test]
// fn call_wasmer_svm_instance_context_node_data_get() {
//     unsafe {
//         let node_data = NodeData::default();
//         let set_ip_ptr = cast_vmcall_to_import_func_t!(set_ip, vec![Type::I32], vec![]);
//         let mut set_ip_import = build_wasmer_import_t("node", "set_ip", set_ip_ptr);
//
//         let raw_import_object = alloc_raw_import_object();
//
//         svm_import_object(
//             raw_import_object,
//             u32_addr_as_ptr(0x11_22_33_44),  // `raw_addr: *const u8`
//             u32_state_as_ptr(0x00_00_00_00), // `raw_state: *const u8`,
//             5,                               // `max_pages: libc::c_int`
//             100,                             // `max_pages_slices: libc::c_int`
//             node_data_as_ptr(&node_data),    // `node_data_ptr:: *const c_void`
//             &mut set_ip_import as *mut _,    // `imports: *mut wasmer_import_t`
//             1,                               // `imports_len: libc::c_int`
//         );
//
//         let import_object = deref_import_obj!(raw_import_object);
//         let raw_instance = alloc_raw_instance();
//         let module = wasmer_compile_module_file!("wasm/set_ip.wast");
//
//         let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
//         let instance: &Instance = deref_instance!(raw_instance);
//         let func: Func<i32> = instance.func("set_ip_proxy").unwrap();
//
//         assert_eq!([0, 0, 0, 0], node_data.ip);
//         let _ = func.call(0x10_20_30_40).unwrap();
//         assert_eq!([0x10, 0x20, 0x30, 0x40], node_data.ip);
//     }
// }
//
// #[test]
// fn call_wasmer_svm_register_get_set() {
//     unsafe {
//         let copy_reg2reg_ptr =
//             cast_vmcall_to_import_func_t!(copy_reg_to_reg, vec![Type::I32, Type::I32], vec![]);
//
//         let mut copy_reg2reg_import =
//             build_wasmer_import_t("node", "copy_reg_to_reg", copy_reg2reg_ptr);
//
//         let raw_import_object = alloc_raw_import_object();
//
//         svm_import_object(
//             raw_import_object,
//             u32_addr_as_ptr(0x11_22_33_44),  // `raw_addr: *const u8`
//             u32_state_as_ptr(0x00_00_00_00), // `raw_state: *const u8`,
//             5,                               // `max_pages: libc::c_int`
//             100,                             // `max_pages_slices: libc::c_int`
//             std::ptr::null(),                // `node_data_ptr:: *const c_void`
//             &mut copy_reg2reg_import as *mut _, // `imports: *mut wasmer_import_t`
//             1,                               // `imports_len: libc::c_int`
//         );
//
//         let import_object = deref_import_obj!(raw_import_object);
//         let raw_instance = alloc_raw_instance();
//         let module = wasmer_compile_module_file!("wasm/copy_reg_to_reg.wast");
//
//         let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
//         let instance: &Instance = deref_instance!(raw_instance);
//         let func: Func<(i32, i32)> = instance.func("copy_reg_to_reg_proxy").unwrap();
//
//         let ctx = instance.context() as *const Ctx as *const wasmer_instance_context_t;
//         let reg2 = svm_register_get(ctx, 64, 2);
//         let reg3 = wasmer_ctx_reg!(instance.context(), 64, 3, RocksPageCache);
//
//         // setting register `2` with data that will be copied later to register `3`
//         let buf: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80];
//         svm_register_set(ctx, 64, 2, buf.as_ptr() as *const c_void, 8);
//
//         assert_eq!(vec![0; 8], reg3.view());
//
//         // should trigger copying the contents of register `2` to register `3`
//         let _ = func.call(2, 3).unwrap();
//
//         assert_eq!(vec![10, 20, 30, 40, 50, 60, 70, 80], reg3.view());
//     }
// }
