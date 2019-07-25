extern crate svm_wasmer_c_api;

use svm_wasmer_c_api::include_svm_wasmer_instance_api;

use std::sync::Arc;
use svm_common::Address;

use svm_storage::memory::{MemKVStore, MemPageCache32, MemPages};
use svm_wasmer::*;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_t, wasmer_import_t},
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime::ImportObject;
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

/// Represents a fake `get_balance` node vmcall
#[no_mangle]
unsafe extern "C" fn get_balance(_ctx: &wasmer_runtime::Ctx, addr: i32) -> i64 {
    return (addr + 100) as i64;
}

fn cast_str_to_wasmer_byte_array(s: &str) -> wasmer_byte_array {
    let bytes_vec = s.bytes();
    let bytes_len: u32 = bytes_vec.len() as u32;

    let boxed_bytes = Box::new(bytes_vec);
    let bytes: *const u8 = Box::into_raw(boxed_bytes) as *const u8;

    wasmer_byte_array { bytes, bytes_len }
}

macro_rules! svm_vmcall_as_wasmer_import_func_t {
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

#[test]
fn create_import_object() {
    let node_data = NodeData {
        ip: [192, 168, 1, 10],
        os: String::from("mac"),
    };

    let addr_ptr: *const u8 = Address::from(0x11_22_33_44).as_ptr();
    let node_data_ptr: *const c_void = &node_data as *const NodeData as *const _;

    let params = vec![Type::I32];
    let returns = vec![Type::I64];
    let get_balance_ptr: *const wasmer_import_func_t =
        svm_vmcall_as_wasmer_import_func_t!(get_balance, params, returns);

    let mut get_balance_import = wasmer_import_t {
        module_name: cast_str_to_wasmer_byte_array("env"),
        import_name: cast_str_to_wasmer_byte_array("get_balance"),
        tag: wasmer_import_export_kind::WASM_FUNCTION,
        value: wasmer_import_export_value {
            func: get_balance_ptr,
        },
    };

    let imports: *mut wasmer_import_t = &mut get_balance_import as *mut _;
    let imports_len: libc::c_int = 1;

    unsafe {
        let mut import_object_inner: *mut c_void = std::mem::MaybeUninit::uninit().assume_init();

        let import_object_ptr: *mut *mut c_void = &mut import_object_inner as *mut _;

        wasmer_svm_import_object(
            import_object_ptr,
            addr_ptr,
            node_data_ptr,
            imports,
            imports_len,
        );

        let import_obj: &mut ImportObject = &mut *(*import_object_ptr as *mut _);

        let ns = import_obj.get_namespace("svm").unwrap();
        let export: Export = ns.get_export("mem_to_reg_copy").unwrap();

        match export {
            Export::Function { func: func, .. } => {
                let expected = FuncPointer::new(mem_to_reg_copy as *const _).inner();
                let actual = func.inner();

                assert_eq!(expected, actual);
            }
            _ => unreachable!(),
        }

        // let ns = import_obj.get_namespace("env").unwrap();
    }
}
