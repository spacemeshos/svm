use std::convert::{TryFrom, TryInto};
use std::ffi::c_void;

use crate::Context;

use wasmer::{Export, Exportable, Function, FunctionType, RuntimeError, Store, Type, Val};

use svm_ffi::{svm_byte_array, svm_env_t, svm_func_callback_t, svm_trap_t, svm_wasm_types_t};
use svm_types::{WasmType, WasmValue};

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
            // The following code has been highly influenced by code here:
            // https://github.com/wasmerio/wasmer/blob/7847acaae1e7a0eade13b65def1f3feeac95efd7/lib/c-api/src/wasm_c_api/externals/func.rs#L86

            let callback: svm_func_callback_t = std::mem::transmute(self.func_ptr);
            let returns = self.returns.clone();

            let inner_callback =
                move |env: &mut *mut c_void, args: &[Val]| -> Result<Vec<Val>, RuntimeError> {
                    let args: Vec<WasmValue> = wasmer_vals_to_wasm_vals(args)?;
                    let args: svm_byte_array = args.into();

                    let results = alloc_results(*env);
                    let results = Box::into_raw(Box::new(results));

                    let trap = callback(*env, &args, results);

                    // manually releasing `args` internals
                    args.destroy();

                    if !trap.is_null() {
                        let trap: Box<svm_trap_t> = Box::from_raw(trap);

                        let err_msg: String = (*trap).into();
                        let err = RuntimeError::new(err_msg);

                        return Err(err);
                    }

                    let results = Box::from_raw(results);

                    match Vec::<WasmValue>::try_from(&*results) {
                        Ok(vals) => {
                            let wasmer_vals = wasm_vals_to_wasmer_vals(&vals);

                            // manually releasing `results` internals
                            results.destroy();

                            Ok(wasmer_vals)
                        }
                        Err(..) => Err(RuntimeError::new("Invalid WASM values")),
                    }
                };

            let func_ty = self.wasmer_function_ty();

            /// making the input `&mut Context` appear as `*const c_void`
            let inner_env = ctx as *mut Context as *const Context as *const c_void;
            let host_env = self.host_env;

            let (ptr, length, capacity) = Vec::into_raw_parts(self.returns.clone());

            let returns = svm_wasm_types_t {
                ptr: ptr as *const c_void,
                length,
                capacity,
            };

            /// The import used `env` (using Wasmer terminology) will be a struct of `svm_env_t`
            /// This `#[repr(C)]` struct will contain two pointers to two types of `env`:
            ///
            /// 1. SVM inner env - a pointer to the `Context`
            ///    Once SVM has finished executing a transaction its memory will be deallocated.
            ///
            /// 2. Host env - a pointer given as input by the so-called `Host`
            ///    The responsibility of release that memory is up to the caller (the `host`).
            let func_env = svm_env_t {
                inner_env,
                host_env,
                returns,
            };

            /// The heap-allocated `func_env` will be dellocated by later by `SVM` running runtime.
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

fn wasmer_vals_to_wasm_vals(wasmer_vals: &[Val]) -> Result<Vec<WasmValue>, RuntimeError> {
    let mut values = Vec::new();

    for val in wasmer_vals {
        let value = match val {
            Val::I32(v) => WasmValue::I32(*v as u32),
            Val::I64(v) => WasmValue::I64(*v as u64),
            _ => return Err(RuntimeError::new("Invalid argument type")),
        };

        values.push(value);
    }

    Ok(values)
}

fn wasm_vals_to_wasmer_vals(vals: &[WasmValue]) -> Vec<Val> {
    vals.iter()
        .map(|val| match val {
            WasmValue::I32(v) => Val::I32(*v as i32),
            WasmValue::I64(v) => Val::I64(*v as i64),
        })
        .collect()
}

unsafe fn alloc_results(env: *mut c_void) -> svm_byte_array {
    let env: &svm_env_t = env.into();
    let types = env.return_types();

    svm_ffi::alloc_wasm_values(types)
}
