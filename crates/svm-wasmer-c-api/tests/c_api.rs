extern crate svm_wasmer_c_api;

use svm_wasmer_c_api::mem_c_api::*;

use std::ffi::c_void;
use svm_storage::memory::MemPageCache32;
use svm_wasmer::*;

use wasmer_runtime_c_api::{
    export::{wasmer_import_export_kind, wasmer_import_export_value},
    import::{wasmer_import_func_t, wasmer_import_object_t, wasmer_import_t},
    instance::{wasmer_instance_context_t, wasmer_instance_t, wasmer_module_import_instantiate},
    module::wasmer_module_t,
    wasmer_byte_array, wasmer_result_t,
};

use wasmer_runtime::{Ctx, Func, ImportObject, Instance};
use wasmer_runtime_core::{
    export::{Context, Export, FuncPointer},
    types::{FuncSig, Type},
};

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
    let node_data: *mut c_void = wasmer_svm_instance_context_node_data_get(ctx) as *mut _;
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
    let src_reg_ptr: *const u8 = wasmer_svm_register_get(ctx, src_reg_idx);
    let dst_reg_ptr: *mut u8 = wasmer_svm_register_get(ctx, dst_reg_idx) as *mut _;

    std::ptr::copy_nonoverlapping(src_reg_ptr, dst_reg_ptr, 8);
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
    let slice: &[u8] =
        std::slice::from_raw_parts(wasmer_bytes.bytes, wasmer_bytes.bytes_len as usize);

    if let Ok(s) = std::str::from_utf8(slice) {
        s.to_string()
    } else {
        panic!("error converting `wasmer_byte_array` to string")
    }
}

fn u32_addr_as_ptr(addr: u32) -> *const u8 {
    use svm_common::Address;
    Address::from(addr).as_ptr()
}

fn node_data_as_ptr(node_data: &NodeData) -> *const c_void {
    node_data as *const NodeData as *const _
}

macro_rules! cast_vmcall_to_import_func_t {
    ($func: path, $params: expr, $returns: expr) => {{
        use std::sync::Arc;

        let export = Box::new(Export::Function {
            func: FuncPointer::new($func as _),
            ctx: Context::Internal,
            signature: Arc::new(FuncSig::new($params, $returns)),
        });

        Box::into_raw(export) as *const wasmer_import_func_t
    }};
}

macro_rules! wasmer_compile_module {
    ($wasm:expr) => {{
        let mut wasm = wabt::wat2wasm(&$wasm).unwrap();

        let wasm_bytes = wasm.as_mut_ptr();
        let wasm_bytes_len = wasm.len() as u32;
        let raw_module = alloc_raw_module();
        let compile_res = wasmer_svm_compile(raw_module, wasm_bytes, wasm_bytes_len);

        // TODO: assert `compile_res` is OK`
        // assert_eq!(wasmer_result_t::WASMER_OK, compile_res);

        let module: *const wasmer_module_t = *raw_module as *const _;
        module
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

macro_rules! alloc_ptr_ptr {
    ($ptr_type: ident) => {{
        use std::alloc::Layout;

        let ptr_size: usize = std::mem::size_of::<*mut $ptr_type>();
        let layout = Layout::from_size_align(ptr_size, std::mem::align_of::<u8>()).unwrap();
        let mut ptr: *mut $ptr_type = unsafe { std::alloc::alloc(layout) as *mut _ };

        &mut ptr as *mut *mut $ptr_type
    }};
}

fn alloc_raw_module() -> *mut *mut wasmer_module_t {
    alloc_ptr_ptr!(wasmer_module_t)
}

fn alloc_raw_instance() -> *mut *mut wasmer_instance_t {
    alloc_ptr_ptr!(wasmer_instance_t)
}

fn alloc_raw_import_object() -> *mut *mut wasmer_import_object_t {
    alloc_ptr_ptr!(wasmer_import_object_t)
}

macro_rules! deref_import_obj {
    ($raw_import_object: expr) => {{
        let import_obj: &mut ImportObject = &mut *(*$raw_import_object as *mut _);
        import_obj as *const ImportObject as *const wasmer_import_object_t
    }};
}

macro_rules! deref_instance {
    ($raw_instance: expr) => {{
        &mut *(*$raw_instance as *mut _)
    }};
}

#[test]
fn cast_string_to_wasmer_by_array() {
    let module_bytes = cast_str_to_wasmer_byte_array("env");
    let module_str = unsafe { cast_wasmer_byte_array_to_string(&module_bytes) };

    assert_eq!("env", module_str.as_str());
}

#[test]
fn call_storage_mem_to_reg_copy() {
    unsafe {
        let node_data = NodeData::default();
        let raw_import_object = alloc_raw_import_object();

        wasmer_svm_import_object(
            raw_import_object,
            u32_addr_as_ptr(0x11_22_33_44), // `addr_ptr: *const u8`
            5,                              // `max_pages: libc::c_int`
            100,                            // `max_pages_slices: libc::c_int`
            node_data_as_ptr(&node_data),   // `node_data_ptr:: *const c_void`
            std::ptr::null_mut(),           // `imports: *mut wasmer_import_t`
            0,                              // `imports_len: libc::c_int`
        );

        let import_object = deref_import_obj!(raw_import_object);
        let raw_instance = alloc_raw_instance();
        let module = wasmer_compile_module_file!("wasm/mem_to_reg_copy.wast");

        let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
        let instance: &Instance = deref_instance!(raw_instance);

        // initializing memory #0 cells `200..203` with values `10, 20, 30` respectively
        wasmer_ctx_mem_cells_write!(instance.context(), 0, 200, &[10, 20, 30]);

        let func: Func<(i32, i32, i32)> = instance.func("do_copy_to_reg").unwrap();
        assert!(func.call(200, 3, 2).is_ok());

        // asserting register `2` content is `10, 20, 30, 0, ... 0`
        let reg = wasmer_ctx_reg!(instance.context(), 2, MemPageCache32);
        assert_eq!([10, 20, 30, 0, 0, 0, 0, 0], reg.view());
    }
}

#[test]
fn call_node_get_balance() {
    unsafe {
        let node_data = NodeData::default();
        let gb_ptr = cast_vmcall_to_import_func_t!(get_balance, vec![Type::I32], vec![Type::I64]);
        let mut gb_import = build_wasmer_import_t("node", "get_balance", gb_ptr);
        let raw_import_object = alloc_raw_import_object();

        wasmer_svm_import_object(
            raw_import_object,
            u32_addr_as_ptr(0x11_22_33_44), // `addr_ptr: *const u8`
            5,                              // `max_pages: libc::c_int`
            100,                            // `max_pages_slices: libc::c_int`
            node_data_as_ptr(&node_data),   // `node_data_ptr:: *const c_void`
            &mut gb_import as *mut _,       // `imports: *mut wasmer_import_t`
            1,                              // `imports_len: libc::c_int`
        );

        let import_object = deref_import_obj!(raw_import_object);
        let raw_instance = alloc_raw_instance();
        let module = wasmer_compile_module_file!("wasm/get_balance.wast");
        let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
        let instance: &Instance = deref_instance!(raw_instance);

        let func: Func<i32, i64> = instance.func("get_balance_proxy").unwrap();
        let res = func.call(20).unwrap();
        assert_eq!(100 + 20, res);
    }
}

#[test]
fn call_wasmer_svm_instance_context_node_data_get() {
    unsafe {
        let node_data = NodeData::default();
        let set_ip_ptr = cast_vmcall_to_import_func_t!(set_ip, vec![Type::I32], vec![]);
        let mut set_ip_import = build_wasmer_import_t("node", "set_ip", set_ip_ptr);

        let raw_import_object = alloc_raw_import_object();

        wasmer_svm_import_object(
            raw_import_object,
            u32_addr_as_ptr(0x11_22_33_44), // `addr_ptr: *const u8`
            5,                              // `max_pages: libc::c_int`
            100,                            // `max_pages_slices: libc::c_int`
            node_data_as_ptr(&node_data),   // `node_data_ptr:: *const c_void`
            &mut set_ip_import as *mut _,   // `imports: *mut wasmer_import_t`
            1,                              // `imports_len: libc::c_int`
        );

        let import_object = deref_import_obj!(raw_import_object);
        let raw_instance = alloc_raw_instance();
        let module = wasmer_compile_module_file!("wasm/set_ip.wast");

        let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
        let instance: &Instance = deref_instance!(raw_instance);
        let func: Func<i32> = instance.func("set_ip_proxy").unwrap();

        assert_eq!([0, 0, 0, 0], node_data.ip);
        let _ = func.call(0x10_20_30_40).unwrap();
        assert_eq!([0x10, 0x20, 0x30, 0x40], node_data.ip);
    }
}

#[test]
fn call_wasmer_svm_register_get_set() {
    unsafe {
        let copy_reg2reg_ptr =
            cast_vmcall_to_import_func_t!(copy_reg_to_reg, vec![Type::I32, Type::I32], vec![]);

        let mut copy_reg2reg_import =
            build_wasmer_import_t("node", "copy_reg_to_reg", copy_reg2reg_ptr);

        let raw_import_object = alloc_raw_import_object();

        wasmer_svm_import_object(
            raw_import_object,
            u32_addr_as_ptr(0x11_22_33_44), // `addr_ptr: *const u8`
            5,                              // `max_pages: libc::c_int`
            100,                            // `max_pages_slices: libc::c_int`
            std::ptr::null(),               // `node_data_ptr:: *const c_void`
            &mut copy_reg2reg_import as *mut _, // `imports: *mut wasmer_import_t`
            1,                              // `imports_len: libc::c_int`
        );

        let import_object = deref_import_obj!(raw_import_object);
        let raw_instance = alloc_raw_instance();
        let module = wasmer_compile_module_file!("wasm/copy_reg_to_reg.wast");

        let res = wasmer_module_import_instantiate(raw_instance, module, import_object);
        let instance: &Instance = deref_instance!(raw_instance);
        let func: Func<(i32, i32)> = instance.func("copy_reg_to_reg_proxy").unwrap();

        let ctx = instance.context() as *const Ctx as *const wasmer_instance_context_t;
        let reg2 = wasmer_svm_register_get(ctx, 2);
        let reg3 = wasmer_ctx_reg!(instance.context(), 3, MemPageCache32);

        //setting register `2` with data that will be copied later to register `3`
        wasmer_svm_register_set(ctx, 2, [10, 20, 30, 40, 50, 60, 70, 80].as_ptr(), 8);

        assert_eq!([0; 8], reg3.view());

        // should trigger copying the contents of register `2` to register `3`
        let _ = func.call(2, 3).unwrap();

        assert_eq!([10, 20, 30, 40, 50, 60, 70, 80], reg3.view());
    }
}
