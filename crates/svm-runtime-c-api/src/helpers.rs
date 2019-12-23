use std::ffi::c_void;
use std::slice;

use crate::RuntimePtr;
use svm_runtime::traits::Runtime;

use wasmer_runtime::{Global, Memory, Module, Table};
use wasmer_runtime_c_api::{
    export::wasmer_import_export_kind, import::wasmer_import_t, wasmer_result_t,
};
use wasmer_runtime_core::export::Export;

#[inline(always)]
pub fn into_raw<T>(obj: T) -> *const c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *const _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

#[inline(always)]
pub fn into_raw_mut<T>(obj: T) -> *mut c_void {
    let boxed_obj = Box::new(obj);
    let raw_obj_ptr: *mut _ = Box::into_raw(boxed_obj);

    raw_obj_ptr as _
}

#[inline(always)]
pub unsafe fn from_raw<'a, T>(raw_obj: *const c_void) -> &'a T {
    &*(raw_obj as *const T)
}

#[inline(always)]
pub unsafe fn from_raw_mut<'a, T>(raw_obj: *mut c_void) -> &'a mut T {
    &mut *(raw_obj as *mut T)
}

#[inline(always)]
pub unsafe fn cast_to_runtime<'a>(raw_runtime: *const c_void) -> &'a Box<dyn Runtime> {
    &*(raw_runtime as *const RuntimePtr)
}

#[inline(always)]
pub unsafe fn cast_to_runtime_mut<'a>(raw_runtime: *mut c_void) -> &'a mut Box<dyn Runtime> {
    &mut *(raw_runtime as *mut RuntimePtr)
}

pub unsafe fn cast_host_imports(
    imports: *mut c_void,
    imports_len: libc::c_uint,
) -> Vec<(String, String, Export)> {
    // function code extracted from `wasmer_import_object_extend` here:
    // https://github.com/wasmerio/wasmer/blob/f9bb579c05abc795d597a03352683fc62a4121d5/lib/runtime-c-api/src/import/mod.rs#L373

    let mut res: Vec<(String, String, Export)> = Vec::new();

    let imports: &[wasmer_import_t] = slice::from_raw_parts(imports as _, imports_len as usize);

    for import in imports {
        let module_name = slice::from_raw_parts(
            import.module_name.bytes,
            import.module_name.bytes_len as usize,
        );
        let module_name = if let Ok(s) = std::str::from_utf8(module_name) {
            s
        } else {
            panic!("error converting module name to string".to_string());
        };

        let import_name = slice::from_raw_parts(
            import.import_name.bytes,
            import.import_name.bytes_len as usize,
        );

        let import_name = if let Ok(s) = std::str::from_utf8(import_name) {
            s
        } else {
            panic!("error converting import_name to string".to_string());
        };

        let export = match import.tag {
            wasmer_import_export_kind::WASM_MEMORY => {
                let mem = import.value.memory as *mut Memory;
                Export::Memory((&*mem).clone())
            }
            wasmer_import_export_kind::WASM_FUNCTION => {
                let func_export = import.value.func as *mut Export;
                (&*func_export).clone()
            }
            wasmer_import_export_kind::WASM_GLOBAL => {
                let global = import.value.global as *mut Global;
                Export::Global((&*global).clone())
            }
            wasmer_import_export_kind::WASM_TABLE => {
                let table = import.value.table as *mut Table;
                Export::Table((&*table).clone())
            }
        };

        let export_tuple = (module_name.to_string(), import_name.to_string(), export);
        res.push(export_tuple);
    }

    res
}
