extern crate svm_wasmer_c_api;

use svm_wasmer_c_api::include_svm_wasmer_instance_api;

use std::slice;
use std::sync::Arc;
use svm_common::Address;

use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};
use svm_wasmer::*;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_t, wasmer_import_t},
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime::{Func, ImportObject};
use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

// Injects in this file the `svm wasmer` instance API backed by: (`MemKVStore, MemPages, MemPageCache32`)
include_svm_wasmer_instance_api!(MemKVStore, MemPages, MemPageCache32);

/// Represents a fake `Node`
#[repr(C)]
struct NodeData {
    ip: [u8; 4],
    os: String,
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            ip: [0; 4],
            os: "max".to_string(),
        }
    }
}

/// Represents a fake `get_balance` node vmcall
#[no_mangle]
unsafe extern "C" fn get_balance(_ctx: &wasmer_runtime::Ctx, addr: i32) -> i64 {
    return (addr + 100) as i64;
}

fn cast_str_to_wasmer_byte_array(s: &str) -> wasmer_byte_array {
    let bytes: &[u8] = s.as_bytes();
    let bytes_ptr: *const u8 = bytes.as_ptr();
    let bytes_len: u32 = bytes.len() as u32;

    std::mem::forget(bytes);

    wasmer_byte_array {
        bytes: bytes_ptr,
        bytes_len,
    }
}

unsafe fn cast_wasmer_byte_array_to_string(wasmer_bytes: &wasmer_byte_array) -> String {
    let slice: &[u8] = slice::from_raw_parts(wasmer_bytes.bytes, wasmer_bytes.bytes_len as usize);

    if let Ok(s) = std::str::from_utf8(slice) {
        s.to_string()
    } else {
        panic!("error converting `wasmer_byte_array` to string")
    }
}

fn us32_addr_as_ptr(addr: u32) -> *const u8 {
    Address::from(addr).as_ptr()
}

fn node_data_as_ptr(node_data: &NodeData) -> *const c_void {
    node_data as *const NodeData as *const _
}

macro_rules! cast_vmcall_to_import_func_t {
    ($func: path, $params: expr, $returns: expr) => {
        unsafe {
            let export = Box::new(Export::Function {
                func: FuncPointer::new($func as _),
                ctx: Context::Internal,
                signature: Arc::new(FuncSig::new($params, $returns)),
            });

            Box::into_raw(export) as *const wasmer_import_func_t
        }
    };
}

macro_rules! wasmer_compile_module {
    ($wasm:expr) => {{
        let wasm = wabt::wat2wasm(&$wasm).unwrap();
        wasmer_runtime::compile(&wasm).unwrap()
    }};
}

macro_rules! wasmer_compile_module_file {
    ($file:expr) => {{
        let wasm = include_str!($file);
        wasmer_compile_module!(wasm)
    }};
}

fn build_wasmer_import_t(
    mode_name: &str,
    import_name: &str,
    func: *const wasmer_import_func_t,
) -> wasmer_import_t {
    wasmer_import_t {
        module_name: cast_str_to_wasmer_byte_array(mode_name),
        import_name: cast_str_to_wasmer_byte_array(import_name),
        tag: wasmer_import_export_kind::WASM_FUNCTION,
        value: wasmer_import_export_value { func },
    }
}

fn alloc_import_obj_ptr_ptr() -> *mut *mut c_void {
    let mut import_object_inner: *mut c_void =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };
    &mut import_object_inner as *mut _
}

#[test]
fn cast_string_to_wasmer_by_array() {
    let module_bytes = cast_str_to_wasmer_byte_array("env");
    let module_str = unsafe { cast_wasmer_byte_array_to_string(&module_bytes) };

    assert_eq!("env", module_str.as_str());
}

#[test]
fn call_node_get_balance() {
    let node_data = NodeData::default();
    let gb_ptr = cast_vmcall_to_import_func_t!(get_balance, vec![Type::I32], vec![Type::I64]);
    let mut gb_import = build_wasmer_import_t("node", "get_balance", gb_ptr);

    let import_obj: &mut ImportObject;
    let import_obj_ptr_ptr = alloc_import_obj_ptr_ptr();

    unsafe {
        wasmer_svm_import_object(
            import_obj_ptr_ptr,
            us32_addr_as_ptr(0x11_22_33_44), // `addr_ptr: *const u8
            5,                               // `max_pages: libc::c_int`
            100,                             // `max_pages_slices: libc::c_int`
            node_data_as_ptr(&node_data),    // node_data_ptr:: *const c_void
            &mut gb_import as *mut _,        // `imports: *mut wasmer_import_t
            1,                               // `imports_len: libc::c_int`
        );

        import_obj = &mut *(*import_obj_ptr_ptr as *mut _);
    };

    let module = wasmer_compile_module_file!("wasm/get_balance.wast");
    let instance = module.instantiate(&import_obj).unwrap();
    let func: Func<i32, i64> = instance.func("get_balance_proxy").unwrap();
    let res = func.call(20).unwrap();
    assert_eq!(100 + 20, res);
}
