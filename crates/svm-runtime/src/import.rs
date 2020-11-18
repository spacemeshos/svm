use std::convert::{TryFrom, TryInto};
use std::ffi::c_void;

use crate::Context;

use svm_types::WasmType;

use wasmer::{Export, Exportable, Function, FunctionType, RuntimeError, Store, Type, Val};

use wasmer_c_api::wasm_c_api::{
    externals::{
        wasm_env_finalizer_t, wasm_extern_as_func, wasm_extern_t, wasm_func_as_extern,
        wasm_func_callback_with_env_t, wasm_func_new_with_env, wasm_func_t,
    },
    trap::wasm_trap_t,
    types::{wasm_functype_t, wasm_valkind_enum},
    value::{wasm_val_t, wasm_val_vec_t},
};

#[allow(non_camel_case_types)]
#[derive(Clone)]
#[repr(C)]
pub struct svm_env_t {
    pub inner_env: *const c_void,

    pub host_env: *const c_void,
}

impl svm_env_t {
    pub unsafe fn inner(&self) -> &Context {
        &*{ self.inner_env as *const Context }
    }

    pub unsafe fn inner_mut(&self) -> &mut Context {
        &mut *(self.inner_env as *const Context as *mut Context)
    }

    pub unsafe fn host_env<T>(&self) -> &T {
        &*(self.host_env as *const T)
    }
}

impl Drop for svm_env_t {
    fn drop(&mut self) {
        dbg!("dropping `svm_env_t`");
    }
}

impl From<*mut c_void> for &svm_env_t {
    fn from(env: *mut c_void) -> Self {
        unsafe { &*(env as *mut svm_env_t) }
    }
}

#[derive(Debug, Clone)]
pub struct ExternImport {
    pub name: String,

    pub namespace: String,

    pub params: Vec<WasmType>,

    pub returns: Vec<WasmType>,

    pub func_ptr: *const c_void,

    pub host_env: *const c_void,
}

impl ExternImport {
    pub fn wasmer_export(&self, store: &Store, ctx: &mut Context) -> (Export, *const svm_env_t) {
        unsafe {
            // This code is almost a clone of the code here:
            // https://github.com/wasmerio/wasmer/blob/7847acaae1e7a0eade13b65def1f3feeac95efd7/lib/c-api/src/wasm_c_api/externals/func.rs#L86

            let func_ty = self.wasmer_function_ty();

            let callback: wasm_func_callback_with_env_t = std::mem::transmute(self.func_ptr);

            let num_rets = func_ty.results().len();

            let inner_callback =
                move |env: &mut *mut c_void, args: &[Val]| -> Result<Vec<Val>, RuntimeError> {
                    let processed_args: wasm_val_vec_t = args
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<wasm_val_t>, _>>()
                        .expect("Argument conversion failed")
                        .into();

                    let zero = wasm_val_t::try_from(Val::I64(0)).unwrap();
                    let mut results: wasm_val_vec_t = vec![zero; num_rets].into();

                    let trap = callback(*env, &processed_args, &mut results);

                    if !trap.is_null() {
                        let trap: Box<wasm_trap_t> = Box::from_raw(trap);

                        // TODO: we want access to `trap.inner`
                        let err = RuntimeError::new("unexpected error");
                        return Err(err);
                    }

                    let processed_results = results
                        .into_slice()
                        .expect("Failed to convert `results` into a slice")
                        .into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<Val>, _>>()
                        .expect("Result conversion failed");

                    Ok(processed_results)
                };

            let inner_env = ctx as *mut Context as *const Context as *const c_void;
            let host_env = self.host_env;

            let func_env = svm_env_t {
                inner_env,
                host_env,
            };

            let func_env = svm_common::into_raw_mut(func_env);

            let func = Function::new_with_env(store, &func_ty, func_env, inner_callback);
            let export = func.to_export();

            let func_env = func_env as *const svm_env_t;

            (export, func_env)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    fn wasmer_function_ty(&self) -> FunctionType {
        let params = to_wasmer_types(&self.params);
        let returns = to_wasmer_types(&self.returns);

        FunctionType::new(params, returns)
    }
}

fn to_wasmer_types(types: &[WasmType]) -> Vec<Type> {
    types
        .iter()
        .map(|ty| match ty {
            WasmType::I32 => Type::I32,
            WasmType::I64 => Type::I64,
            _ => panic!("Only i32 and i64 are supported."),
        })
        .collect()
}
